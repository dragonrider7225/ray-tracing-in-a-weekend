//! The non-interactive layer.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

/// An RGB color. The intensity of each component is in the range `[0.0, 1.0]`.
pub mod color;
pub use color::Color;

/// A 3D vector.
pub mod vec3;
pub use vec3::Vec3;
pub use vec3::Vec3 as Point3;
