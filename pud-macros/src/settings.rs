use crate::utils::parse_parenthesized_list;
use ::{
	alloc::vec::Vec,
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
}

impl From<Punctuated<Argument, ::syn::Token![,]>> for Settings {
	fn from(punctuated_args: Punctuated<Argument, ::syn::Token![,]>) -> Self {
		let mut args = Self::default();
		for arg in punctuated_args {
			match arg {
				Argument::Rename(new_name) => args.rename = Some(new_name),
				Argument::Vis(vis) => args.vis = Some(vis),
				Argument::Attrs(attrs) => args.attrs.extend(attrs),
			}
		}
		args
	}
}

pub enum Argument {
	Rename(::syn::Ident),
	Vis(::syn::Visibility),
	Attrs(Punctuated<::syn::Meta, ::syn::Token![,]>),
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
			_ => return Err(::syn::Error::new_spanned(ident, "Unknown argument.")),
		};
		Ok(arg)
	}
}
