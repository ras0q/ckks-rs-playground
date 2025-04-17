use super::poly::ModPoly;

#[derive(Debug, Clone, Copy)]
pub struct SecretKey<const N: usize> {
    pub s: ModPoly<i64, N>,
}

impl<const N: usize> SecretKey<N> {
    pub fn generate(modulo: i64) -> Self {
        let s = ModPoly::<i64, N>::new_random(-1..2, modulo);
        Self { s }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PublicKey<const N: usize> {
    pub b: ModPoly<i64, N>,
    pub a: ModPoly<i64, N>,
}

impl<const N: usize> PublicKey<N> {
    pub fn generate(secret_key: SecretKey<N>, modulo: i64) -> Self {
        let a = ModPoly::<i64, N>::new_random(-100..100, modulo);
        let e = ModPoly::<i64, N>::new_random(-3..3, modulo);
        let b = -a * secret_key.s + e;
        Self { b, a }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EvaluationKey<const N: usize> {
    pub b: ModPoly<i64, N>,
    pub a: ModPoly<i64, N>,
}

impl<const N: usize> EvaluationKey<N> {
    pub fn generate(secret_key: SecretKey<N>, modulo: i64, scale: i64) -> Self {
        let modulo_scaled = modulo * scale;

        let s = ModPoly {
            modulo: modulo_scaled,
            ..secret_key.s
        };
        let a = ModPoly::<i64, N>::new_random(-100..100, modulo_scaled);
        let e = ModPoly::<i64, N>::new_random(-3..3, modulo_scaled);
        let b = -a * s + e + (s * s) * scale;
        Self { b, a }
    }
}
