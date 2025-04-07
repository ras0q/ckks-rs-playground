use std::ops::Neg;

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
