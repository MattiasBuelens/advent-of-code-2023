use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::Zero;

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

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Vector<const N: usize, T: Num = i32> {
    pub coords: [T; N],
}

#[allow(dead_code)]
impl<const N: usize, T: Num> Vector<N, T> {
    #[inline]
    pub fn zero() -> Self {
        Self {
            coords: [T::zero(); N],
        }
    }

    #[inline]
    pub fn abs(mut self) -> Self {
        self.map_in_place(|x| x.abs());
        self
    }

    #[inline]
    pub fn manhattan_distance(&self) -> T {
        self.coords.iter().map(|x| x.abs()).sum()
    }

    #[inline]
    pub fn for_each(&mut self, f: impl FnMut(&mut T)) {
        self.coords.iter_mut().for_each(f);
    }

    #[inline]
    pub fn map_in_place(&mut self, mut f: impl FnMut(T) -> T) {
        self.for_each(|x| *x = f(*x))
    }

    #[inline]
    pub fn map(&self, f: impl FnMut(T) -> T) -> Self {
        Self {
            coords: self.coords.map(f),
        }
    }

    #[inline]
    pub fn zip_in_place(&mut self, other: &Self, mut f: impl FnMut(&mut T, T)) {
        for i in 0..N {
            f(&mut self.coords[i], other.coords[i]);
        }
    }

    #[inline]
    pub fn zip_with(&self, other: &Self, mut f: impl FnMut(T, T) -> T) -> Self {
        let mut result = *self;
        for i in 0..N {
            result.coords[i] = f(result.coords[i], other.coords[i]);
        }
        result
    }

    #[inline]
    pub fn from_iter(iter: impl Iterator<Item = T>) -> Self {
        let mut coords = [Zero::zero(); N];
        for (i, value) in iter.take(N).enumerate() {
            coords[i] = value
        }
        Self { coords }
    }
}

impl<const N: usize, T: Num> Default for Vector<N, T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize, T: Num> Display for Vector<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, coord) in self.coords.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", coord)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<const N: usize, T: Num> Debug for Vector<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple(&format!("Vector<{}>", N));
        for coord in self.coords.iter() {
            f.field(coord);
        }
        f.finish()
    }
}

impl<const N: usize, T: Num> From<[T; N]> for Vector<N, T> {
    fn from(coords: [T; N]) -> Self {
        Self { coords }
    }
}

impl<const N: usize, T: Num> From<Vector<N, T>> for [T; N] {
    fn from(vector: Vector<N, T>) -> Self {
        vector.coords
    }
}

impl<const N: usize, T: Num> Add for Vector<N, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.zip_with(&other, |x, y| x + y)
    }
}

impl<const N: usize, T: Num> Sub for Vector<N, T> {
    type Output = Self;

    fn sub(self: Self, other: Self) -> Self {
        self.zip_with(&other, |x, y| x - y)
    }
}

impl<const N: usize, T: Num> Neg for Vector<N, T> {
    type Output = Self;

    fn neg(self) -> Self {
        self.map(|x| x.neg())
    }
}

impl<const N: usize, T: Num> AddAssign for Vector<N, T> {
    fn add_assign(&mut self, other: Self) {
        self.zip_in_place(&other, |x, y| x.add_assign(y));
    }
}

impl<const N: usize, T: Num> SubAssign for Vector<N, T> {
    fn sub_assign(&mut self, other: Self) {
        self.zip_in_place(&other, |x, y| x.sub_assign(y));
    }
}

impl<const N: usize, T: Num> Mul<T> for Vector<N, T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.map(|x| x * rhs)
    }
}

impl<const N: usize, T: Num> MulAssign<T> for Vector<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        self.for_each(|x| x.mul_assign(rhs));
    }
}

impl<const N: usize, T: Num> Div<T> for Vector<N, T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

impl<const N: usize, T: Num> DivAssign<T> for Vector<N, T> {
    fn div_assign(&mut self, rhs: T) {
        self.for_each(|x| x.div_assign(rhs));
    }
}

pub type Vector2D<T = i32> = Vector<2, T>;

#[allow(dead_code)]
impl<T: Num> Vector2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { coords: [x, y] }
    }

    #[inline]
    pub fn x(&self) -> T {
        self.coords[0]
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.coords[0]
    }

    #[inline]
    pub fn y(&self) -> T {
        self.coords[1]
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.coords[1]
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        [
            self + Vector2D::new(T::zero(), -T::one()),
            self + Vector2D::new(-T::one(), T::zero()),
            self + Vector2D::new(T::one(), T::zero()),
            self + Vector2D::new(T::zero(), T::one()),
        ]
        .into_iter()
    }

    pub fn neighbours_diagonal(self) -> impl Iterator<Item = Self> {
        [
            self + Vector2D::new(-T::one(), -T::one()),
            self + Vector2D::new(T::zero(), -T::one()),
            self + Vector2D::new(T::one(), -T::one()),
            self + Vector2D::new(-T::one(), T::zero()),
            self + Vector2D::new(T::one(), T::zero()),
            self + Vector2D::new(-T::one(), T::one()),
            self + Vector2D::new(T::zero(), T::one()),
            self + Vector2D::new(T::one(), T::one()),
        ]
        .into_iter()
    }
}

pub type Vector3D<T = i32> = Vector<3, T>;

#[allow(dead_code)]
impl<T: Num> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { coords: [x, y, z] }
    }

    #[inline]
    pub fn x(&self) -> T {
        self.coords[0]
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.coords[0]
    }

    #[inline]
    pub fn y(&self) -> T {
        self.coords[1]
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.coords[1]
    }

    #[inline]
    pub fn z(&self) -> T {
        self.coords[2]
    }

    #[inline]
    pub fn z_mut(&mut self) -> &mut T {
        &mut self.coords[2]
    }

    pub fn cross_product(self, other: Self) -> Self {
        // https://en.wikipedia.org/wiki/Cross_product
        Vector3D::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        [
            self + Vector3D::new(-T::one(), T::zero(), T::zero()),
            self + Vector3D::new(T::one(), T::zero(), T::zero()),
            self + Vector3D::new(T::zero(), -T::one(), T::zero()),
            self + Vector3D::new(T::zero(), T::one(), T::zero()),
            self + Vector3D::new(T::zero(), T::zero(), -T::one()),
            self + Vector3D::new(T::zero(), T::zero(), T::one()),
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vector2d {
        use super::*;

        #[test]
        fn test_new() {
            assert_eq!(Vector2D::new(3, 4), Vector2D::from([3, 4]));
        }

        #[test]
        fn test_abs() {
            assert_eq!(Vector2D::new(3, -4).abs(), Vector2D::from([3, 4]));
        }

        #[test]
        fn test_add() {
            assert_eq!(
                Vector2D::new(3, 4) + Vector2D::new(5, 10),
                Vector2D::from([8, 14])
            );
            assert_eq!(
                Vector2D::new(3, 4) + Vector2D::zero(),
                Vector2D::from([3, 4])
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                Vector2D::new(3, 4) - Vector2D::new(5, 10),
                Vector2D::from([-2, -6])
            );
            assert_eq!(
                Vector2D::new(3, 4) - Vector2D::zero(),
                Vector2D::from([3, 4])
            );
        }

        #[test]
        fn test_neg() {
            assert_eq!(-Vector2D::new(3, 4), Vector2D::from([-3, -4]));
        }

        #[test]
        fn test_add_assign() {
            let mut vector = Vector2D::new(3, 4);
            vector += Vector2D::new(10, 20);
            assert_eq!(vector, Vector2D::from([13, 24]));
        }

        #[test]
        fn test_sub_assign() {
            let mut vector = Vector2D::new(3, 4);
            vector -= Vector2D::new(10, 20);
            assert_eq!(vector, Vector2D::from([-7, -16]));
        }

        #[test]
        fn test_mul() {
            assert_eq!(Vector2D::new(3, 4) * 2, Vector2D::from([6, 8]));
        }

        #[test]
        fn test_mul_assign() {
            let mut vector = Vector2D::new(3, 4);
            vector *= 3;
            assert_eq!(vector, Vector2D::from([9, 12]));
        }

        #[test]
        fn test_display() {
            assert_eq!(format!("{}", Vector2D::new(3, 4)), "(3, 4)");
        }
    }

    mod vector3d {
        use super::*;

        #[test]
        fn test_new() {
            assert_eq!(Vector3D::new(3, 4, 5), Vector3D::from([3, 4, 5]));
        }

        #[test]
        fn test_abs() {
            assert_eq!(Vector3D::new(3, -4, -5).abs(), Vector3D::from([3, 4, 5]));
        }

        #[test]
        fn test_add() {
            assert_eq!(
                Vector3D::new(3, 4, 5) + Vector3D::new(5, 10, 15),
                Vector3D::from([8, 14, 20])
            );
            assert_eq!(
                Vector3D::new(3, 4, 5) + Vector3D::zero(),
                Vector3D::from([3, 4, 5])
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                Vector3D::new(3, 4, 5) - Vector3D::new(5, 10, 15),
                Vector3D::from([-2, -6, -10])
            );
            assert_eq!(
                Vector3D::new(3, 4, 5) - Vector3D::zero(),
                Vector3D::from([3, 4, 5])
            );
        }

        #[test]
        fn test_add_assign() {
            let mut vector = Vector3D::new(3, 4, 5);
            vector += Vector3D::new(10, 20, 30);
            assert_eq!(vector, Vector3D::from([13, 24, 35]));
        }

        #[test]
        fn test_sub_assign() {
            let mut vector = Vector3D::new(3, 4, 5);
            vector -= Vector3D::new(10, 20, 30);
            assert_eq!(vector, Vector3D::from([-7, -16, -25]));
        }

        #[test]
        fn test_mul() {
            assert_eq!(Vector3D::new(3, 4, 5) * 2, Vector3D::from([6, 8, 10]));
        }

        #[test]
        fn test_mul_assign() {
            let mut vector = Vector3D::new(3, 4, 5);
            vector *= 3;
            assert_eq!(vector, Vector3D::from([9, 12, 15]));
        }

        #[test]
        fn test_display() {
            assert_eq!(format!("{}", Vector3D::new(3, 4, 5)), "(3, 4, 5)");
        }

        #[test]
        fn test_debug() {
            assert_eq!(
                format!("{:?}", Vector3D::new(3, 4, 5)),
                "Vector<3>(3, 4, 5)"
            );
        }
    }
}
