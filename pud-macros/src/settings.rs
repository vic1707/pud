use crate::utils::parse_parenthesized_list;
use ::{
	alloc::{collections::BTreeMap, vec::Vec},
	syn::{
		parse::{Parse, ParseStream},
		punctuated::Punctuated,
	},
};

#[derive(Default)]
pub struct Settings {
	pub rename: Option<::syn::Ident>,
	pub vis: Option<::syn::Visibility>,
	pub attrs: Vec<::syn::Meta>,
	pub phantom_attrs: Vec<::syn::Meta>,
	pub group_attrs: BTreeMap<::syn::Ident, Vec<::syn::Meta>>,
	pub whole: Option<::syn::Ident>,
}

impl TryFrom<Punctuated<Argument, ::syn::Token![,]>> for Settings {
	type Error = ::syn::Error;

	fn try_from(punctuated_args: Punctuated<Argument, ::syn::Token![,]>) -> ::syn::Result<Self> {
		let mut args = Self::default();
		for arg in punctuated_args {
			match arg {
				Argument::Rename(new_name) => args.rename = Some(new_name),
				Argument::Vis(vis) => args.vis = Some(vis),
				Argument::Attrs(attrs) => args.attrs.extend(attrs),
				Argument::PhantomAttrs(attrs) => args.phantom_attrs.extend(attrs),
				Argument::GroupAttrs(entries) => {
					for GroupAttrsEntry { group, attrs } in entries {
						if args
							.group_attrs
							.insert(group.clone(), attrs.into_iter().collect())
							.is_some()
						{
							return Err(::syn::Error::new_spanned(
								group,
								"Duplicate `group_attrs` entry.",
							));
						}
					}
				},
				Argument::Whole(name) => args.whole = Some(name),
			}
		}
		Ok(args)
	}
}

pub enum Argument {
	Rename(::syn::Ident),
	Vis(::syn::Visibility),
	Attrs(Punctuated<::syn::Meta, ::syn::Token![,]>),
	PhantomAttrs(Punctuated<::syn::Meta, ::syn::Token![,]>),
	GroupAttrs(Punctuated<GroupAttrsEntry, ::syn::Token![,]>),
	Whole(::syn::Ident),
}

impl Parse for Argument {
	fn parse(input: ParseStream) -> ::syn::Result<Self> {
		use ::alloc::string::ToString as _;

		let ident = input.parse::<::syn::Ident>()?;
		let arg = match ident.to_string().as_str() {
			"rename" => {
				input.parse::<::syn::Token![=]>()?;
				let new_name = input.parse()?;
				Self::Rename(new_name)
			},
			"vis" => {
				input.parse::<::syn::Token![=]>()?;
				let vis = input.parse()?;
				Self::Vis(vis)
			},
			"attrs" => {
				let attrs = parse_parenthesized_list(input)?;
				Self::Attrs(attrs)
			},
			"phantom_attrs" => {
				let attrs = parse_parenthesized_list(input)?;
				Self::PhantomAttrs(attrs)
			},
			"group_attrs" => {
				let attrs = parse_parenthesized_list(input)?;
				Self::GroupAttrs(attrs)
			},
			"whole" => {
				let name = if input.peek(syn::Token![=]) {
					input.parse::<::syn::Token![=]>()?;
					input.parse()?
				} else {
					::syn::Ident::new("Whole", ident.span())
				};
				Self::Whole(name)
			},
			_ => return Err(::syn::Error::new_spanned(ident, "Unknown argument.")),
		};
		Ok(arg)
	}
}

pub struct GroupAttrsEntry {
	pub group: ::syn::Ident,
	pub attrs: Punctuated<::syn::Meta, ::syn::Token![,]>,
}

impl Parse for GroupAttrsEntry {
	fn parse(input: ParseStream) -> ::syn::Result<Self> {
		let group = input.parse()?;
		let attrs = parse_parenthesized_list(input)?;

		Ok(Self { group, attrs })
	}
}
