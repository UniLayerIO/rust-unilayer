//! Rust-Bitcoin IO Library
//!
//! Because the core `std::io` module is not yet exposed in `no-std` Rust, building `no-std`
//! applications which require reading and writing objects via standard traits is not generally
//! possible. While there is ongoing work to improve this situation, this module is not likely to
//! be available for applications with broad rustc version support for some time.
//!
//! Thus, this library exists to export a minmal version of `std::io`'s traits which `no-std`
//! applications may need. With the `std` feature, these traits are also implemented for the
//! `std::io` traits, allowing standard objects to be used wherever the traits from this crate are
//! required.
//!
//! This traits are not one-for-one drop-ins, but are as close as possible while still implementing
//! `std::io`'s traits without unnecessary complexity.

// Experimental features we need.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), not(feature = "core2")))]
compile_error!("At least one of std or core2 must be enabled");

#[cfg(feature = "std")]
pub use std::io;
#[cfg(feature = "std")]
pub use std::error;

#[cfg(not(feature = "std"))]
pub use core2::io;
#[cfg(not(feature = "std"))]
pub use core2::error;
