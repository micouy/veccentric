//! Angles.

use std::{f64::consts::PI, ops::*};

/// Wrapper type storing angle expressed in radians.
///
/// ```
/// use veccentric::{Angle, Angular};
///
/// let half_pi: Angle = (3.14_f32 / 2.0).rad();
/// ```
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Angle(f64);

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

/// A trait exposing a convenient API to work with angles. It is implemented for
/// every type implementing `Into<f64>`.
///
/// ```
/// use veccentric::{Angular, Angle, Fecc};
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
/// The API prevents calling [`deg()`](Angular::deg) or [`rad()`](Angular::rad)  twice.
/// ```compile_fail
/// let pi = 180.0_f32.deg();
///
/// // Doesn't work! `pi` is of type `Angle` which has no such method.
/// let not_pi = pi.deg();
/// ```
pub trait Angular {
    /// Interpret the value as radians.
    fn rad(self) -> Angle;

    /// Interpret the value as degrees.
    fn deg(self) -> Angle;
}

impl<T> Angular for T
where
    T: Into<f64> {
    fn rad(self) -> Angle {
        Angle(self.into())
    }

    fn deg(self) -> Angle {
        Angle(self.into() * PI / 180.0)
    }
}
