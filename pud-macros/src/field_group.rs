use crate::field::Field;

#[derive(Default)]
pub(crate) struct FieldGroups<'a>(
	::alloc::collections::BTreeMap<&'a ::syn::Ident, ::alloc::vec::Vec<&'a Field>>,
);

impl FieldGroups<'_> {
	pub(crate) fn variants(&self) -> impl Iterator<Item = ::syn::Variant> {
		self.0.iter().map(|(group_name, fields)| {
			let fields_types = fields.iter().map(|f| &f.ty);

			::syn::parse_quote! { #group_name ( #( #fields_types ),* ) }
		})
	}

	pub(crate) fn match_arms(&self) -> impl Iterator<Item = ::syn::Arm> {
		self.0.iter().map(|(group_name, fields)| {
			let fields_names = fields.iter().map(|f| &f.ident);
			let fields_assignments = fields.iter().map(|f| f.assignment());

			::syn::parse_quote! { Self::#group_name ( #( #fields_names ),* ) => { #( #fields_assignments );* } }
		})
	}
}

impl<'a> FromIterator<&'a Field> for FieldGroups<'a> {
	fn from_iter<T: IntoIterator<Item = &'a Field>>(iter: T) -> Self {
		iter.into_iter().fold(Self::default(), |mut acc, f| {
			for group in f.groups() {
				acc.0.entry(group).or_default().push(f)
			}
			acc
		})
	}
}
