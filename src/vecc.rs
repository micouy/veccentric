//! Generic vector with two components.

use std::{cmp::Ord, ops::*};

/// Generic vector with two components.
///
/// It implements multiple operators (for each combination of owned and borrowed
/// args), namely addition, subtraction, element-wise multiplication,
/// element-wise division and multiplication & division by a number. (Note that
/// you can only multiply and divide in the following order: `vector op number`,
/// since it is not possible to implement a foreign trait on `T`.)
///
/// This crate exports a specific version of [`Vecc`](crate::vecc::Vecc) with
/// [`f64`](f64) components — [`Fecc`](crate::fecc::Fecc). It implements
/// additional methods and is heavily inspired by [`p5.Vector`](https://p5js.org/reference/#/p5.Vector).
///
/// # Examples
///
/// Basic arithmetic.
///
/// ```
/// use veccentric::Vecc;
///
/// let a = Vecc::new(3_i32, 4);
/// let b = a * 5; // (15, 20)
/// let c = Vecc::new(-10, -8);
/// let d = b - c; // (5, 12)
/// let e = -d; // (-5, -12)
/// ```
///
/// Shorthand construction using [`From`](std::convert::From).
///
/// ```
/// use veccentric::Vecc;
///
/// let a: Vecc<i32> = (10, 5).into();
/// ```
///
/// Using [`Fecc`](crate::fecc::Fecc)'s extended API.
///
/// ```
/// # use float_cmp::assert_approx_eq;
/// # use std::f64::consts::PI;
/// use veccentric::Fecc;
///
/// let a: Fecc = (3.0, 4.0).into();
/// let b = a / 0.2; // (15.0, 20.0)
/// let c = b.limit(20.0); // (12.0, 16.0)
/// let d = c.rotate(PI); // (-12.0, -16.0)
/// let e = d.turn(0.0); // (20.0, 0.0)
///
/// assert_approx_eq!(f64, e.mag(), 20.0);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Vecc<T> {
    #[allow(missing_docs)]
    pub x: T,

    #[allow(missing_docs)]
    pub y: T,
}

impl<T> Vecc<T> {
    /// Constructs a new vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = Vecc::new(10, 0);
    /// ```
    ///
    /// You can also construct it from a tuple:
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = (10, 0).into();
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Takes a dot product of the vector with another.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = Vecc::new(10, 0);
    /// let b: Vecc<i32> = Vecc::new(5, 0);
    ///
    /// assert_eq!(a.dot(b), 50);
    /// ```
    pub fn dot(self, rhs: Vecc<T>) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Takes the cross-product (a scalar) of the vector with another.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = Vecc::new(10, 0);
    /// let b: Vecc<i32> = Vecc::new(0, -10);
    ///
    /// assert_eq!(a.cross(b), -100);
    /// ```
    pub fn cross(self, rhs: Vecc<T>) -> T
    where
        T: Sub<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * rhs.y - self.y * rhs.x
    }
}

/// Advanced Rust-magic. This trait is needed to implement `min` and `max` for
/// `Fecc`, otherwise it conflicts with `Vecc<T>`'s implementation. Big thanks to [u/fisgoda](https://www.reddit.com/user/figsoda/) ([link to Reddit post](https://www.reddit.com/r/rust/comments/paw1lm/implementation_of_from_for_generic_struct/)).
pub auto trait Notf64 {}
impl !Notf64 for f64 {}

impl<T> Vecc<T>
where
    T: Ord + Notf64,
{
    /// Performs element-wise [`min`](std::cmp::Ord::min).
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = Vecc::new(-100, 100);
    /// let b: Vecc<i32> = Vecc::new(0, 0);
    /// let min = a.min(b);
    ///
    /// assert_eq!(min.x, -100);
    /// assert_eq!(min.y, 0);
    /// ```
    pub fn min(self, rhs: Vecc<T>) -> Vecc<T> {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// Performs element-wise [`max`](std::cmp::Ord::max).
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = Vecc::new(-100, 100);
    /// let b: Vecc<i32> = Vecc::new(0, 0);
    /// let max = a.max(b);
    ///
    /// assert_eq!(max.x, 0);
    /// assert_eq!(max.y, 100);
    /// ```
    pub fn max(self, rhs: Vecc<T>) -> Vecc<T> {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Performs element-wise [`clamp`](std::cmp::Ord::clamp).
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = Vecc::new(-100, 100);
    /// let min: Vecc<i32> = Vecc::new(0, 10);
    /// let max: Vecc<i32> = Vecc::new(0, 10);
    /// let clamped = a.clamp(min, max);
    ///
    /// assert_eq!(clamped.x, 0);
    /// assert_eq!(clamped.y, 10);
    /// ```
    pub fn clamp(self, min: Vecc<T>, max: Vecc<T>) -> Vecc<T> {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

impl<T> From<(T, T)> for Vecc<T> {
    /// Constructs a new vector from a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use veccentric::Vecc;
    ///
    /// let a: Vecc<i32> = (10, 0).into();
    /// ```
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<(T, T)> for Vecc<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

/// Advanced Rust-magic. This trait is needed to implement `From<Vecc<U>> for
/// `Vecc<T>`, otherwise it conflicts with core's implementation of `From<T> for T` (when `U == T`). Big thanks to [u/fisgoda](https://www.reddit.com/user/figsoda/) ([link to Reddit post](https://www.reddit.com/r/rust/comments/paw1lm/implementation_of_from_for_generic_struct/)).
pub auto trait Different {}

impl<T> !Different for (T, T) {}

impl<T, U> From<Vecc<U>> for Vecc<T>
where
    T: From<U>,
    (T, U): Different,
{
    fn from(other: Vecc<U>) -> Vecc<T> {
        Vecc {
            x: From::from(other.x),
            y: From::from(other.y),
        }
    }
}

// Unary operators.

// Neg.

// Owned.
impl<T, U> Neg for Vecc<T>
where
    T: Neg<Output = U>,
{
    type Output = Vecc<U>;

    fn neg(self) -> Self::Output {
        Vecc {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

// Borrowed.
impl<T, U> Neg for &Vecc<T>
where
    T: Neg<Output = U> + Copy,
{
    type Output = Vecc<U>;

    fn neg(self) -> Self::Output {
        Vecc {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

// Not.

// Owned.
impl<T, U> Not for Vecc<T>
where
    T: Not<Output = U>,
{
    type Output = Vecc<U>;

    fn not(self) -> Self::Output {
        Vecc {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

// Borrowed.
impl<T, U> Not for &Vecc<T>
where
    T: Not<Output = U> + Copy,
{
    type Output = Vecc<U>;

    fn not(self) -> Self::Output {
        Vecc {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

// Binary operators.

// Add.

// Owned & owned.
impl<T> Add<Vecc<T>> for Vecc<T>
where
    T: Add<Output = T>,
{
    type Output = Vecc<T>;

    fn add(self, rhs: Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Owned & borrowed.
impl<T> Add<&Vecc<T>> for Vecc<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn add(self, rhs: &Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Borrowed & owned.
impl<T> Add<Vecc<T>> for &Vecc<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn add(self, rhs: Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl<T> Add<&Vecc<T>> for &Vecc<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn add(self, rhs: &Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

// Sub.

// Owned & owned.
impl<T> Sub<Vecc<T>> for Vecc<T>
where
    T: Sub<Output = T>,
{
    type Output = Vecc<T>;

    fn sub(self, rhs: Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Owned & borrowed.
impl<T> Sub<&Vecc<T>> for Vecc<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn sub(self, rhs: &Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Borrowed & owned.
impl<T> Sub<Vecc<T>> for &Vecc<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn sub(self, rhs: Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl<T> Sub<&Vecc<T>> for &Vecc<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn sub(self, rhs: &Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

// Mul with T.

// Owned & owned.
impl<T> Mul<T> for Vecc<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vecc {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

// Owned & borrowed.
impl<T> Mul<&T> for Vecc<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        Vecc {
            x: self.x.mul(*rhs),
            y: self.y.mul(*rhs),
        }
    }
}

// Borrowed & owned.
impl<T> Mul<T> for &Vecc<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vecc {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

// Borrowed & borrowed.
impl<T> Mul<&T> for &Vecc<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        Vecc {
            x: self.x.mul(*rhs),
            y: self.y.mul(*rhs),
        }
    }
}

// Div with T.

// Owned & owned.
impl<T> Div<T> for Vecc<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vecc {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

// Owned & borrowed.
impl<T> Div<&T> for Vecc<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn div(self, rhs: &T) -> Self::Output {
        Vecc {
            x: self.x.div(*rhs),
            y: self.y.div(*rhs),
        }
    }
}

// Borrowed & owned.
impl<T> Div<T> for &Vecc<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vecc {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

// Borrowed & borrowed.
impl<T> Div<&T> for &Vecc<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vecc<T>;

    fn div(self, rhs: &T) -> Self::Output {
        Vecc {
            x: self.x.div(*rhs),
            y: self.y.div(*rhs),
        }
    }
}

// Rem.

// Owned & owned.
impl<T> Rem<Vecc<T>> for Vecc<T>
where
    T: Rem<Output = T> + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

// Owned & borrowed.
impl<T> Rem<&Vecc<T>> for Vecc<T>
where
    T: Rem<Output = T> + Copy + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: &Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

// Borrowed & owned.
impl<T> Rem<Vecc<T>> for &Vecc<T>
where
    T: Rem<Output = T> + Copy + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

// Borrowed & borrowed.
impl<T> Rem<&Vecc<T>> for &Vecc<T>
where
    T: Rem<Output = T> + Copy + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: &Vecc<T>) -> Self::Output {
        Vecc {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

// Rem with T.

// Owned & owned.
impl<T> Rem<T> for Vecc<T>
where
    T: Rem<Output = T> + Copy + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Vecc {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

// Owned & borrowed.
impl<T> Rem<&T> for Vecc<T>
where
    T: Rem<Output = T> + Copy + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: &T) -> Self::Output {
        Vecc {
            x: self.x.rem(*rhs),
            y: self.y.rem(*rhs),
        }
    }
}

// Borrowed & owned.
impl<T> Rem<T> for &Vecc<T>
where
    T: Rem<Output = T> + Copy + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Vecc {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

// Borrowed & borrowed.
impl<T> Rem<&T> for &Vecc<T>
where
    T: Rem<Output = T> + Copy + Notf64,
{
    type Output = Vecc<T>;

    fn rem(self, rhs: &T) -> Self::Output {
        Vecc {
            x: self.x.rem(*rhs),
            y: self.y.rem(*rhs),
        }
    }
}

// *Assign.

// AddAssign.

// Owned.
impl<T> AddAssign<Vecc<T>> for Vecc<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, other: Vecc<T>) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
    }
}

// Borrowed.
impl<T> AddAssign<&Vecc<T>> for Vecc<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, other: &Vecc<T>) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
    }
}

// SubAssign.

// Owned.
impl<T> SubAssign<Vecc<T>> for Vecc<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Vecc<T>) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

// Borrowed.
impl<T> SubAssign<&Vecc<T>> for Vecc<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: &Vecc<T>) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

// MulAssign with T.

// Owned.
impl<T> MulAssign<T> for Vecc<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

// Borrowed.
impl<T> MulAssign<&T> for Vecc<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: &T) {
        self.x.mul_assign(*rhs);
        self.y.mul_assign(*rhs);
    }
}

// DivAssign with T.

// Owned.
impl<T> DivAssign<T> for Vecc<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

// Borrowed.
impl<T> DivAssign<&T> for Vecc<T>
where
    T: DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: &T) {
        self.x.div_assign(*rhs);
        self.y.div_assign(*rhs);
    }
}

// RemAssign.

// Owned.
impl<T> RemAssign<Vecc<T>> for Vecc<T>
where
    T: RemAssign<T> + Notf64,
{
    fn rem_assign(&mut self, rhs: Vecc<T>) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

// Borrowed.
impl<T> RemAssign<&Vecc<T>> for Vecc<T>
where
    T: RemAssign<T> + Copy + Notf64,
{
    fn rem_assign(&mut self, rhs: &Vecc<T>) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

// RemAssign with T.

// Owned.
impl<T> RemAssign<T> for Vecc<T>
where
    T: RemAssign<T> + Copy + Notf64,
{
    fn rem_assign(&mut self, rhs: T) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

// Borrowed.
impl<T> RemAssign<&T> for Vecc<T>
where
    T: RemAssign<T> + Copy + Notf64,
{
    fn rem_assign(&mut self, rhs: &T) {
        self.x.rem_assign(*rhs);
        self.y.rem_assign(*rhs);
    }
}
