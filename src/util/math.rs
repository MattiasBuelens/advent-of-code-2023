use super::num::Num;

#[allow(dead_code)]
pub fn gcd<T: Num>(mut a: T, mut b: T) -> T {
    while a != T::zero() {
        let old_a = a;
        a = b % a;
        b = old_a;
    }
    b.abs()
}

#[allow(dead_code)]
pub fn lcm<T: Num>(a: T, b: T) -> T {
    (a * b).abs() / gcd(a, b)
}
