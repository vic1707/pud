#![doc = include_str!("../README.md")]
// #![no_implicit_prelude] // TODO: enable?
#![no_std]

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
	fn apply(&mut self, pud: impl Pud<Target = Self>) {
		pud.apply(self);
	}

	/// Apply multiple modifications in sequence.
	fn apply_batch(&mut self, puds: impl Iterator<Item = impl Pud<Target = Self>>) {
		puds.for_each(|p| p.apply(self));
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
	fn try_into_pud(self) -> Result<Self::Pud, Self::Error>;
}

impl<T> TryIntoPud for T
where
	T: IntoPud,
{
	type Pud = T::Pud;
	type Error = ::core::convert::Infallible;

	fn try_into_pud(self) -> Result<Self::Pud, Self::Error> {
		Ok(self.into_pud())
	}
}
