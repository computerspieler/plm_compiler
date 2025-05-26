use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(InstructionLister)]
pub fn instruction_lister(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;
	let variants = match input.data {
		Data::Enum(ref data_enum) => &data_enum.variants,
		_ => {
			return syn::Error::new_spanned(
				name,
				"Parser can only be used with enums",
			)
			.to_compile_error()
			.into();
		}
	};

	let variant_matches = variants.iter().map(|variant| {
		let variant_name = &variant.ident;

		let nb_args = match &variant.fields {
            Fields::Named(fields) => fields.named.len(),
            Fields::Unnamed(fields) => fields.unnamed.len(),
            Fields::Unit => 0
        };

		quote! {
			print!("- {}", stringify!(#variant_name));
			for i in 0 .. #nb_args {
				if i > 0 {
					print!(",");
				}
				print!(" op{}", i);
			}
			println!("");
		}
	});

	let expanded = quote! {
		pub fn list_instructions() {
			println!("Here's the list of all the supported instructions:");
			#(#variant_matches)*
		}
	};

	TokenStream::from(expanded)
}
