pub(crate) fn syn_ident_to_pascal_case(ident: &::syn::Ident) -> ::syn::Ident {
    use ::alloc::string::ToString as _;

	::quote::format_ident!("{}", ::convert_case::ccase!(pascal, ident.to_string()))
}
