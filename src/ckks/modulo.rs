use std::ops::Neg;

use num_integer::{Integer, gcd};
use num_traits::Num;

// Remainder in range (-modulo/2, modulo/2]
pub fn cmod<T>(x: T, modulo: i64) -> T
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    let modulo = T::from(modulo);
    let t = x % modulo;

    if t < modulo / T::from(2) {
        t
    } else {
        t - modulo
    }
}

pub fn is_in_range<T>(x: T, modulo: i64) -> bool
where
    T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T>,
{
    let half_modulo = T::from(modulo) / T::from(2);

    x > -half_modulo && x <= half_modulo
}

pub fn inv<T>(x: T, modulo: i64) -> T
where
    T: Copy + Num + From<i64> + PartialOrd + Neg<Output = T> + Integer,
{
    if x == T::zero() {
        panic!("Cannot compute inverse of zero");
    }
    if x == T::one() {
        return T::one();
    }
    if x + T::one() == T::zero() {
        return -T::one();
    }
    if modulo == i64::MAX {
        // TODO: Handle this case
        println!("WARNING: Inverse modulo is i64::MAX");
        return T::zero();
    }
    if !is_in_range(x, modulo) {
        panic!("Inverse out of range");
    }
    if gcd(x, T::from(modulo)) != T::one() {
        panic!("Inverse not coprime");
    }

    let half_modulo = T::from(modulo) / T::from(2);
    let mut t = -half_modulo;
    while t <= half_modulo {
        if (t * x) % modulo.into() == T::one() {
            return t;
        }
        t = t + T::one();
    }
    panic!("No modular inverse found in the range (-modulo/2, modulo/2]");
}
