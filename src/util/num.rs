use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};

pub trait Num:
    num_traits::Num
    + Neg<Output = Self>
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Sum<Self>
    + Copy
    + Display
    + Debug
{
    fn abs(self) -> Self;

    fn into_decimal(self) -> f64;
}

impl Num for i32 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn into_decimal(self) -> f64 {
        self as f64
    }
}

impl Num for i64 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn into_decimal(self) -> f64 {
        self as f64
    }
}

impl Num for i128 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn into_decimal(self) -> f64 {
        self as f64
    }
}

impl Num for f64 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn into_decimal(self) -> f64 {
        self
    }
}
