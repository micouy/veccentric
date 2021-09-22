//! Implementation of [`Fecc`](crate::fecc::Fecc) - the alias for
//! [`Vecc<f64>`](crate::vecc::Vecc).

#[cfg(feature = "random")]
use rand::{Rng, SeedableRng};
use std::{
    f64::consts::PI,
    ops::{Rem, RemAssign},
};

use crate::{Angle, Vecc};

/// Vector with two [`f64`](f64) components.
///
/// It implements the same methods as [`p5.Vector`](https://p5js.org/reference/#/p5.Vector)
/// (although some of them are named differently). Since
/// [`Fecc`](Fecc) is [`Copy`](std::marker::Copy) none of the
/// methods mutates the vector, they may only return a new one.
///
/// Note that [`Fecc`](Fecc)'s implementations of [`Rem`](std::ops::Rem)
/// and [`RemAssign`](std::ops::RemAssign) use
/// [`f64::rem_euclid`](f64::rem_euclid), not [`f64::rem`](Rem::rem).
/// (TODO: Link `f64::rem` directly after [#74563](https://github.com/rust-lang/rust/issues/74563)
/// is resolved.) This kind of modulo operation is expected to be more useful
/// for game developers since it is used to emulate 'wrapping' of a game
/// object's position.
///
/// # Examples
///
/// ```
/// # use float_cmp::assert_approx_eq;
/// # use std::f64::consts::PI;
/// use veccentric::Fecc;
///
/// let a: Fecc = (3.0, 4.0).into();
/// let b = a * 5.0; // (15.0, 20.0)
/// let c = b.limit(20.0); // (12.0, 16.0)
/// let d = c.rotate(PI); // (-12.0, -16.0)
/// let e = d.turn(0.0); // (20.0, 0.0)
///
/// assert_approx_eq!(f64, e.mag(), 20.0);
/// ```
pub type Fecc = Vecc<f64>;

impl Fecc {
    /// Constructs a new vector of zero magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let zero = Fecc::zero();
    ///
    /// assert_approx_eq!(f64, zero.mag(), 0.0);
    /// ```
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Constructs a new unit vector pointing in the specified direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// # use std::f64::consts::PI;
    /// use veccentric::Fecc;
    ///
    /// // 3.14 / 2 = PI / 2 = 90 degrees (upwards)
    /// let one_up = Fecc::from_angle(PI / 2.0);
    ///
    /// assert_approx_eq!(f64, one_up.mag(), 1.0);
    /// ```
    pub fn from_angle<A>(angle: A) -> Self
    where
        A: Into<Angle>,
    {
        let angle = angle.into();

        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Constructs a new unit vector pointing in random direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use rand::{rngs::SmallRng, SeedableRng};
    /// use veccentric::Fecc;
    ///
    /// let rng = SmallRng::from_seed([0xdd; 32]);
    /// let random = Fecc::from_rng(rng);
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

    /// Constructs a new unit vector pointing in random direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use rand::rngs::SmallRng;
    /// use veccentric::Fecc;
    ///
    /// let random = Fecc::from_seed::<SmallRng>([0xdd; 32]);
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

    /// Constructs a new unit vector pointing in random direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use rand::rngs::SmallRng;
    /// use veccentric::Fecc;
    ///
    /// let random = Fecc::from_entropy::<SmallRng>();
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

    /// Normalizes the vector (construct a new **unit** vector pointing in the
    /// same direction as the original one).
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(10.0, 10.0);
    /// let normalized = a.normalize();
    ///
    /// assert_approx_eq!(f64, normalized.mag(), 1.0);
    /// ```
    pub fn normalize(&self) -> Self {
        if self.is_zero() {
            Fecc::zero()
        } else {
            self / self.mag()
        }
    }

    /// Limits the magnitude of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(100.0, 0.0);
    /// let limited_a = a.limit(10.0);
    /// assert_approx_eq!(f64, limited_a.mag(), 10.0);
    ///
    /// let b = Fecc::new(1.0, 0.0);
    /// let limited_b = b.limit(10.0);
    /// assert_approx_eq!(f64, limited_b.mag(), 1.0);
    /// ```
    pub fn limit(&self, limit: f64) -> Self {
        let mag = self.mag();

        if mag > limit {
            *self * (limit / mag)
        } else {
            *self
        }
    }

    /// Sets the magnitude of the vector, leaving its angle unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(2.0, -10.0);
    /// let resized_a = a.resize(100.0);
    ///
    /// assert_approx_eq!(f64, resized_a.mag(), 100.0);
    /// ```
    pub fn resize(&self, mag: f64) -> Self {
        *self * mag / self.mag()
    }

    /// Sets the angle of the vector, leaving its magnitude unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// // `a` is pointing upwards.
    /// let a = Fecc::new(0.0, 10.0);
    /// let turned_a = a.turn(0.0);
    ///
    /// assert_approx_eq!(f64, turned_a.angle(), 0.0);
    /// ```
    ///
    /// The [`Angular`](crate::angle::Angular) trait allows the user specify the
    /// units of the angle but it is not required (radians are the default
    /// unit).
    ///
    /// ```
    /// use veccentric::{Angular, Fecc};
    ///
    /// let a = Fecc::new(1.0, 0.0);
    ///
    /// // These all mean the same thing (except for precision).
    /// let turned_a = a.turn(3.14);
    /// let turned_a = a.turn(3.14.rad());
    /// let turned_a = a.turn(180.0.deg());
    /// let turned_a = a.turn(180.deg());
    /// ```
    pub fn turn<A>(&self, angle: A) -> Self
    where
        A: Into<Angle>,
    {
        Self::from_angle(angle.into()) * self.mag()
    }

    /// Rotates the vector, leaving its magnitude unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// let a = Fecc::new(0.0, -10.0);
    /// let rotated_a = a.rotate(FRAC_PI_2);
    ///
    /// assert_approx_eq!(f64, rotated_a.angle(), 0.0);
    /// ```
    ///
    ///
    /// The [`Angular`](crate::angle::Angular) trait allows the user specify the
    /// units of the angle but it is not required (radians are the default
    /// unit).
    ///
    /// ```
    /// use veccentric::{Angular, Fecc};
    ///
    /// let a = Fecc::new(1.0, 0.0);
    ///
    /// // These all mean the same thing (except for precision).
    /// let rotated_a = a.rotate(std::f64::consts::FRAC_PI_2);
    /// let rotated_a = a.rotate(3.14);
    /// let rotated_a = a.rotate(3.14.rad());
    /// let rotated_a = a.rotate(180.0.deg());
    /// let rotated_a = a.rotate(180.deg());
    /// ```
    pub fn rotate<A>(&self, angle: A) -> Self
    where
        A: Into<Angle>,
    {
        let angle = angle.into();

        Self {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
        }
    }

    /// Reflects the vector about a normal. Reflection about a zero vector
    /// results in the original vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// // `a` is pointing right.
    /// let a = Fecc::new(5.0, 0.0);
    /// // `normal` is poiting upwards.
    /// let normal = Fecc::new(0.0, 1.0);
    /// // `reflected_a` is pointing left.
    /// let reflected_a = a.reflect(normal);
    ///
    /// assert_approx_eq!(f64, reflected_a.x, -a.x);
    /// assert_approx_eq!(f64, reflected_a.y, a.y);
    /// ```
    pub fn reflect(&self, normal: Fecc) -> Self {
        if normal.is_zero() {
            *self
        } else {
            -(self + self.project(normal) * 2.0)
        }
    }

    /// Projects a vector onto another. Projection onto a zero vector results in
    /// the original vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(1.0, 3.0);
    /// let b = Fecc::new(4.0, 1.0);
    /// let projected_a = a.project(b);
    ///
    /// assert_approx_eq!(f64, b.angle(), projected_a.angle());
    /// ```
    pub fn project(&self, other: Self) -> Self {
        if other.is_zero() {
            *self
        } else {
            other * self.dot(other) / other.dot(other)
        }
    }

    /// Returns the distance between two points (the tips of the vectors
    /// pointing from the origin).
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(3.0, 0.0);
    /// let b = Fecc::new(0.0, 4.0);
    ///
    /// assert_approx_eq!(f64, a.dist(b), 5.0);
    /// ```
    pub fn dist(&self, other: Self) -> f64 {
        (*self - other).mag()
    }

    /// Returns the square of the distance between two points (the tips of the
    /// vectors pointing from the origin).
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(3.0, 0.0);
    /// let b = Fecc::new(0.0, 4.0);
    ///
    /// assert_approx_eq!(f64, a.dist_squared(b), 25.0);
    /// ```
    pub fn dist_squared(&self, other: Self) -> f64 {
        (*self - other).mag_squared()
    }

    /// Checks whether the vector has zero magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Fecc;
    ///
    /// let zero = Fecc::new(0.0, 0.0);
    /// let one = Fecc::new(1.0, 0.0);
    ///
    /// assert!(zero.is_zero());
    /// assert!(!one.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        (self.x == 0.0) && (self.y == 0.0)
    }

    /// Returns the angle between two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// # use std::f64::consts::PI;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(1.0, 0.0);
    /// let b = Fecc::new(0.0, 1.0);
    ///
    /// assert_approx_eq!(f64, a.angle_to(b), PI / 2.0);
    /// ```
    pub fn angle_to(&self, other: Self) -> f64 {
        let angle = other.angle() - self.angle();

        if angle > PI {
            angle - 2.0 * PI
        } else if angle < -PI {
            angle + 2.0 * PI
        } else {
            angle
        }
    }

    /// Returns the angle between the positive X axis and the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// # use std::f64::consts::PI;
    /// use veccentric::Fecc;
    ///
    /// let up = Fecc::new(0.0, 1.0);
    ///
    /// assert_approx_eq!(f64, up.angle(), PI / 2.0);
    /// ```
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Returns the magnitude of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let five = Fecc::new(3.0, 4.0);
    ///
    /// assert_approx_eq!(f64, five.mag(), 5.0);
    /// ```
    pub fn mag(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    /// Returns the square of the magnitude of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let five = Fecc::new(3.0, 4.0);
    ///
    /// assert_approx_eq!(f64, five.mag_squared(), 25.0);
    /// ```
    pub fn mag_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }

    /// Performs component-wise [`round`](f64::round) and convert the
    /// components to `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(3.1, 4.6);
    /// let rounded_a = a.round();
    ///
    /// assert_eq!(rounded_a.x, 3);
    /// assert_eq!(rounded_a.y, 5);
    /// ```

    // TODO: Consider returning `Result` — [https://stackoverflow.com/questions/41138599/rounding-a-f64-to-nearest-i64-in-rust].
    pub fn round(&self) -> Vecc<i64> {
        Vecc {
            x: self.x.round() as i64,
            y: self.y.round() as i64,
        }
    }

    /// Performs component-wise [`floor`](f64::floor) and convert the
    /// components to `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(3.1, 4.6);
    /// let floored_a = a.floor();
    ///
    /// assert_eq!(floored_a.x, 3);
    /// assert_eq!(floored_a.y, 4);
    /// ```

    // TODO: Consider returning `Result` — [https://stackoverflow.com/questions/41138599/rounding-a-f64-to-nearest-i64-in-rust].
    pub fn floor(&self) -> Vecc<i64> {
        Vecc {
            x: self.x.floor() as i64,
            y: self.y.floor() as i64,
        }
    }

    /// Performs component-wise [`ceil`](f64::ceil) and convert the
    /// components to `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(3.1, 4.6);
    /// let ceiled_a = a.ceil();
    ///
    /// assert_eq!(ceiled_a.x, 4);
    /// assert_eq!(ceiled_a.y, 5);
    /// ```

    // TODO: Consider returning `Result` — [https://stackoverflow.com/questions/41138599/rounding-a-f64-to-nearest-i64-in-rust].
    pub fn ceil(&self) -> Vecc<i64> {
        Vecc {
            x: self.x.ceil() as i64,
            y: self.y.ceil() as i64,
        }
    }

    /// Performs element-wise [`min`](f64::min).
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(-100.0, 100.0);
    /// let b = Fecc::new(0.0, 0.0);
    /// let min = a.min(b);
    ///
    /// assert_approx_eq!(f64, min.x, -100.0);
    /// assert_approx_eq!(f64, min.y, 0.0);
    /// ```
    pub fn min(&self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Performs element-wise [`max`](f64::max).
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(-100.0, 100.0);
    /// let b = Fecc::new(0.0, 0.0);
    /// let max = a.max(b);
    ///
    /// assert_approx_eq!(f64, max.x, 0.0);
    /// assert_approx_eq!(f64, max.y, 100.0);
    /// ```
    pub fn max(&self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Performs element-wise [`clamp`](f64::clamp).
    ///
    /// # Examples
    ///
    /// ```
    /// # use float_cmp::assert_approx_eq;
    /// use veccentric::Fecc;
    ///
    /// let a = Fecc::new(-100.0, 100.0);
    /// let min = Fecc::new(0.0, 10.0);
    /// let max = Fecc::new(0.0, 10.0);
    /// let clamped = a.clamp(min, max);
    ///
    /// assert_approx_eq!(f64, clamped.x, 0.0);
    /// assert_approx_eq!(f64, clamped.y, 10.0);
    /// ```
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

// Euclidean modulo.

// Owned & owned.
impl Rem<Fecc> for Fecc {
    type Output = Fecc;

    fn rem(self, rhs: Fecc) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

// Owned & borrowed.
impl Rem<&Fecc> for Fecc {
    type Output = Fecc;

    fn rem(self, rhs: &Fecc) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

// Borrowed & owned.
impl Rem<Fecc> for &Fecc {
    type Output = Fecc;

    fn rem(self, rhs: Fecc) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl Rem<&Fecc> for &Fecc {
    type Output = Fecc;

    fn rem(self, rhs: &Fecc) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

// Euclidean modulo with f64.

// Owned & owned.
impl Rem<f64> for Fecc {
    type Output = Fecc;

    fn rem(self, rhs: f64) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(rhs),
            y: self.y.rem_euclid(rhs),
        }
    }
}

// Owned & borrowed.
impl Rem<&f64> for Fecc {
    type Output = Fecc;

    fn rem(self, rhs: &f64) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(*rhs),
            y: self.y.rem_euclid(*rhs),
        }
    }
}

// Borrowed & owned.
impl Rem<f64> for &Fecc {
    type Output = Fecc;

    fn rem(self, rhs: f64) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(rhs),
            y: self.y.rem_euclid(rhs),
        }
    }
}

// Borrowed & borrowed.
impl Rem<&f64> for &Fecc {
    type Output = Fecc;

    fn rem(self, rhs: &f64) -> Self::Output {
        Vecc {
            x: self.x.rem_euclid(*rhs),
            y: self.y.rem_euclid(*rhs),
        }
    }
}

// (Euclidean modulo)Assign.

// Owned.
impl RemAssign<Fecc> for Fecc {
    fn rem_assign(&mut self, rhs: Fecc) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}

// Borrowed.
impl RemAssign<&Fecc> for Fecc {
    fn rem_assign(&mut self, rhs: &Fecc) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}

// (Euclidean modulo)Assign with f64.

// Owned.
impl RemAssign<f64> for Fecc {
    fn rem_assign(&mut self, rhs: f64) {
        self.x = self.x.rem_euclid(rhs);
        self.y = self.y.rem_euclid(rhs);
    }
}

// Borrowed.
impl RemAssign<&f64> for Fecc {
    fn rem_assign(&mut self, rhs: &f64) {
        self.x = self.x.rem_euclid(*rhs);
        self.y = self.y.rem_euclid(*rhs);
    }
}
