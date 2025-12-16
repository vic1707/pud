#[derive(Default)]
pub(crate) struct Arguments;

impl From<::syn::punctuated::Punctuated<Argument, ::syn::Token![,]>> for Arguments {
	fn from(punctuated_args: ::syn::punctuated::Punctuated<Argument, ::syn::Token![,]>) -> Self {
		let mut args = Self::default();
		for arg in punctuated_args {}
		args
	}
}

pub(crate) enum Argument {}

impl ::syn::parse::Parse for Argument {
	fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
		use ::alloc::string::ToString as _;

		let ident = input.parse::<::syn::Ident>()?;
		let arg = match ident.to_string().as_str() {
			_ => return Err(::syn::Error::new_spanned(ident, "Unknown argument.")),
		};
		Ok(arg)
	}
}
