#[cfg(feature = "random")]
use rand::{Rng, SeedableRng};

use crate::{Angle, Vecc};

/// Vector with two [`f64`](f64) components.
///
/// It implements the same methods as [`p5.Vector`](https://p5js.org/reference/#/p5.Vector)
/// (although some of them are named differently). Since
/// [`Fecc`](crate::fecc::Fecc) is [`Copy`](std::marker::Copy) none of the
/// methods mutates the vector, they may only return a new one.

pub type Fecc = Vecc<f64>;

impl Fecc {
    /// Construct a new vector of zero magnitude.
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

    /// Construct a new unit vector pointing in the specified direction.
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

    /// Construct a new unit vector pointing in random direction.
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

    /// Construct a new unit vector pointing in random direction.
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

    /// Normalize the vector (construct a new **unit** vector pointing in the
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

    /// Limit the magnitude of the vector.
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

    /// Set the magnitude of the vector, leaving its angle unchanged.
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

    /// Set the angle of the vector, leaving its magnitude unchanged.
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
    /// You can use this API to make sure the angles are correct.
    ///
    /// ```
    /// use veccentric::{Angular, Fecc};
    ///
    /// let a = Fecc::new(1.0, 0.0);
    ///
    /// // These all mean the same thing.
    /// let turned_a = a.turn(3.14);
    /// let turned_a = a.turn(3.14.rad());
    /// let turned_a = a.turn(180.0.deg());
    /// let turned_a = a.turn(180.deg());
    /// ```
    pub fn turn<A>(&self, angle: A) -> Self
    where
        A: Angle,
    {
        Self::from_angle(angle.to_rad()) * self.mag()
    }

    /// Reflect the vector about a normal.
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
    /// let reflected_a = a.reflect(normal);
    ///
    /// assert_approx_eq!(f64, reflected_a.x, -a.x);
    /// assert_approx_eq!(f64, reflected_a.y, a.y);
    /// ```
    pub fn reflect(&self, normal: Fecc) -> Self {
        -self - normal * 2.0 * self.dot(normal) / normal.dot(normal)
    }

    /// Rotate the vector, leaving its magnitude unchanged.
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
    /// You can use this API to express angles in either degrees or radians
    /// (radians are the default). Read more [here](crate::Angular).
    ///
    /// ```
    /// use veccentric::{Angular, Fecc};
    ///
    /// let a = Fecc::new(1.0, 0.0);
    ///
    /// // These all mean the same thing (except for the precision).
    /// let rotated_a = a.rotate(std::f64::consts::FRAC_PI_2);
    /// let rotated_a = a.rotate(3.14);
    /// let rotated_a = a.rotate(3.14.rad());
    /// let rotated_a = a.rotate(180.0.deg());
    /// let rotated_a = a.rotate(180.deg());
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

    /// Measure the square of the distance between two points (the tips of the
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

    /// Check whether the vector has zero magnitude.
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

    /// Measure the angle between two vectors.
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
        other.angle() - self.angle()
    }

    /// Measure the angle between the positive X axis and the vector.
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

    /// Measure the magnitude of the vector.
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

    /// Measure the square of the magnitude of the vector.
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

    /// Perform element-wise [`round`](f64::round) and convert the
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

    // TODO: Consider returning `Result` - [https://stackoverflow.com/questions/41138599/rounding-a-f64-to-nearest-i64-in-rust].
    pub fn round(&self) -> Vecc<i64> {
        Vecc {
            x: self.x.round() as i64,
            y: self.y.round() as i64,
        }
    }

    /// Perform element-wise [`floor`](f64::floor) and convert the
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

    // TODO: Consider returning `Result` - [https://stackoverflow.com/questions/41138599/rounding-a-f64-to-nearest-i64-in-rust].
    pub fn floor(&self) -> Vecc<i64> {
        Vecc {
            x: self.x.floor() as i64,
            y: self.y.floor() as i64,
        }
    }

    /// Perform element-wise [`ceil`](f64::ceil) and convert the
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

    // TODO: Consider returning `Result` - [https://stackoverflow.com/questions/41138599/rounding-a-f64-to-nearest-i64-in-rust].
    pub fn ceil(&self) -> Vecc<i64> {
        Vecc {
            x: self.x.ceil() as i64,
            y: self.y.ceil() as i64,
        }
    }
}
