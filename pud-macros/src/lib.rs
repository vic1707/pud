#![doc = include_str!("../README.md")]
#![no_std]
extern crate alloc;
mod utils;
use crate::utils::syn_ident_to_pascal_case;

#[proc_macro_attribute]
pub fn pud(
	args: ::proc_macro::TokenStream,
	item: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
	expand(args, item)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

fn expand(
	args: ::proc_macro::TokenStream,
	item: ::proc_macro::TokenStream,
) -> ::syn::Result<::proc_macro2::TokenStream> {
	let original_declaration: ::proc_macro2::TokenStream = item.clone().into();
	let ::syn::ItemStruct {
		fields,
		vis,
		ident,
		generics,
		..
	} = ::syn::parse(item)?;

	let enum_name = ::quote::format_ident!("{}Pud", ident);
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let variants = fields
		.into_iter()
		.map(|::syn::Field { ident, ty, .. }| -> syn::Variant {
			let ident = ident
				.map(|i| syn_ident_to_pascal_case(&i))
				.expect("Expected named field");

			::syn::parse_quote! { #ident ( #ty ) }
		});

	Ok(::quote::quote! {
		#original_declaration

		#vis enum #enum_name #impl_generics #where_clause {
			#( #variants ),*
		}
	})
}
