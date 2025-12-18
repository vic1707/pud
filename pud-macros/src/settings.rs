use crate::utils::parse_parenthesized;

#[derive(Default)]
pub(crate) struct Settings {
	pub rename: Option<::syn::Ident>,
	pub derives: Option<::syn::punctuated::Punctuated<::syn::Path, ::syn::Token![,]>>,
	pub vis: Option<::syn::Visibility>,
	pub attrs: ::alloc::vec::Vec<::syn::Meta>,
}

impl From<::syn::punctuated::Punctuated<Argument, ::syn::Token![,]>> for Settings {
	fn from(punctuated_args: ::syn::punctuated::Punctuated<Argument, ::syn::Token![,]>) -> Self {
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

pub(crate) enum Argument {
	Rename(::syn::Ident),
	Vis(::syn::Visibility),
	Attrs(::syn::punctuated::Punctuated<::syn::Meta, ::syn::Token![,]>),
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
			"vis" => {
				input.parse::<::syn::Token![=]>()?;
				let vis = input.parse()?;
				Self::Vis(vis)
			},
			"attrs" => {
				let attrs = parse_parenthesized(input)?;
				Self::Attrs(attrs)
			},
			_ => return Err(::syn::Error::new_spanned(ident, "Unknown argument.")),
		};
		Ok(arg)
	}
}
