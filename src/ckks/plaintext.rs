use std::ops::{Add, Mul};

use num_integer::Integer;

use super::poly::Poly;

#[derive(Debug, Clone, Copy)]
pub struct Plaintext<T: Integer, const N: usize> {
    pub m: Poly<T, N>,
    pub scale: T,
}

impl<T: Integer, const N: usize> Plaintext<T, N> {
    pub fn new(m: Poly<T, N>, scale: T) -> Self {
        Self { m, scale }
    }
}

impl<T: Integer + Copy + Default, const N: usize> Add for Plaintext<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            m: self.m + rhs.m,
            scale: self.scale,
        }
    }
}

impl<T: Integer + Copy + Default, const N: usize> Mul for Plaintext<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            m: self.m * rhs.m,
            scale: self.scale * rhs.scale,
        }
    }
}
