use super::{ciphertext::Ciphertext, plaintext::Plaintext, poly::Polynomial, random::rand_array};

pub struct KeyGenerator<const N: usize> {
    // limit: i64,
    // p: i64,
    // q0: i64,
    ql: i64,
    scale: i64,
}

impl<const N: usize> KeyGenerator<N> {
    pub fn new(limit: u32, p: i64, q0: i64, scale: i64) -> Self {
        Self {
            ql: (p.pow(limit)) * q0,
            scale,
        }
    }

    pub fn generate_keys(&self) -> (PublicKey<N>, SecretKey<N>, EveluationKey<N>) {
        let secret_key = SecretKey::generate(self.ql);
        let public_key = PublicKey::generate(secret_key, self.ql);
        let evaluation_key = EveluationKey::generate(secret_key, self.ql, self.scale);

        (public_key, secret_key, evaluation_key)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SecretKey<const N: usize> {
    pub s: Polynomial<i64, N>,
}

impl<const N: usize> SecretKey<N> {
    pub fn generate(modulo: i64) -> Self {
        let s = Polynomial::<i64, N>::new(rand_array(-1..2), modulo);
        Self { s }
    }

    pub fn decrypt(&self, ciphertext: Ciphertext<N>) -> Plaintext<N> {
        let m = ciphertext.c0 + ciphertext.c1 * self.s;
        Plaintext::new(m)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PublicKey<const N: usize> {
    pub b: Polynomial<i64, N>,
    pub a: Polynomial<i64, N>,
}

impl<const N: usize> PublicKey<N> {
    pub fn generate(secret_key: SecretKey<N>, modulo: i64) -> Self {
        let a = Polynomial::<i64, N>::new(rand_array(-100..100), modulo);
        let e = Polynomial::<i64, N>::new(rand_array(-3..3), modulo);
        let b = -a * secret_key.s + e;
        Self { b, a }
    }

    pub fn encrypt(
        &self,
        plaintext: Plaintext<N>,
        evaluation_key: EveluationKey<N>,
        scale: i64,
    ) -> Ciphertext<N> {
        let modulo = self.b.modulo;
        let v = Polynomial::new(rand_array(-1..2), modulo);
        let e0 = Polynomial::new(rand_array(-3..3), modulo);
        let e1 = Polynomial::new(rand_array(-3..3), modulo);

        let c0 = v * self.b + plaintext.m + e0;
        let c1 = v * self.a + e1;

        Ciphertext {
            c0,
            c1,
            evaluation_key,
            scale,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EveluationKey<const N: usize> {
    pub b: Polynomial<i64, N>,
    pub a: Polynomial<i64, N>,
}

impl<const N: usize> EveluationKey<N> {
    pub fn generate(secret_key: SecretKey<N>, modulo: i64, scale: i64) -> Self {
        let modulo_scaled = modulo * scale;

        let s = Polynomial {
            modulo: modulo_scaled,
            ..secret_key.s
        };
        let a = Polynomial::<i64, N>::new(rand_array(-100..100), modulo_scaled);
        let e = Polynomial::<i64, N>::new(rand_array(-3..3), modulo_scaled);
        let b = -a * s + e + (s * s) * scale;
        Self { b, a }
    }
}
