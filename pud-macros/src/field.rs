use crate::syn_ident_to_pascal_case;

pub(crate) struct Field {
	ident: ::syn::Ident,
	ty: ::syn::Type,
}

impl TryFrom<::syn::Field> for Field {
	type Error = ::syn::Error;
	fn try_from(field: ::syn::Field) -> ::syn::Result<Self> {
		let Some(ident) = field.ident else {
			return Err(::syn::Error::new_spanned(field, "Expected a named field."));
		};
		let ::syn::Field { ty, .. } = field;

		Ok(Self { ident, ty })
	}
}

impl Field {
	pub(crate) fn variant_ident(&self) -> ::syn::Ident {
		syn_ident_to_pascal_case(&self.ident)
	}

	pub(crate) fn to_variant(&self) -> ::syn::Variant {
		let Self { ty, .. } = self;
		let variant_ident = self.variant_ident();

		::syn::parse_quote! { #variant_ident ( #ty ) }
	}

	pub(crate) fn match_arm_update(&self) -> ::syn::Arm {
		let Self { ident, .. } = self;
		let variant_ident = self.variant_ident();

		::syn::parse_quote! { Self::#variant_ident ( #ident ) => {target. #ident = #ident;} }
	}
}
