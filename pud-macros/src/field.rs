use crate::{syn_ident_to_pascal_case, utils::parse_parentheses};
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
				"Unnamed fields must have a `#[pud(rename = Name)]`.",
			));
		}

		if args.map.is_some() && args.flatten.is_some() {
			return Err(::syn::Error::new_spanned(
				&field,
				"Use either `map` or `flatten`, not both.",
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

		if let Some(ref from) = self.settings.flatten {
			::syn::parse_quote! { #variant_ident ( #from ) }
		} else if let Some((ref from, _)) = self.settings.map {
			::syn::parse_quote! { #variant_ident ( #from ) }
		} else {
			::syn::parse_quote! { #variant_ident ( #ty ) }
		}
	}

	pub(crate) fn assignment(&self) -> ::syn::Expr {
		let Self {
			member,
			v_name: name_as_var,
			..
		} = self;

		if self.settings.flatten.is_some() {
			::syn::Expr::MethodCall(
				::syn::parse_quote! { #name_as_var.apply(&mut target. #member ) },
			)
		} else if let Some((_, ref map)) = self.settings.map {
			::syn::Expr::Assign(::syn::parse_quote! { target. #member = ( #map )( #name_as_var ) })
		} else {
			::syn::Expr::Assign(::syn::parse_quote! { target. #member = #name_as_var })
		}
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
	flatten: Option<::syn::Path>,
	groups: ::alloc::vec::Vec<::syn::Ident>,
	map: Option<(::syn::Path, crate::utils::CustomFunction)>,
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
						Argument::Flatten(from) => args.flatten = Some(from),
						Argument::Group(group) => args.groups.push(group),
						Argument::Map(map) => args.map = Some(map),
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
	Flatten(::syn::Path),
	Map((::syn::Path, crate::utils::CustomFunction)),
}

impl ::syn::parse::Parse for Argument {
	fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
		use ::alloc::string::ToString as _;

		let ident = input.parse::<::syn::Ident>()?;
		let arg = match ident.to_string().as_str() {
			"rename" => {
				input.parse::<::syn::Token![=]>()?;
				let new_name = input.parse()?;
				Self::Rename(new_name)
			},
			"group" => {
				input.parse::<::syn::Token![=]>()?;
				let group = input.parse()?;
				Self::Group(group)
			},
			"flatten" => {
				input.parse::<::syn::Token![=]>()?;
				let from = input.parse()?;
				Self::Flatten(from)
			},
			"map" => {
				let inner = parse_parentheses(input)?;
				let ty = inner.parse()?;
				let _ = inner.parse::<::syn::Token![>>=]>()?;
				let func = inner.parse()?;
				Self::Map((ty, func))
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
