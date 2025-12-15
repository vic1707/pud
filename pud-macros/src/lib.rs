#![doc = include_str!("../README.md")]
#![no_std]

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
	let derive_input: ::syn::DeriveInput = ::syn::parse(item)?;

	Ok(::quote::quote! {})
}
