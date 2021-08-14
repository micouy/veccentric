#![warn(missing_docs)]
#![feature(doc_cfg)]

//! 2D vector library inspired by [p5.js](https://p5js.org/)'s
//! [`p5.Vecctor`](https://p5js.org/reference/#/p5.Vector).
//!
//! The main type, [`Vecctor`](crate::Vecctor), has two double-precision
//! components.
//!
//! # Features
//!
//! The `random` feature enables [`Vecctor`](crate::Vecctor)'s additional
//! methods: [`from_rng`](crate::Vecctor::from_rng),
//! [`from_seed`](crate::Vecctor::from_seed),
//! [`from_entropy`](crate::Vecctor::from_entropy).
//!
//! ```
//! use float_cmp::assert_approx_eq;
//! use std::f64::consts::PI;
//! use veccentric::Vecctor;
//!
//! let a = Vecctor::new(3.0, 4.0);
//! assert_approx_eq!(f64, a.mag(), 5.0);
//!
//! let five_a = a * 5.0;
//! assert_approx_eq!(f64, five_a.mag(), 25.0);
//!
//! let b = Vecctor::new(-3.0, 0.0);
//! let c = a + b; // (0, 4.0)
//! assert_approx_eq!(f64, c.angle(), PI / 2.0);
//! ```

// `std` imports.
use std::{f64::consts::PI, ops};

// External crate imports.
use overload::overload;
#[cfg(feature = "random")]
use rand::{Rng, SeedableRng};

/// The main vector type with two double-precision components.
///
/// It implements the same methods as [`p5.Vector`](https://p5js.org/reference/#/p5.Vector)
/// (although some of them are named differently). Since
/// [`Vecctor`](crate::Vecctor) is [`Copy`](std::marker::Copy) none of the
/// methods mutates the vector, they may only return a new one.
///
/// This type implements multiple operators (for each combination of owned and
/// borrowed args), namely `+`, `-`, element-wise `*`, element-wise `/`,
/// multiplication and division by a number, element-wise modulo by a number and
/// element-wise modulo by another vector (it may be useful when e.g. a game
/// object crosses a boundary of the game). The modulo operation is implemented
/// in such a way that it works even when the components of the original vector
/// are negative, for example `(-3) % 10` is `7`, not `-3`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vecctor {
    #[allow(missing_docs)]
    pub x: f64,
    #[allow(missing_docs)]
    pub y: f64,
}

impl Vecctor {
    /// Construct a new vector.
    ///
    /// ```
    /// use veccentric::Vecctor;
    ///
    /// let v = Vecctor::new(21.0, 37.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Construct a new vector of zero magnitude.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let zero = Vecctor::zero();
    ///
    /// assert_approx_eq!(f64, zero.mag(), 0.0);
    /// ```
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Construct a new unit vector pointing in the specified direction.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use std::f64::consts::PI;
    /// use veccentric::Vecctor;
    ///
    /// // 3.14 / 2 = PI / 2 = 90 degrees (upwards)
    /// let zero = Vecctor::from_angle(PI / 2.0);
    ///
    /// assert_approx_eq!(f64, zero.mag(), 1.0);
    /// ```
    pub fn from_angle<A>(angle: A) -> Self
    where
        A: Angle,
    {
        let angle = angle.to_rad();

        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Construct a new unit vector pointing in random direction.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use rand::{rngs::SmallRng, SeedableRng};
    /// use veccentric::Vecctor;
    ///
    /// let rng = SmallRng::from_seed([0xdd; 32]);
    /// let random = Vecctor::from_rng(rng);
    ///
    /// assert_approx_eq!(f64, random.mag(), 1.0);
    /// ```
    #[cfg(feature = "random")]
    #[doc(cfg(feature = "random"))]
    pub fn from_rng<R>(mut rng: R) -> Self
    where
        R: Rng,
    {
        let angle = rng.gen::<f64>();

        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Construct a new unit vector pointing in random direction.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use rand::rngs::SmallRng;
    /// use veccentric::Vecctor;
    ///
    /// let random = Vecctor::from_seed::<SmallRng>([0xdd; 32]);
    ///
    /// assert_approx_eq!(f64, random.mag(), 1.0);
    /// ```
    #[cfg(feature = "random")]
    #[doc(cfg(feature = "random"))]
    pub fn from_seed<R>(seed: R::Seed) -> Self
    where
        R: Rng + SeedableRng,
    {
        let mut rng = R::from_seed(seed);
        let angle = rng.gen::<f64>();

        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Construct a new unit vector pointing in random direction.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use rand::rngs::SmallRng;
    /// use veccentric::Vecctor;
    ///
    /// let random = Vecctor::from_entropy::<SmallRng>();
    ///
    /// assert_approx_eq!(f64, random.mag(), 1.0);
    /// ```
    #[cfg(feature = "random")]
    #[doc(cfg(feature = "random"))]
    pub fn from_entropy<R>() -> Self
    where
        R: Rng + SeedableRng,
    {
        let mut rng = R::from_entropy();
        let angle = rng.gen::<f64>();

        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Normalize the vector (construct a new **unit** vector pointing in the
    /// same direction as the original one).
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let original = Vecctor::new(10.0, 10.0);
    /// let normalized = original.normalize();
    ///
    /// assert_approx_eq!(f64, normalized.mag(), 1.0);
    /// ```
    pub fn normalize(&self) -> Self {
        if self.is_zero() {
            Vecctor::zero()
        } else {
            self / self.mag()
        }
    }

    /// Take a dot product of two vectors.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(10.0, 0.0);
    /// let b = Vecctor::new(10.0, 0.0);
    ///
    /// assert_approx_eq!(f64, a.dot(b), 100.0);
    /// ```
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Take the 'magnitude' (quotes, because it may be negative) of a cross
    /// product of two vectors.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(10.0, 0.0);
    /// let b = Vecctor::new(0.0, -10.0);
    ///
    /// assert_approx_eq!(f64, a.cross(b), -100.0);
    /// ```
    pub fn cross(&self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Limit the magnitude of the vector.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(100.0, 0.0);
    /// let a = a.limit(10.0);
    /// assert_approx_eq!(f64, a.mag(), 10.0);
    ///
    /// let b = Vecctor::new(1.0, 0.0);
    /// let b = b.limit(10.0);
    /// assert_approx_eq!(f64, b.mag(), 1.0);
    /// ```
    pub fn limit(&self, limit: f64) -> Self {
        let mag = self.mag();

        if mag > limit {
            *self * (limit / mag)
        } else {
            *self
        }
    }

    /// Set the magnitude of the vector, leaving its angle unchanged.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(2.0, -10.0);
    /// let a = a.resize(100.0);
    ///
    /// assert_approx_eq!(f64, a.mag(), 100.0);
    /// ```
    pub fn resize(&self, mag: f64) -> Self {
        *self * mag / self.mag()
    }

    /// Set the angle of the vector, leaving its magnitude unchanged.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// // `a` is pointing upwards.
    /// let a = Vecctor::new(0.0, 10.0);
    /// let a = a.turn(0.0);
    ///
    /// assert_approx_eq!(f64, a.angle(), 0.0);
    /// ```
    ///
    /// You can use this API to make sure the angles are correct.
    /// ```
    /// use veccentric::{Angular, Vecctor};
    ///
    /// let a = Vecctor::new(1.0, 0.0);
    ///
    /// // These all mean the same thing.
    /// let b = a.turn(3.14);
    /// let b = a.turn(3.14.rad());
    /// let b = a.turn(180.0.deg());
    /// let b = a.turn(180.deg());
    /// ```
    pub fn turn<A>(&self, angle: A) -> Self
    where
        A: Angle,
    {
        Self::from_angle(angle.to_rad()) * self.mag()
    }

    /// Rotate the vector, leaving its magnitude unchanged.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// let down = Vecctor::new(0.0, -10.0);
    /// let right = down.rotate(FRAC_PI_2);
    ///
    /// assert_approx_eq!(f64, right.angle(), 0.0);
    /// ```
    ///
    /// You can use this API to express angles in either degrees or radians
    /// (radians are the default). Read more [here](crate::Angular).
    /// ```
    /// use veccentric::{Angular, Vecctor};
    ///
    /// let a = Vecctor::new(1.0, 0.0);
    ///
    /// // These all mean the same thing (except for the precision).
    /// let b = a.rotate(std::f64::consts::FRAC_PI_2);
    /// let b = a.rotate(3.14);
    /// let b = a.rotate(3.14.rad());
    /// let b = a.rotate(180.0.deg());
    /// let b = a.rotate(180.deg());
    /// ```
    pub fn rotate<A>(&self, angle: A) -> Self
    where
        A: Angle,
    {
        let angle = angle.to_rad();

        Self {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
        }
    }

    /// Measure the distance between two points (the tips of the vectors
    /// pointing from the origin).
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(3.0, 0.0);
    /// let b = Vecctor::new(0.0, 4.0);
    ///
    /// assert_approx_eq!(f64, a.dist(b), 5.0);
    /// ```
    pub fn dist(&self, other: Self) -> f64 {
        (*self - other).mag()
    }

    /// Measure the square of the distance between two points (the tips of the
    /// vectors pointing from the origin).
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(3.0, 0.0);
    /// let b = Vecctor::new(0.0, 4.0);
    ///
    /// assert_approx_eq!(f64, a.dist_squared(b), 25.0);
    /// ```
    pub fn dist_squared(&self, other: Self) -> f64 {
        (*self - other).mag_squared()
    }

    /// Check whether the vector has zero magnitude.
    ///
    /// ```
    /// use veccentric::Vecctor;
    ///
    /// let zero = Vecctor::new(0.0, 0.0);
    /// let unit = Vecctor::new(1.0, 0.0);
    ///
    /// assert!(zero.is_zero());
    /// assert!(!unit.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        (self.x == 0.0) && (self.y == 0.0)
    }

    /// Measure the angle between two vectors.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use std::f64::consts::PI;
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(1.0, 0.0);
    /// let b = Vecctor::new(0.0, 1.0);
    ///
    /// assert_approx_eq!(f64, a.angle_to(b), PI / 2.0);
    /// ```
    pub fn angle_to(&self, other: Self) -> f64 {
        other.angle() - self.angle()
    }

    /// Measure the angle between the positive X axis and the vector.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use std::f64::consts::PI;
    /// use veccentric::Vecctor;
    ///
    /// let up = Vecctor::new(0.0, 1.0);
    ///
    /// assert_approx_eq!(f64, up.angle(), PI / 2.0);
    /// ```
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Measure the magnitude of the vector.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let five = Vecctor::new(3.0, 4.0);
    ///
    /// assert_approx_eq!(f64, five.mag(), 5.0);
    /// ```
    pub fn mag(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    /// Measure the square of the magnitude of the vector.
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use veccentric::Vecctor;
    ///
    /// let five = Vecctor::new(3.0, 4.0);
    ///
    /// assert_approx_eq!(f64, five.mag_squared(), 25.0);
    /// ```
    pub fn mag_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }

    /// Get the vector's components rounded to the nearest integer.
    ///
    /// ```
    /// use veccentric::Vecctor;
    ///
    /// let a = Vecctor::new(3.1, 4.6);
    /// let (x, y) = a.round();
    ///
    /// assert_eq!(x, 3);
    /// assert_eq!(y, 5);
    /// ```
    pub fn round(&self) -> (i32, i32) {
        (self.x.round() as i32, self.y.round() as i32)
    }
}
// Vecctor + Vecctor
overload!((a: ?Vecctor) + (b: ?Vecctor) -> Vecctor { Vecctor { x: a.x + b.x, y: a.y + b.y } });
// Vecctor - Vecctor
overload!((a: ?Vecctor) - (b: ?Vecctor) -> Vecctor { Vecctor { x: a.x - b.x, y: a.y - b.y } });

// Vecctor % Number
overload!((a: ?Vecctor) % (m: f64) -> Vecctor { Vecctor { x: ((a.x % m) + m) % m, y: ((a.y % m) + m) % m } });
// Vecctor % Vecctor
overload!((a: ?Vecctor) % (b: ?Vecctor) -> Vecctor { Vecctor { x: ((a.x % b.x) + b.x) % b.x, y: ((a.y % b.y) + b.y) % b.y } });

// Vecctor * Number
overload!((a: ?Vecctor) * (m: f64) -> Vecctor { Vecctor { x: a.x * m, y: a.y * m } });
// Vecctor * Vecctor
overload!((a: ?Vecctor) * (b: ?Vecctor) -> Vecctor { Vecctor { x: a.x * b.x, y: a.y * b.y } });

// Vecctor / Number
overload!((a: ?Vecctor) / (m: f64) -> Vecctor { Vecctor { x: a.x / m, y: a.y / m } });
// Vecctor / Vecctor
overload!((a: ?Vecctor) / (b: ?Vecctor) -> Vecctor { Vecctor { x: a.x / b.x, y: a.y / b.y } });

// Using `Rad` and `Deg` prevents the user from accidentally calling `.deg()`
// twice on an angle. `f64` may still be used as angle measured in radians.

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
/// [`Vecctor`](crate::Vecctor)'s methods. See [`Angular`](crate::Angular).
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
/// every type `T` such that `T: Into<f64>` (i.e. `i32`, `u32`, `f64`, `f32`).
///
/// ```
/// use veccentric::{Angular, Deg, Rad, Vecctor};
///
/// let half_pi_rad: Rad = (3.14_f32 / 2.0).rad();
/// let half_pi_deg: Deg = 90_i32.deg();
/// let half_pi_f64: f64 = 3.14 / 2.0;
///
/// let a = Vecctor::new(1.0, 0.0);
/// let b = a.rotate(half_pi_rad);
/// let b = a.rotate(half_pi_deg);
/// let b = a.rotate(half_pi_f64);
/// ```
///
/// The API prevents calling [`deg()`](crate::Angular::deg) twice.
/// ```compile_fail
/// let pi = 180.0.deg();
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
