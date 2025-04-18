use std::ops::Neg;

use num_integer::Integer;
use rand::distr::uniform::SampleUniform;

use super::poly::ModPoly;

#[derive(Debug, Clone, Copy)]
pub struct SecretKey<T: Integer, const N: usize> {
    pub s: ModPoly<T, N>,
}

impl<T: Integer, const N: usize> SecretKey<T, N>
where
    T: Default + Copy + SampleUniform + From<i64> + Neg,
{
    pub fn generate(modulo: T) -> Self {
        let s = ModPoly::<T, N>::new_random((-1).into()..2.into(), modulo);
        Self { s }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PublicKey<T: Integer, const N: usize> {
    pub b: ModPoly<T, N>,
    pub a: ModPoly<T, N>,
}

impl<T: Integer, const N: usize> PublicKey<T, N>
where
    T: Default + Copy + SampleUniform + From<i64> + Neg<Output = T>,
{
    pub fn generate(secret_key: SecretKey<T, N>, modulo: T) -> Self {
        let a = ModPoly::<T, N>::new_random((-100).into()..100.into(), modulo);
        let e = ModPoly::<T, N>::new_random((-3).into()..3.into(), modulo);
        let b = -a * secret_key.s + e;
        Self { b, a }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EvaluationKey<T: Integer, const N: usize> {
    pub b: ModPoly<T, N>,
    pub a: ModPoly<T, N>,
}

impl<T: Integer, const N: usize> EvaluationKey<T, N>
where
    T: Default + Copy + SampleUniform + From<i64> + Neg<Output = T>,
{
    pub fn generate(secret_key: SecretKey<T, N>, modulo: T, scale: T) -> Self {
        let modulo_scaled = modulo * scale;

        let s = ModPoly {
            modulo: modulo_scaled,
            ..secret_key.s
        };
        let a = ModPoly::<T, N>::new_random((-100).into()..100.into(), modulo_scaled);
        let e = ModPoly::<T, N>::new_random((-3).into()..3.into(), modulo_scaled);
        let b = -a * s + e + (s * s) * scale;
        Self { b, a }
    }
}
