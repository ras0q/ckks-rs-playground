use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Range, Sub},
};

use num_integer::{Integer, gcd};
use num_traits::Num;

#[derive(Debug, Clone, Copy)]
// Polynomial expression on (ℤ/nℤ)[X]/(X^N + 1)
// P(X) = coeffs[0] + coeffs[1]*X + ... + coeffs[N-1]*X^(N-1)
pub struct Polynomial<T, const N: usize> {
    pub coeffs: [T; N],
    pub modulo: i64,
}

impl<T, const N: usize> Polynomial<T, N> {
    pub fn new(coeffs: [T; N], modulo: i64) -> Self {
        Self { coeffs, modulo }
    }

    pub fn new_random(range: Range<i64>, modulo: i64) -> Self
    where
        T: Copy + PartialOrd + Num + From<i64>,
    {
        let coeffs: [T; N] = std::array::from_fn(|_| T::from(rand::random_range(range.clone())));

        Self { coeffs, modulo }
    }

    // (-x/2, x/2]の範囲に収める
    fn cmod(&self, x: T) -> T
    where
        T: Copy + PartialOrd + Num + From<i64>,
    {
        let modulo = T::from(self.modulo);
        let t = x % modulo;

        if t < T::from(self.modulo / 2) {
            t
        } else {
            t - modulo
        }
    }

    fn is_in_range(&self, x: T) -> bool
    where
        T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T>,
    {
        let modulo = T::from(self.modulo);
        let half_modulo = modulo / T::from(2);

        x > -half_modulo && x <= half_modulo
    }
}

impl<T, const N: usize> Neg for Polynomial<T, N>
where
    T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| self.cmod(-c));

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Add for Polynomial<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn add(self, rhs: Polynomial<T, N>) -> Self::Output {
        let mut new_coeffs: [T; N] = [T::zero(); N];
        for (i, (a, b)) in self.coeffs.iter().zip(rhs.coeffs.iter()).enumerate() {
            new_coeffs[i] = self.cmod(*a + *b);
        }

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Sub for Polynomial<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn sub(self, rhs: Polynomial<T, N>) -> Self::Output {
        let mut new_coeffs: [T; N] = [T::zero(); N];
        for (i, (a, b)) in self.coeffs.iter().zip(rhs.coeffs.iter()).enumerate() {
            new_coeffs[i] = self.cmod(*a - *b);
        }

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Mul for Polynomial<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn mul(self, rhs: Polynomial<T, N>) -> Self::Output {
        let mut product = vec![T::zero(); 2 * N - 1];

        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                product[i + j] = self.cmod(product[i + j] + self.cmod(*a * *b));
            }
        }
        // 2(N-1)次式をN-1次式にする
        // X^N + 1で割ってN-1次式にする
        let mut new_coeffs: [T; N] = [T::zero(); N];
        for i in 0..(N - 1) {
            new_coeffs[i] = self.cmod(product[i] - product[i + N]);
        }

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Mul<T> for Polynomial<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| self.cmod(c * rhs));

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Div<T> for Polynomial<T, N>
where
    T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T> + Integer + Debug,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        if rhs == T::one() {
            return self;
        }
        if rhs + T::one() == T::zero() {
            return -self;
        }

        if rhs == T::zero() {
            panic!("Division by zero");
        }
        if !self.is_in_range(rhs) {
            panic!("Division out of range");
        }
        if gcd(rhs, T::from(self.modulo)) != T::one() {
            panic!("Division not coprime");
        }

        let modulo = T::from(self.modulo);
        let rhs_inv = {
            let mut t = T::one();
            while (t * rhs) % modulo != T::one() {
                t = t + T::one();
            }
            t
        };
        println!("rhs_inv: {:?}", rhs_inv);

        let new_coeffs: [T; N] = self.coeffs.map(|c| self.cmod(c * rhs_inv));

        Self::new(new_coeffs, self.modulo)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cmod() {
        use super::*;

        let poly = Polynomial::<i64, 1>::new([0], 5);
        assert_eq!(poly.cmod(0), 0);
        assert_eq!(poly.cmod(1), 1);
        assert_eq!(poly.cmod(2), -3);
        assert_eq!(poly.cmod(3), -2);
        assert_eq!(poly.cmod(4), -1);
        assert_eq!(poly.cmod(5), 0);
    }

    #[test]
    fn neg() {
        use super::*;

        let poly = Polynomial::<i64, 4>::new([1, 2, 3, 4], 5);
        let neg_poly = -poly;
        assert_eq!(neg_poly.coeffs, [-1, -2, -3, -4]);
    }

    #[test]
    fn add() {
        use super::*;

        let poly1 = Polynomial::<i64, 4>::new([1, 2, 3, 4], 5);
        let poly2 = Polynomial::<i64, 4>::new([4, 3, 2, 1], 5);
        let sum_poly = poly1 + poly2;
        assert_eq!(sum_poly.coeffs, [0, 0, 0, 0]);
    }

    #[test]
    fn sub() {
        use super::*;

        let poly1 = Polynomial::<i64, 4>::new([1, 2, 3, 4], 5);
        let poly2 = Polynomial::<i64, 4>::new([4, 3, 2, 1], 5);
        let diff_poly = poly1 - poly2;
        assert_eq!(diff_poly.coeffs, [-3, -1, 1, -2]);
    }

    #[test]
    fn mul() {
        use super::*;

        let poly1 = Polynomial::<i64, 4>::new([1, 2, 3, 4], 5);
        let poly2 = Polynomial::<i64, 4>::new([4, 3, 2, 1], 5);
        let prod_poly = poly1 * poly2;
        assert_eq!(prod_poly.coeffs, [-1, 0, 1, 0]);
    }

    #[test]
    fn mul_scalar() {
        use super::*;

        let poly = Polynomial::<i64, 4>::new([1, 2, 3, 4], 5);
        let scalar = 2;
        let prod_poly = poly * scalar;
        assert_eq!(prod_poly.coeffs, [-3, -1, 1, -2]);
    }

    #[test]
    fn div_scalar() {
        use super::*;

        let poly = Polynomial::<i64, 4>::new([1, 2, 3, 4], 5);
        let scalar = 2;
        let div_poly = poly / scalar;
        assert_eq!(div_poly.coeffs, [-2, 1, -1, -3]);
    }
}
