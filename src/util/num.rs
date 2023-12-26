use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::Neg;

pub trait Num:
    num_traits::NumAssign + Neg<Output = Self> + PartialOrd + Sum<Self> + Copy + Display + Debug
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
