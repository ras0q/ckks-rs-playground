use std::ops::{Add, Mul};

use super::{modulo::inv, poly::ModPoly};

#[derive(Debug, Clone, Copy)]
pub struct Plaintext<const N: usize> {
    pub m: ModPoly<i64, N>,
    pub scale: i64,
    scale_inv: i64,
}

impl<const N: usize> Plaintext<N> {
    pub fn new(m: ModPoly<i64, N>, scale: i64) -> Self {
        let scale_inv = inv(scale, m.modulo);
        Self {
            m,
            scale,
            scale_inv,
        }
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
            m: self.m * rhs.m * self.scale_inv,
            ..self
        }
    }
}
