#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
#![allow(clippy::missing_docs_in_private_items, reason = "expect doesn't work")]
#![expect(
	clippy::blanket_clippy_restriction_lints,
	clippy::implicit_return,
	clippy::arbitrary_source_item_ordering,
	clippy::min_ident_chars,
	clippy::missing_trait_methods,
	clippy::question_mark_used,
	clippy::single_call_fn,
	clippy::single_char_lifetime_names,
	clippy::ref_patterns,
	reason = "_"
)]

extern crate alloc;

mod field;
mod field_group;
mod settings;
mod utils;
use crate::{
	field::Field,
	field_group::FieldGroups,
	settings::{Argument, Settings},
	utils::syn_ident_to_pascal_case,
};
use ::{
	alloc::vec::Vec,
	syn::{parse::Parser as _, punctuated::Punctuated},
};

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
	let input_struct: ::syn::ItemStruct = ::syn::parse(item)?;
	let item_copy = stripped_pud_field_attrs(input_struct.clone());
	let ::syn::ItemStruct {
		fields,
		vis: original_vis,
		ident,
		generics,
		..
	} = input_struct;

	let Settings {
		rename,
		vis,
		attrs: transparent_attrs,
	} = Settings::from(Punctuated::<Argument, ::syn::Token![,]>::parse_terminated.parse(args)?);

	let enum_name = rename.unwrap_or_else(|| ::quote::format_ident!("{}Pud", ident));
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let fields_and_types: Vec<_> = fields
		.into_iter()
		.enumerate()
		.map(Field::try_from)
		.collect::<::syn::Result<_>>()?;

	let variants = fields_and_types.iter().map(Field::to_variant);
	let match_arms = fields_and_types.iter().map(Field::match_arm);

	let groups = FieldGroups::from_iter(&fields_and_types);
	let groups_variants = groups.variants();
	let groups_arms = groups.match_arms();

	let pud_vis = vis.unwrap_or(original_vis);
	let has_generics = !generics.params.is_empty();
	let phantom_variant = has_generics.then(|| {
		let generic_idents = generics.params.iter().filter_map(|p| match *p {
			syn::GenericParam::Type(ref ty) => Some(&ty.ident),
			syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => None,
		});
		::quote::quote! { #[doc(hidden)] __(::core::marker::PhantomData<( #( #generic_idents,)* )>), }
	});
	let phantom_arm = has_generics.then_some(::quote::quote! { Self::__(_) => {}, });
	Ok(::quote::quote! {
		#item_copy

		#( #[ #transparent_attrs ] )*
		#[allow(clippy::exhaustive_enums)]
		#pud_vis enum #enum_name #impl_generics #where_clause {
			#( #variants ),*,
			#( #groups_variants ),*
			#phantom_variant
		}

		#[automatically_derived]
		impl #impl_generics ::pud::Pud for #enum_name #ty_generics #where_clause {
			type Target = #ident #ty_generics;
			fn apply(self, target: &mut Self::Target) {
				match self {
					#( #match_arms ),*
					#( #groups_arms ),*
					#phantom_arm
				}
			}
		}
	})
}

fn stripped_pud_field_attrs(mut item: ::syn::ItemStruct) -> ::syn::ItemStruct {
	for field in &mut item.fields {
		field.attrs.retain(|attr| !attr.path().is_ident("pud"));
	}
	item
}
