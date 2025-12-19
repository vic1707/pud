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
