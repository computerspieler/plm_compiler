use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Type, Data, DeriveInput, Fields, Ident, LitStr};

fn operand_quote(i: usize, name: Option<&Ident>, _field_type: &Type) -> proc_macro2::TokenStream {
	let default_name = Ident::new(
		format!("op{}", i).as_str(),
		Span::call_site()
	);
	let field_name = name.unwrap_or(&default_name);

	if i > 0 {
		quote! { print!(", {}", stringify!(#field_name)); }
	} else {
		quote! { print!(" {}", stringify!(#field_name));  }
	}
}

#[proc_macro_derive(InstructionLister, attributes(help))]
pub fn instruction_lister(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	
	let name = input.ident;
	let variants = match input.data {
		Data::Enum(ref data_enum) => &data_enum.variants,
		_ => {
			return syn::Error::new_spanned(
				name,
				"InstructionLister can only be used with enums",
			)
			.to_compile_error()
			.into();
		}
	};

	let variant_matches = variants.iter().map(|variant| {
		let variant_name = &variant.ident;
		let mut out = quote! {
			print!("- {}", stringify!(#variant_name));
		};
		
		match &variant.fields {
            Fields::Named(fields) => {
				for (i, field) in fields.named.iter().enumerate() {
					out.extend(operand_quote(i, field.ident.as_ref(), &field.ty));
				}
			},
            Fields::Unnamed(fields) => {
				for (i, field) in fields.unnamed.iter().enumerate() {
					out.extend(operand_quote(i, None, &field.ty));
				}
			},
            Fields::Unit => {}
        };


		let mut help_text: LitStr = LitStr::new("", Span::call_site());
		for attr in variant.attrs.iter() {
			if attr.path().is_ident("help") {
				help_text = attr.parse_args().unwrap();
			}
		}
		
		
		out.extend(quote! {
			if #help_text.len() > 0 {
				print!(": {}", #help_text)
			}

			println!("");
		});
		out.extend(quote! {});
		out
	});

	let expanded = quote! {
		pub fn list_instructions() {
			println!("Here's the list of all the supported instructions:");
			#(#variant_matches)*
		}
	};

	TokenStream::from(expanded)
}

#[proc_macro_derive(InstructionParser)]
pub fn instruction_parser(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	
	let name = input.ident;
	let variants = match input.data {
		Data::Enum(ref data_enum) => &data_enum.variants,
		_ => {
			return syn::Error::new_spanned(
				name,
				"InstructionParser can only be used with enums",
			)
			.to_compile_error()
			.into();
		}
	};

	let instructions_parser = variants.iter().map(|variant| {
		let variant_name = &variant.ident;

		let fields = match &variant.fields {
            Fields::Unit => None,
            Fields::Named(fields) => Some(&fields.named),
            Fields::Unnamed(fields) => Some(&fields.unnamed),
        };

		if let Some(fields) = fields {
			//TODO
			let _fields = fields.iter().map(|_field| {
				quote! {

				}
			});
			quote! {
				tag(Token::Keyword(stringify!(#variant_name)))
			}
		} else {
			quote! {
				tag(Token::Keyword(stringify!(#variant_name)))
			}
		}
	});

	let expanded = quote! {
		impl #name {
			pub fn parser<'a>(
				input: &'a str
			) -> nom::IResult<&'a str, &'a str> {
				use nom::{
					Parser,
					bytes::complete::tag,
					branch::alt,
					sequence::tuple
				};
				use utils::token::Token;
				
				alt([
					#(#instructions_parser),*
				]).parse(input)
			}
		}
	};

	TokenStream::from(expanded)
}


#[proc_macro_derive(IdentifierParser, attributes(rename))]
pub fn identifier_parser(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	
	let name = input.ident;
	let variants = match input.data {
		Data::Enum(ref data_enum) => &data_enum.variants,
		_ => {
			return syn::Error::new_spanned(
				name,
				"IdentifierParser can only be used with enums",
			)
			.to_compile_error()
			.into();
		}
	};

	let nb_invalid_variants = variants.iter().filter(|variant| -> bool {
		match &variant.fields {
		Fields::Unit => false,
		_ => true
		}
	}).count();

	if nb_invalid_variants > 0 {
		return syn::Error::new_spanned(
			name,
			"IdentifierParser doesn't support variants with fields",
		)
			.to_compile_error()
			.into();
	}

	let instructions_parser = variants.iter().map(|variant| {
		let variant_name = &variant.ident;
		
		let mut variant_text_name: Option<LitStr> = None;
		for attr in variant.attrs.iter() {
			if attr.path().is_ident("rename") {
				variant_text_name = Some(attr.parse_args().unwrap());
			}
		}

		if let Some(txt) = variant_text_name {
			quote! {
				value(#name::#variant_name, tag_no_case(#txt))
			}
		} else {
			quote! {
				value(#name::#variant_name, tag_no_case(stringify!(#variant_name)))
			}
		}
	});

	let expanded = quote! {
		impl<'a> nom::Parser<&'a str> for #name {
			type Output = #name;
		    type Error = nom::error::Error<&'a str>;

			fn process<OM: nom::OutputMode>(
				&mut self,
				input: &'a str
			) -> nom::PResult<OM, &'a str, Self::Output, Self::Error> {
				use nom::{
					Parser,
					bytes::complete::tag_no_case,
					branch::alt,
					combinator::value,
					sequence::tuple
				};
				
				alt([
					#(#instructions_parser),*
				]).process::<OM>(input)
			}
		}
	};

	TokenStream::from(expanded)
}
