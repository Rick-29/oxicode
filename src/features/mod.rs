//! Feature-gated type implementations

#[cfg(feature = "alloc")]
mod impl_alloc;

#[cfg(feature = "std")]
mod impl_std;

// Atomic types are available in no_std environments
mod impl_atomic;

#[cfg(feature = "serde")]
pub mod serde;
