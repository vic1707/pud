#![doc = include_str!("../README.md")]
#![no_std]
extern crate alloc;

mod arguments;
mod field;
mod utils;
use crate::{
	arguments::{Argument, Arguments},
	field::Field,
	utils::syn_ident_to_pascal_case,
};
use ::syn::parse::Parser as _;

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

	let args = Arguments::from(
		::syn::punctuated::Punctuated::<Argument, ::syn::Token![,]>::parse_terminated
			.parse(args)?,
	);

	let enum_name = args
		.rename
		.unwrap_or(::quote::format_ident!("{}Pud", ident));
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let fields_and_types = fields
		.into_iter()
		.map(Field::try_from)
		.collect::<::syn::Result<::alloc::vec::Vec<_>>>()?;

	let variants = fields_and_types.iter().map(Field::to_variant);
	let match_arms = fields_and_types.iter().map(Field::match_arm_update);

	Ok(::quote::quote! {
		#original_declaration

		#vis enum #enum_name #impl_generics #where_clause {
			#( #variants ),*
		}

		#[automatically_derived]
		impl #impl_generics ::pud::Pud for #enum_name #ty_generics #where_clause {
			type Target = #ident #ty_generics;
			fn apply(self, target: &mut Self::Target) {
				match self {
					#( #match_arms ),*
				}
			}
		}
	})
}
