#![warn(missing_docs)]
#![feature(doc_cfg, auto_traits, negative_impls)]

//! 2D vector library inspired by [p5.js](https://p5js.org/)'s
//! [`p5.Vector`](https://p5js.org/reference/#/p5.Vector). Meant to be used in small gamedev projects.
//!
//! The main type, [`Vecc`](crate::vecc::Vecc), is a generic struct implementing
//! many useful traits. [`Fecc`](crate::fecc::Fecc) is a type alias for `Vecc<f64>`. It's API is heavily inspired by [`p5.Vector`](https://p5js.org/reference/#/p5.Vector)'s,
//! although some of the methods are named differently.
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
//! [`float_cmp::assert_approx_eq`](https://docs.rs/float-cmp/0.9.0/float_cmp/macro.assert_approx_eq.html) is used in some examples.
//!
//! # Examples
//!
//! For more examples, go to the [project's repo](https://github.com/micouy/veccentric/tree/master/examples).
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

/// Angles and convertion between units.
pub mod angle;

/// Generic vector with two components.
pub mod vecc;

/// Vector with two `f64` components.
pub mod fecc;

pub use angle::{Angle, Angular, Deg, Rad};
pub use fecc::Fecc;
pub use vecc::Vecc;
