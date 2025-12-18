use crate::syn_ident_to_pascal_case;
use ::{
	quote::ToTokens as _,
	syn::parse::{Parse as _, Parser as _},
};

pub(crate) struct Field {
	settings: Settings,
	member: ::syn::Member,
	pub(crate) ty: ::syn::Type,
	pub(crate) v_name: ::syn::Ident,
}

impl TryFrom<(usize, ::syn::Field)> for Field {
	type Error = ::syn::Error;

	fn try_from((idx, field): (usize, ::syn::Field)) -> ::syn::Result<Self> {
		let args = Settings::try_from(field.attrs.as_slice())?;
		let member = field.ident.clone().map_or(idx.into(), Into::into);

		if matches!(member, ::syn::Member::Unnamed(_)) && args.rename.is_none() {
			return Err(::syn::Error::new_spanned(
				&field,
				"Unnamed fields must have a `#[pud(rename = Name)]`",
			));
		}
		Ok(Self {
			settings: args,
			member,
			ty: field.ty,
			v_name: ::quote::format_ident!("_{idx}"),
		})
	}
}

impl Field {
	pub(crate) fn groups(&self) -> impl Iterator<Item = &::syn::Ident> {
		self.settings.groups.iter()
	}

	pub(crate) fn variant_ident(&self) -> ::syn::Ident {
		self.settings.rename.clone().unwrap_or_else(|| {
			let ::syn::Member::Named(ref ident) = self.member else {
				unreachable!("Checked in TryFrom");
			};
			syn_ident_to_pascal_case(ident)
		})
	}

	pub(crate) fn to_variant(&self) -> ::syn::Variant {
		let Self { ty, .. } = self;
		let variant_ident = self.variant_ident();

		::syn::parse_quote! { #variant_ident ( #ty ) }
	}

	pub(crate) fn assignment(&self) -> ::syn::ExprAssign {
		let Self {
			member,
			v_name: name_as_var,
			..
		} = self;

		::syn::parse_quote! { target. #member = #name_as_var }
	}

	pub(crate) fn match_arm(&self) -> ::syn::Arm {
		let name_as_var = &self.v_name;
		let variant_ident = self.variant_ident();
		let assignment = self.assignment();

		::syn::parse_quote! { Self::#variant_ident ( #name_as_var ) => { #assignment; } }
	}
}

#[derive(Default)]
pub(crate) struct Settings {
	rename: Option<::syn::Ident>,
	groups: ::alloc::vec::Vec<::syn::Ident>,
}

impl TryFrom<&[::syn::Attribute]> for Settings {
	type Error = ::syn::Error;

	fn try_from(attrs: &[::syn::Attribute]) -> ::syn::Result<Self> {
		let mut args = Self::default();

		for attr in attrs {
			if attr.path().is_ident("pud") {
				for arg in parse_pud_attr.parse2(attr.meta.to_token_stream())? {
					match arg {
						Argument::Rename(new_name) => args.rename = Some(new_name),
						Argument::Group(group) => args.groups.push(group),
					}
				}
			}
		}

		Ok(args)
	}
}

pub(crate) enum Argument {
	Rename(::syn::Ident),
	Group(::syn::Ident),
}

impl ::syn::parse::Parse for Argument {
	fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
		use ::alloc::string::ToString as _;

		let ident = input.parse::<::syn::Ident>()?;
		let arg = match ident.to_string().as_str() {
			"rename" => {
				input.parse::<::syn::Token![=]>()?;
				let new_name = input.parse::<::syn::Ident>()?;
				Self::Rename(new_name)
			},
			"group" => {
				input.parse::<::syn::Token![=]>()?;
				let group = input.parse::<::syn::Ident>()?;
				Self::Group(group)
			},
			_ => return Err(::syn::Error::new_spanned(ident, "Unknown argument.")),
		};
		Ok(arg)
	}
}

mod kw {
	::syn::custom_keyword!(pud);
}

fn parse_pud_attr(
	attr_ts: ::syn::parse::ParseStream,
) -> syn::Result<::syn::punctuated::Punctuated<Argument, syn::Token![,]>> {
	attr_ts.parse::<kw::pud>()?;
	let content;
	syn::parenthesized!(content in attr_ts);
	content.parse_terminated(Argument::parse, syn::Token![,])
}
