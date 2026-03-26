use ::alloc::{collections::btree_map::BTreeMap, vec::Vec};

use crate::field::Field;

#[derive(Default)]
pub struct FieldGroups<'a>(BTreeMap<::syn::Ident, GroupData<'a>>);

#[derive(Default)]
struct GroupData<'a> {
	fields: Vec<&'a Field>,
	attrs: Vec<::syn::Meta>,
}

impl FieldGroups<'_> {
	pub(crate) fn variants(&self) -> impl Iterator<Item = ::syn::Variant> {
		self.0.iter().map(|(group_name, group)| {
			let attrs = &group.attrs;
			let fields_types = group.fields.iter().map(|f| f.ty());

			::syn::parse_quote! {
				#( #[ #attrs ] )*
				#group_name ( #( #fields_types ),* )
			}
		})
	}

	pub(crate) fn match_arms(&self) -> impl Iterator<Item = ::syn::Arm> {
		self.0.iter().map(|(group_name, group)| {
			let fields_names = group.fields.iter().map(|f| f.v_name());
			let fields_assignments = group.fields.iter().map(|f| f.assignment());

			::syn::parse_quote! { Self::#group_name ( #( #fields_names ),* ) => { #( #fields_assignments );* } }
		})
	}
}

impl<'a> FieldGroups<'a> {
	pub(crate) fn try_from_fields_and_attrs<T: IntoIterator<Item = &'a Field>>(
		iter: T,
		mut group_attrs: BTreeMap<::syn::Ident, Vec<::syn::Meta>>,
	) -> ::syn::Result<Self> {
		let groups = iter.into_iter().fold(Self::default(), |mut acc, f| {
			for group in f.groups() {
				let attrs = group_attrs.remove(group).unwrap_or_default();
				let entry = acc.0.entry(group.clone()).or_default();
				entry.fields.push(f);
				if entry.attrs.is_empty() {
					entry.attrs = attrs;
				}
			}
			acc
		});

		if let Some((group, _)) = group_attrs.into_iter().next() {
			return Err(::syn::Error::new_spanned(
				group,
				"`group_attrs` references an unknown group.",
			));
		}

		Ok(groups)
	}
}
