use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Type, Data, DeriveInput, Fields, Ident, LitStr};

fn operand_quote(i: usize, name: Option<&Ident>, field_type: &Type) -> proc_macro2::TokenStream {
	let default_name = Ident::new(
		format!("op{}", i).as_str(),
		Span::call_site()
	);
	let field_name = name.unwrap_or(&default_name);

	if i > 0 {
		quote! {
			print!(", {}", stringify!(#field_name));
		}
	} else {
		quote! {
			print!(" {}", stringify!(#field_name));
		}
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
