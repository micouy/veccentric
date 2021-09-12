//! Angles and convertion between units.
//!
//! Using [`Rad`](crate::angle::Rad) and [`Deg`](crate::angle::Deg) prevents the
//! user from accidentally calling [`.deg()`](crate::angle::Angular::deg)
//! twice on an angle. [`f64`](f64) may still be used as angle measured in
//! radians.

use std::{f64::consts::PI, ops::*};

/// Wrapper type storing angle expressed in radians.
///
/// ```
/// use veccentric::{Angle, Angular};
///
/// let half_pi: Angle = (3.14_f32 / 2.0).rad();
/// ```
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Angle(pub f64);

/// A trait exposing a convenient API to work with angles. It is implemented for
/// every type `T` such that `T: Into<f64>` (i.e. [`i32`](i32), [`u32`](u32),
/// [`f64`](f64), [`f32`](f32)).
///
/// ```
/// use veccentric::{Angular, Fecc};
///
/// let half_pi_rad: Angle = (3.14_f32 / 2.0).rad();
/// let half_pi_deg: Angle = 90_i32.deg();
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
/// // Doesn't work! The first `pi` is of type `Angle` which has no such method.
/// let pi = pi.deg();
/// ```

impl Deref for Angle {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

// Neg.

// Owned.
impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Angle {
        Angle(self.0.neg())
    }
}

// Borrowed.
impl Neg for &Angle {
    type Output = Angle;

    fn neg(self) -> Angle {
        Angle(self.0.neg())
    }
}

// Add.

// Owned & owned.
impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Angle(self.0.add(rhs.0))
    }
}

// Owned & borrowed.
impl Add<&Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: &Angle) -> Self::Output {
        Angle(self.0.add(rhs.0))
    }
}

// Borrowed & owned.
impl Add<Angle> for &Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Angle(self.0.add(rhs.0))
    }
}

// Borrowed & borrowed.
impl Add<&Angle> for &Angle {
    type Output = Angle;

    fn add(self, rhs: &Angle) -> Self::Output {
        Angle(self.0.add(rhs.0))
    }
}

// Sub.

// Owned & owned.
impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Self::Output {
        Angle(self.0.sub(rhs.0))
    }
}

// Owned & borrowed.
impl Sub<&Angle> for Angle {
    type Output = Angle;

    fn sub(self, rhs: &Angle) -> Self::Output {
        Angle(self.0.sub(rhs.0))
    }
}

// Borrowed & owned.
impl Sub<Angle> for &Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Self::Output {
        Angle(self.0.sub(rhs.0))
    }
}

// Borrowed & borrowed.
impl Sub<&Angle> for &Angle {
    type Output = Angle;

    fn sub(self, rhs: &Angle) -> Self::Output {
        Angle(self.0.sub(rhs.0))
    }
}

// Mul with f64.

// Owned & owned.
impl Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Self::Output {
        Angle(self.0.mul(rhs))
    }
}

// Owned & borrowed.
impl Mul<&f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: &f64) -> Self::Output {
        Angle(self.0.mul(rhs))
    }
}

// Borrowed & owned.
impl Mul<f64> for &Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Self::Output {
        Angle(self.0.mul(rhs))
    }
}

// Borrowed & borrowed.
impl Mul<&f64> for &Angle {
    type Output = Angle;

    fn mul(self, rhs: &f64) -> Self::Output {
        Angle(self.0.mul(rhs))
    }
}

// Div with f64.

// Owned & owned.
impl Div<f64> for Angle {
    type Output = Angle;

    fn div(self, rhs: f64) -> Self::Output {
        Angle(self.0.div(rhs))
    }
}

// Owned & borrowed.
impl Div<&f64> for Angle {
    type Output = Angle;

    fn div(self, rhs: &f64) -> Self::Output {
        Angle(self.0.div(rhs))
    }
}

// Borrowed & owned.
impl Div<f64> for &Angle {
    type Output = Angle;

    fn div(self, rhs: f64) -> Self::Output {
        Angle(self.0.div(rhs))
    }
}

// Borrowed & borrowed.
impl Div<&f64> for &Angle {
    type Output = Angle;

    fn div(self, rhs: &f64) -> Self::Output {
        Angle(self.0.div(rhs))
    }
}

// AddAssign.

// Owned.
impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        self.0.add_assign(rhs.0);
    }
}

// Borrowed.
impl AddAssign<&Angle> for Angle {
    fn add_assign(&mut self, rhs: &Angle) {
        self.0.add_assign(rhs.0);
    }
}

// SubAssign.

// Owned.
impl SubAssign<Angle> for Angle {
    fn sub_assign(&mut self, rhs: Angle) {
        self.0.sub_assign(rhs.0);
    }
}

// Borrowed.
impl SubAssign<&Angle> for Angle {
    fn sub_assign(&mut self, rhs: &Angle) {
        self.0.sub_assign(rhs.0);
    }
}

impl<T> From<T> for Angle
where
    f64: From<T>,
{
    fn from(angle: T) -> Self {
        Self(angle.into())
    }
}

pub trait Angular {
    fn rad(&self) -> Angle;
    fn deg(&self) -> Angle;
}

impl<T> Angular for T
where
    f64: From<T>,
    T: Copy,
{
    fn rad(&self) -> Angle {
        Angle(f64::from(*self))
    }

    fn deg(&self) -> Angle {
        Angle(f64::from(*self) * PI / 180.0)
    }
}
