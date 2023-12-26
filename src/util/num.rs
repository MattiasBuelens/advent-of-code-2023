use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};

pub trait Num:
    num_traits::Num
    + Neg<Output = Self>
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
