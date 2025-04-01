use std::ops::{Add, Div, Mul, Neg, Range, Sub};

use num_traits::Num;

#[derive(Debug, Clone, Copy)]
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

    // [x/2, x/2)の範囲に収める
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
        // 2N-1次式をN-1次式にする
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
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| self.cmod(c / rhs));

        Self::new(new_coeffs, self.modulo)
    }
}
