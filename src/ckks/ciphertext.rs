use super::{keys::EvaluationKey, poly::ModPoly};
use num_integer::Integer;
use std::fmt::Debug;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Ciphertext<T: Integer, const N: usize> {
    pub c0: ModPoly<T, N>,
    pub c1: ModPoly<T, N>,
    pub evaluation_key: EvaluationKey<T, N>,
    pub scale: T,
}

impl<T: Integer, const N: usize> Ciphertext<T, N> {
    pub fn new(
        c0: ModPoly<T, N>,
        c1: ModPoly<T, N>,
        evaluation_key: EvaluationKey<T, N>,
        scale: T,
    ) -> Self {
        Self {
            c0,
            c1,
            evaluation_key,
            scale,
        }
    }
}

impl<T: Integer + Default + Copy + Debug, const N: usize> Add for Ciphertext<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.scale, rhs.scale);

        Self {
            c0: self.c0 + rhs.c0,
            c1: self.c1 + rhs.c1,
            ..self
        }
    }
}

impl<T: Integer + Default + Copy + Debug, const N: usize> Mul for Ciphertext<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // TODO: とりあえず
        assert_eq!(self.scale, rhs.scale);

        let c0 = self.c0 * rhs.c0;
        let c1 = self.c0 * rhs.c1 + self.c1 * rhs.c0;
        let c2 = self.c1 * rhs.c1;

        // calculate with evaluation key's modulo
        let EvaluationKey { b, a } = self.evaluation_key;
        let new_modulo = b.modulo;
        let d0 = c0.with_modulo(new_modulo) + (c2.with_modulo(new_modulo) * b / self.scale);
        let d1 = c1.with_modulo(new_modulo) + (c2.with_modulo(new_modulo) * a / self.scale);

        Self {
            c0: d0,
            c1: d1,
            evaluation_key: self.evaluation_key,
            scale: self.scale * self.scale,
        }
    }
}
