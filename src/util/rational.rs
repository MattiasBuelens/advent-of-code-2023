use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use num_traits::{One, Zero};

use crate::util::{gcd, lcm};

use super::num::Num;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rational<T: Num> {
    nominator: T,
    denominator: T,
}

impl<T: Num> Rational<T> {
    #[inline]
    pub fn new(mut nominator: T, mut denominator: T) -> Self {
        if denominator < T::zero() {
            nominator = nominator.neg();
            denominator = denominator.neg();
        }
        assert!(denominator > T::zero());
        let gcd = gcd(nominator, denominator);
        Self::new_unchecked(nominator / gcd, denominator / gcd)
    }

    #[inline]
    fn new_unchecked(nominator: T, denominator: T) -> Self {
        assert!(denominator > T::zero());
        Self {
            nominator,
            denominator,
        }
    }

    #[inline]
    pub fn nominator(&self) -> T {
        self.nominator
    }

    #[inline]
    pub fn denominator(&self) -> T {
        self.denominator
    }

    fn same_denominators(left: Self, right: Self) -> (Self, Self) {
        let denominator = lcm(left.denominator, right.denominator);
        (
            Self::new_unchecked(
                left.nominator * (denominator / left.denominator),
                denominator,
            ),
            Self::new_unchecked(
                right.nominator * (denominator / right.denominator),
                denominator,
            ),
        )
    }
}

impl<T: Num> From<T> for Rational<T> {
    fn from(value: T) -> Self {
        Self::new(value, T::one())
    }
}

impl<T: Num> Add for Rational<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (left, right) = Self::same_denominators(self, rhs);
        Self::new(left.nominator + right.nominator, left.denominator)
    }
}

impl<T: Num> Add<T> for Rational<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.add(Rational::from(rhs))
    }
}

impl<T: Num> AddAssign for Rational<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl<T: Num> Sub for Rational<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (left, right) = Self::same_denominators(self, rhs);
        Self::new(left.nominator - right.nominator, left.denominator)
    }
}

impl<T: Num> Sub<T> for Rational<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self.sub(Rational::from(rhs))
    }
}

impl<T: Num> SubAssign for Rational<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl<T: Num> Neg for Rational<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new_unchecked(self.nominator.neg(), self.denominator)
    }
}

impl<T: Num> Mul for Rational<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.nominator * rhs.nominator,
            self.denominator * rhs.denominator,
        )
    }
}

impl<T: Num> Mul<T> for Rational<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul(Rational::from(rhs))
    }
}

impl<T: Num> MulAssign for Rational<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs)
    }
}

impl<T: Num> Div for Rational<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.nominator * rhs.denominator,
            self.denominator * rhs.nominator,
        )
    }
}

impl<T: Num> DivAssign for Rational<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs)
    }
}

impl<T: Num> Rem for Rational<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        assert!(rhs.denominator.is_one());
        Self::new(self.nominator % rhs.nominator, self.denominator)
    }
}

impl<T: Num> PartialOrd for Rational<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (left, right) = Self::same_denominators(*self, *other);
        left.nominator.partial_cmp(&right.nominator)
    }
}

impl<T: Num> Zero for Rational<T> {
    fn zero() -> Self {
        Self::from(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.nominator.is_zero()
    }
}

impl<T: Num> One for Rational<T> {
    fn one() -> Self {
        Self::from(T::one())
    }

    fn is_one(&self) -> bool {
        self.nominator.is_one()
    }
}

impl<T: Num> Sum<Self> for Rational<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<T: Num> Display for Rational<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.nominator, self.denominator)
    }
}

impl<T: Num> num_traits::Num for Rational<T> {
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Rational::from(T::from_str_radix(str, radix)?))
    }
}

impl<T: Num> Num for Rational<T> {
    fn abs(self) -> Self {
        Self::new_unchecked(self.nominator.abs(), self.denominator)
    }

    fn into_decimal(self) -> f64 {
        self.nominator.into_decimal() / self.denominator.into_decimal()
    }
}
