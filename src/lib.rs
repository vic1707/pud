#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![expect(
	clippy::pub_use,
	clippy::blanket_clippy_restriction_lints,
	clippy::arbitrary_source_item_ordering,
	clippy::missing_trait_methods,
	clippy::implicit_return,
	reason = "re-exports"
)]
#![allow(clippy::missing_docs_in_private_items, reason = "expect doesn't work")]
#![no_std]

use ::core::convert::Infallible;

pub use ::pud_macros::pud;

/// A single atomic modification that can be applied to a target type.
pub trait Pud {
	/// The type this modification can be applied to.
	type Target;

	/// Apply this modification to `target`.
	fn apply(self, target: &mut Self::Target);
}

/// A type that can have [`Pud`]s applied to it.
///
/// Automatically implemented for all sized types.
/// Provides ability for applying one or more modifications from the target object.
pub trait Pudded: Sized {
	/// Apply a single modification.
	#[inline]
	fn apply(&mut self, pud: impl Pud<Target = Self>) {
		pud.apply(self);
	}

	/// Apply multiple modifications in sequence.
	#[inline]
	fn apply_batch(&mut self, puds: impl Iterator<Item = impl Pud<Target = Self>>) {
		puds.for_each(|pud| pud.apply(self));
	}
}

impl<T: Sized> Pudded for T {}

/// Conversion into a [`Pud`] targeting a specific type.
///
/// This trait is intended for producers of modifications—such as UI events,
/// commands, or configuration updates that wish to generate a [`Pud`]
/// without being coupled to a concrete Pud enum type.
pub trait IntoPud {
	/// The concrete [`Pud`] type produced by this conversion.
	type Pud: Pud;

	/// Convert `self` into a Pud.
	fn into_pud(self) -> Self::Pud;
}

/// Fallible version of into [`IntoPud`].
///
/// A blanket implementation exists for any type implementing [`IntoPud`],
/// using [`::core::convert::Infallible`] as the error type.
pub trait TryIntoPud {
	/// The concrete [`Pud`] type produced by this conversion.
	type Pud: Pud;

	/// The error returned if conversion fails.
	type Error;

	/// Attempt to convert `self` into a Pud.
	///
	/// # Errors
	///
	/// Will return `Error` if conversion fails.
	fn try_into_pud(self) -> Result<Self::Pud, Self::Error>;
}

impl<T> TryIntoPud for T
where
	T: IntoPud,
{
	type Pud = T::Pud;
	type Error = Infallible;

	#[inline]
	fn try_into_pud(self) -> Result<Self::Pud, Self::Error> {
		Ok(self.into_pud())
	}
}
