#![warn(missing_docs)]
#![feature(doc_cfg, auto_traits, negative_impls)]

//! Tiny 2D vector library. Inspired by [p5.js](https://p5js.org/)'s
//! [`p5.Vector`](https://p5js.org/reference/#/p5.Vector).
//!
//! The main type, [`Vecc<T>`](crate::vecc::Vecc), is a generic struct
//! implementing many useful traits and operator overloading.
//! [`Fecc`](crate::fecc::Fecc) is a type alias for [`Vecc<f64>`](crate::vecc::Vecc). It has an extended API, heavily inspired by [`p5.Vector`](https://p5js.org/reference/#/p5.Vector).
//!
//! # Features
//!
//! The `random` feature enables additional
//! methods on [`Fecc`](crate::fecc::Fecc):
//! [`from_rng`](crate::fecc::Fecc::from_rng),
//! [`from_seed`](crate::fecc::Fecc::from_seed),
//! [`from_entropy`](crate::fecc::Fecc::from_entropy).
//!
//! The `all` feature enables just `random`.
//!
//! # Notes
//!
//! [`float_cmp::assert_approx_eq`](https://docs.rs/float-cmp/0.9.0/float_cmp/macro.assert_approx_eq.html)
//! is used in some examples.
//!
//! # Examples
//!
//! ```
//! # use float_cmp::assert_approx_eq;
//! # use std::f64::consts::PI;
//! use veccentric::Fecc;
//!
//! let a = Fecc::new(3.0, 4.0);
//! assert_approx_eq!(f64, a.mag(), 5.0);
//!
//! let five_a = a * 5.0;
//! assert_approx_eq!(f64, five_a.mag(), 25.0);
//!
//! let b = Fecc::new(-3.0, 0.0);
//! let c = a + b; // (0.0, 4.0)
//! assert_approx_eq!(f64, c.angle(), PI / 2.0);
//! ```
//!
//! For more examples, go to [`Vecc`'s docs](crate::vecc::Vecc) or to
//! [the repository](https://github.com/micouy/veccentric/tree/master/examples).

pub mod angle;
pub mod fecc;
pub mod vecc;

pub use angle::{Angle, Angular};
pub use fecc::Fecc;
pub use vecc::Vecc;
