//! Angles and convertion between units.
//!
//! Using [`Rad`](crate::angle::Rad) and [`Deg`](crate::angle::Deg) prevents the
//! user from accidentally calling [`.deg()`](crate::angle::Angular::deg)
//! twice on an angle. [`f64`](f64) may still be used as angle measured in
//! radians.

use std::f64::consts::PI;

/// Wrapper type storing angle expressed in radians.
///
/// ```
/// use veccentric::{Angular, Rad};
///
/// let half_pi: Rad = (3.14_f32 / 2.0).rad();
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rad(pub f64);

/// Wrapper type storing angle expressed in degrees.
///
/// ```
/// use veccentric::{Angular, Deg};
///
/// let half_pi: Deg = 90_i32.deg();
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Deg(pub f64);

/// A trait allowing to use generic angle type in some of
/// [`Fecc`](crate::fecc::Fecc)'s methods. See [`Angular`](crate::Angular).
pub trait Angle {
    /// Convert the angle type to radians.
    fn to_rad(&self) -> f64;
}

impl Angle for Rad {
    fn to_rad(&self) -> f64 {
        self.0
    }
}

impl Angle for Deg {
    fn to_rad(&self) -> f64 {
        self.0 * PI / 180.0
    }
}

impl Angle for f64 {
    fn to_rad(&self) -> f64 {
        *self
    }
}

/// A trait exposing a convenient API to work with angles. It is implemented for
/// every type `T` such that `T: Into<f64>` (i.e. [`i32`](i32), [`u32`](u32),
/// [`f64`](f64), [`f32`](f32)).
///
/// ```
/// use veccentric::{Angular, Deg, Fecc, Rad};
///
/// let half_pi_rad: Rad = (3.14_f32 / 2.0).rad();
/// let half_pi_deg: Deg = 90_i32.deg();
/// let half_pi_f64: f64 = 3.14 / 2.0;
///
/// let a = Fecc::new(1.0, 0.0);
/// let b = a.rotate(half_pi_rad);
/// let b = a.rotate(half_pi_deg);
/// let b = a.rotate(half_pi_f64);
/// ```
///
/// The API prevents calling [`deg()`](crate::Angular::deg) twice.
/// ```compile_fail
/// let pi = 180.0_f32.deg();
///
/// // Doesn't work! The first `pi` is of type `Deg` which has no such method.
/// let pi = pi.deg();
/// ```
pub trait Angular {
    /// Express the angle in radians.
    fn rad(&self) -> Rad;

    /// Express the angle in degrees.
    fn deg(&self) -> Deg;
}

impl<T> Angular for T
where
    f64: From<T>,
    T: Copy,
{
    fn rad(&self) -> Rad {
        Rad(f64::from(*self))
    }

    fn deg(&self) -> Deg {
        Deg(f64::from(*self))
    }
}
