use std::{cmp::Ord, ops::*};

/// Generic vector with two components.
///
/// It implements multiple operators (for each combination of owned and borrowed
/// args), namely `+`, `-`, element-wise `*`, element-wise `/`, multiplication
/// and division by a number. (Note that you can only multiply and divide in the
/// following order: `vector op number` since it is not possible to implement a
/// foreign trait on `T`.)

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Vecc<T> {
    #[allow(missing_docs)]
    pub x: T,

    #[allow(missing_docs)]
    pub y: T,
}

impl<T> Vecc<T> {
    /// Construct a new vector.
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

    /// Take a dot product of the vector with another.
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

    /// Take the cross-product (a scalar) of the vector with another.
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

    /// Perform element-wise [`min`](std::cmp::Ord::min).
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
    pub fn min(self, rhs: Vecc<T>) -> Vecc<T>
    where
        T: Ord,
    {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// Perform element-wise [`max`](std::cmp::Ord::max).
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
    pub fn max(self, rhs: Vecc<T>) -> Vecc<T>
    where
        T: Ord,
    {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Perform element-wise [`clamp`](std::cmp::Ord::clamp).
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
    pub fn clamp(self, min: Vecc<T>, max: Vecc<T>) -> Vecc<T>
    where
        T: Ord,
    {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

impl<T> From<(T, T)> for Vecc<T> {
    /// Construct a new vector from a tuple.
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

// FIXME: Doesn't work for some reason.

/*
impl<T, U> From<Vecc<U>> for Vecc<T> where T: From<U> {
    fn from(other: Vecc<U>) -> Vecc<T> {
        Vecc {
            x: From::from(other.x),
            y: From::from(other.y),
        }
    }
}
*/

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
