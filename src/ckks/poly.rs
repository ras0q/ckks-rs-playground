use super::modulo::cmod;
use num_integer::Integer;
use num_traits::Num;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Range, Sub},
};

#[derive(Debug, Clone, Copy)]
// Polynomial expression on (ℤ/nℤ)[X]/(X^N + 1)
// P(X) = coeffs[0] + coeffs[1]*X + ... + coeffs[N-1]*X^(N-1)
pub struct Poly<T, const N: usize> {
    pub coeffs: [T; N],
}

impl<T, const N: usize> Poly<T, N> {
    pub fn new(coeffs: [T; N]) -> Self {
        Self { coeffs }
    }

    pub fn new_random(range: Range<i64>) -> Self
    where
        T: Copy + PartialOrd + Num + From<i64>,
    {
        let coeffs: [T; N] = std::array::from_fn(|_| T::from(rand::random_range(range.clone())));

        Self { coeffs }
    }
}

impl<T, const N: usize> Neg for Poly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| -c);

        Self::new(new_coeffs)
    }
}

impl<T, const N: usize> Add for Poly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn add(self, rhs: Poly<T, N>) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| c + rhs.coeffs[0]);

        Self::new(new_coeffs)
    }
}

impl<T, const N: usize> Sub for Poly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn sub(self, rhs: Poly<T, N>) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| c - rhs.coeffs[0]);

        Self::new(new_coeffs)
    }
}

impl<T, const N: usize> Mul for Poly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn mul(self, rhs: Poly<T, N>) -> Self::Output {
        let mut product = vec![T::zero(); 2 * N - 1];

        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                product[i + j] = product[i + j] + (*a * *b);
            }
        }
        let mut new_coeffs: [T; N] = [T::zero(); N];
        for i in 0..(N - 1) {
            new_coeffs[i] = product[i] - product[i + N];
        }

        Self::new(new_coeffs)
    }
}

impl<T, const N: usize> Mul<T> for Poly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| c * rhs);

        Self::new(new_coeffs)
    }
}

impl<T, const N: usize> Div<T> for Poly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| c / rhs);

        Self::new(new_coeffs)
    }
}

#[derive(Debug, Clone, Copy)]
// Polynomial expression on (ℤ/nℤ)[X]/(X^N + 1) with modulo
// P(X) = coeffs[0] + coeffs[1]*X + ... + coeffs[N-1]*X^(N-1)
pub struct ModPoly<T: Integer, const N: usize> {
    pub coeffs: [T; N],
    pub modulo: i64,
}

impl<T: Integer, const N: usize> ModPoly<T, N> {
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

    pub fn with_modulo(self, modulo: i64) -> Self {
        Self {
            coeffs: self.coeffs,
            modulo,
        }
    }
}

impl<T: Integer, const N: usize> Neg for ModPoly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| cmod(-c, self.modulo));

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T: Integer, const N: usize> Add for ModPoly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn add(self, rhs: ModPoly<T, N>) -> Self::Output {
        let mut new_coeffs: [T; N] = [T::zero(); N];
        for (i, (a, b)) in self.coeffs.iter().zip(rhs.coeffs.iter()).enumerate() {
            new_coeffs[i] = cmod(*a + *b, self.modulo);
        }

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T: Integer, const N: usize> Sub for ModPoly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn sub(self, rhs: ModPoly<T, N>) -> Self::Output {
        let mut new_coeffs: [T; N] = [T::zero(); N];
        for (i, (a, b)) in self.coeffs.iter().zip(rhs.coeffs.iter()).enumerate() {
            new_coeffs[i] = cmod(*a - *b, self.modulo);
        }

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T: Integer, const N: usize> Mul for ModPoly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn mul(self, rhs: ModPoly<T, N>) -> Self::Output {
        let mut product = vec![T::zero(); 2 * N - 1];

        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                product[i + j] = cmod(product[i + j] + cmod(*a * *b, self.modulo), self.modulo);
            }
        }
        let mut new_coeffs: [T; N] = [T::zero(); N];
        for i in 0..(N - 1) {
            new_coeffs[i] = cmod(product[i] - product[i + N], self.modulo);
        }

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T: Integer, const N: usize> Mul<T> for ModPoly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| cmod(c * rhs, self.modulo));

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Div<T> for ModPoly<T, N>
where
    T: Copy + PartialOrd + Num + From<i64> + Neg<Output = T> + Integer + Debug,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        // TODO: 少数に直す
        let new_coeffs: [T; N] = self.coeffs.map(|c| cmod(c / rhs, self.modulo));

        Self::new(new_coeffs, self.modulo)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn neg() {
        use super::*;

        let poly = ModPoly::<i64, 4>::new([1, 2, 3, 4], 5);
        let neg_poly = -poly;
        assert_eq!(neg_poly.coeffs, [-1, -2, -3, -4]);
    }

    #[test]
    fn add() {
        use super::*;

        let poly1 = ModPoly::<i64, 4>::new([1, 2, 3, 4], 5);
        let poly2 = ModPoly::<i64, 4>::new([4, 3, 2, 1], 5);
        let sum_poly = poly1 + poly2;
        assert_eq!(sum_poly.coeffs, [0, 0, 0, 0]);
    }

    #[test]
    fn sub() {
        use super::*;

        let poly1 = ModPoly::<i64, 4>::new([1, 2, 3, 4], 5);
        let poly2 = ModPoly::<i64, 4>::new([4, 3, 2, 1], 5);
        let diff_poly = poly1 - poly2;
        assert_eq!(diff_poly.coeffs, [-3, -1, 1, -2]);
    }

    #[test]
    fn mul() {
        use super::*;

        let poly1 = ModPoly::<i64, 4>::new([1, 2, 3, 4], 5);
        let poly2 = ModPoly::<i64, 4>::new([4, 3, 2, 1], 5);
        let prod_poly = poly1 * poly2;
        assert_eq!(prod_poly.coeffs, [-1, 0, 1, 0]);
    }

    #[test]
    fn mul_scalar() {
        use super::*;

        let poly = ModPoly::<i64, 4>::new([1, 2, 3, 4], 5);
        let scalar = 2;
        let prod_poly = poly * scalar;
        assert_eq!(prod_poly.coeffs, [-3, -1, 1, -2]);
    }
}
