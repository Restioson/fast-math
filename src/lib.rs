//! Fast, approximate versions of mathematical functions.
//!
//! This crate includes implementations of "expensive" mathematical
//! functions that are much faster, at the expense of some
//! accuracy. All functions have good guarantees on accuracy to some
//! degree (both relative and absolute).
//!
//! # Installation
//!
//! Add this to your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! fast-math = "0.1"
//! ```
//!
//! # Examples
//!
//! ```rust
//! let x = 10.4781;
//! let approx = fast_math::log2(x);
//! let real = x.log2();
//! // they should be close
//! assert!((approx - real).abs() < 0.01);
//! ```

#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#[cfg(test)] extern crate quickcheck;
#[cfg(all(test, feature = "unstable"))] extern crate test;

pub use log::{log2, log2_raw};

mod log;


#[doc(hidden)]
pub mod float;
