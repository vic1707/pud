#[derive(Default)]
pub(crate) struct Arguments {
	pub rename: Option<::syn::Ident>,
}

impl From<::syn::punctuated::Punctuated<Argument, ::syn::Token![,]>> for Arguments {
	fn from(punctuated_args: ::syn::punctuated::Punctuated<Argument, ::syn::Token![,]>) -> Self {
		let mut args = Self::default();
		for arg in punctuated_args {
			match arg {
				Argument::Rename(new_name) => args.rename = Some(new_name),
			}
		}
		args
	}
}

pub(crate) enum Argument {
	Rename(::syn::Ident),
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
			_ => return Err(::syn::Error::new_spanned(ident, "Unknown argument.")),
		};
		Ok(arg)
	}
}
