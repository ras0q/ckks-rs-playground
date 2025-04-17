use std::ops::{Add, Mul};

use super::poly::Poly;

#[derive(Debug, Clone, Copy)]
pub struct Plaintext<const N: usize> {
    pub m: Poly<i64, N>,
    pub scale: i64,
}

impl<const N: usize> Plaintext<N> {
    pub fn new(m: Poly<i64, N>, scale: i64) -> Self {
        Self { m, scale }
    }
}

impl<const N: usize> Add for Plaintext<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            m: self.m + rhs.m,
            ..self
        }
    }
}

impl<const N: usize> Mul for Plaintext<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            m: self.m * rhs.m / self.scale,
            ..self
        }
    }
}
