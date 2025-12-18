#![doc = include_str!("../README.md")]
#![no_std]
extern crate alloc;

mod arguments;
mod field;
mod field_group;
mod utils;
use crate::{
	arguments::{Argument, Arguments},
	field::Field,
	field_group::FieldGroups,
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
	let item: ::syn::ItemStruct = ::syn::parse(item)?;
	let item_copy = stripped_pud_field_attrs(item.clone());
	let ::syn::ItemStruct {
		fields,
		vis: original_vis,
		ident,
		generics,
		..
	} = item;

	let Arguments { rename, derives, vis } = Arguments::from(
		::syn::punctuated::Punctuated::<Argument, ::syn::Token![,]>::parse_terminated
			.parse(args)?,
	);

	let enum_name = rename.unwrap_or(::quote::format_ident!("{}Pud", ident));
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let fields_and_types: ::alloc::vec::Vec<_> = fields
		.into_iter()
		.enumerate()
		.map(Field::try_from)
		.collect::<::syn::Result<_>>()?;

	let variants = fields_and_types.iter().map(Field::to_variant);
	let match_arms = fields_and_types.iter().map(Field::match_arm);

	let groups = FieldGroups::from_iter(&fields_and_types);
	let groups_variants = groups.variants();
	let groups_arms = groups.match_arms();

    let vis = vis.unwrap_or(original_vis);
	Ok(::quote::quote! {
		#item_copy

		#[derive( #derives )]
		#vis enum #enum_name #impl_generics #where_clause {
			#( #variants ),*,
			#( #groups_variants ),*
		}

		#[automatically_derived]
		impl #impl_generics ::pud::Pud for #enum_name #ty_generics #where_clause {
			type Target = #ident #ty_generics;
			fn apply(self, target: &mut Self::Target) {
				match self {
					#( #match_arms ),*
					#( #groups_arms ),*
				}
			}
		}
	})
}

fn stripped_pud_field_attrs(mut item: ::syn::ItemStruct) -> ::syn::ItemStruct {
	for field in item.fields.iter_mut() {
		field.attrs.retain(|attr| !attr.path().is_ident("pud"));
	}
	item
}
