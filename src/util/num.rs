use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::Neg;

pub trait Num:
    num_traits::NumAssign + Neg<Output = Self> + PartialOrd + Sum<Self> + Copy + Display + Debug
{
    fn abs(self) -> Self;
}

impl Num for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Num for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Num for i128 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Num for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
