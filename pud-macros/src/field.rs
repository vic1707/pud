// TODO: `skip` doesn't handle missing generics
use crate::syn_ident_to_pascal_case;
use ::{
	quote::ToTokens as _,
	syn::parse::{Parse as _, Parser as _},
};

pub(crate) struct Field {
	args: Arguments,
	pub(crate) ident: ::syn::Ident,
	pub(crate) ty: ::syn::Type,
}

impl TryFrom<::syn::Field> for Field {
	type Error = ::syn::Error;
	fn try_from(field: ::syn::Field) -> ::syn::Result<Self> {
		let Some(ident) = field.ident else {
			return Err(::syn::Error::new_spanned(field, "Expected a named field."));
		};
		let ::syn::Field { ty, attrs, .. } = field;
		let args = Arguments::try_from(attrs.as_slice())?;

		Ok(Self { ident, ty, args })
	}
}

impl Field {
	pub(crate) fn skip(&self) -> bool {
		self.args.skip
	}

	pub(crate) fn groups(&self) -> impl Iterator<Item = &::syn::Ident> {
		self.args.groups.iter()
	}

	fn variant_ident(&self) -> ::syn::Ident {
		self.args
			.rename
			.clone()
			.unwrap_or_else(|| syn_ident_to_pascal_case(&self.ident))
	}

	pub(crate) fn to_variant(&self) -> ::syn::Variant {
		let Self { ty, .. } = self;
		let variant_ident = self.variant_ident();

		::syn::parse_quote! { #variant_ident ( #ty ) }
	}

	pub(crate) fn assignment(&self) -> ::syn::ExprAssign {
		let Self { ident, .. } = self;
		::syn::parse_quote! { target. #ident = #ident }
	}

	pub(crate) fn match_arm(&self) -> ::syn::Arm {
		let Self { ident, .. } = self;
		let variant_ident = self.variant_ident();
		let assignment = self.assignment();

		::syn::parse_quote! { Self::#variant_ident ( #ident ) => { #assignment; } }
	}
}

#[derive(Default)]
pub(crate) struct Arguments {
	rename: Option<::syn::Ident>,
	skip: bool,
	groups: ::alloc::vec::Vec<::syn::Ident>,
}

impl TryFrom<&[::syn::Attribute]> for Arguments {
	type Error = ::syn::Error;

	fn try_from(attrs: &[::syn::Attribute]) -> ::syn::Result<Self> {
		let mut args = Self::default();

		for attr in attrs {
			if attr.path().is_ident("pud") {
				for arg in parse_pud_attr.parse2(attr.meta.to_token_stream())? {
					match arg {
						Argument::Rename(new_name) => args.rename = Some(new_name),
						Argument::Skip => args.skip = true,
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
	Skip,
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
			"skip" => Self::Skip,
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
