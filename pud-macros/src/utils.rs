pub(crate) fn syn_ident_to_pascal_case(ident: &::syn::Ident) -> ::syn::Ident {
	use ::alloc::string::ToString as _;

	::quote::format_ident!("{}", ::convert_case::ccase!(pascal, ident.to_string()))
}

pub(crate) fn parse_parenthesized_list<T: ::syn::parse::Parse>(
	input: ::syn::parse::ParseStream,
) -> ::syn::Result<::syn::punctuated::Punctuated<T, ::syn::Token![,]>> {
	let content: ::syn::parse::ParseBuffer<'_>;
	syn::parenthesized!(content in input);
	content.parse_terminated(T::parse, ::syn::Token![,])
}

pub(crate) fn parse_parentheses(
	input: ::syn::parse::ParseStream<'_>,
) -> ::syn::Result<::syn::parse::ParseBuffer<'_>> {
	let content: ::syn::parse::ParseBuffer<'_>;
	syn::parenthesized!(content in input);
	Ok(content)
}

/// Either
/// custom = `path::to::fn`
/// custom = |input: Type| { ...; return .. }
pub(crate) enum CustomFunction {
	/// Path to the function to run of signature
	/// Fn(mut Inner) -> Inner
	/// mut being optional
	Path(syn::Path),
	/// Closure of type
	/// Fn(mut Inner) -> Inner
	/// mut being optional
	Closure(syn::ExprClosure),
}

impl ::syn::parse::Parse for CustomFunction {
	fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
		let closure = if let Ok(path) = input.parse() {
			Self::Path(path)
		} else if let Ok(closure) = input.parse() {
			Self::Closure(closure)
		} else {
			return Err(syn::Error::new(
				input.span(),
				"Invalid `map` argument input.",
			));
		};

		if !input.peek(syn::Token![,]) && !input.is_empty() {
			return Err(input.error("Unexpected token(s)."));
		}

		Ok(closure)
	}
}

impl ::quote::ToTokens for CustomFunction {
	fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
		match *self {
			Self::Path(ref path) => path.to_tokens(tokens),
			Self::Closure(ref closure) => closure.to_tokens(tokens),
		}
	}
}
