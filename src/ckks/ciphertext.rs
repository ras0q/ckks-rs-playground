use std::ops::{Add, Mul};

use super::{keys::EveluationKey, poly::Polynomial};

#[derive(Debug, Clone, Copy)]
pub struct Plaintext<const N: usize> {
    pub m: Polynomial<i64, N>,
}

impl<const N: usize> Add for Plaintext<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { m: self.m + rhs.m }
    }
}

impl<const N: usize> Mul for Plaintext<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self { m: self.m * rhs.m }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ciphertext<const N: usize> {
    pub c0: Polynomial<i64, N>,
    pub c1: Polynomial<i64, N>,
    pub evaluation_key: EveluationKey<N>,
    pub scale: i64,
}

impl<const N: usize> Add for Ciphertext<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            c0: self.c0 + rhs.c0,
            c1: self.c1 + rhs.c1,
            ..self
        }
    }
}

impl<const N: usize> Mul for Ciphertext<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let c0 = self.c0 * rhs.c0;
        let c1 = self.c0 * rhs.c1 + self.c1 * rhs.c0;
        let c2 = self.c1 * rhs.c1;

        let d0 = c0 + (c2 * self.evaluation_key.b * (1 / self.scale));
        let d1 = c1 + (c2 * self.evaluation_key.a * (1 / self.scale));

        Self {
            c0: d0,
            c1: d1,
            ..self
        }
    }
}
