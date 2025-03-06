use crate::{poly::Polynomial, random::rand_array};

pub struct KeyGenerator<const N: usize> {
    // limit: i64,
    // p: i64,
    // q0: i64,
    ql: i64,
}

impl<const N: usize> KeyGenerator<N> {
    pub fn new(limit: i64, p: i64, q0: i64) -> Self {
        Self {
            ql: (p ^ limit) * q0,
        }
    }

    pub fn generate_keys(&self) -> (PublicKey<N>, SecretKey<N>) {
        type T = i64;

        let s = Polynomial::<T, N>::new(rand_array(-1..2), self.ql);

        let a = Polynomial::<T, N>::new(rand_array(-100..100), self.ql);
        let e = Polynomial::<T, N>::new(rand_array(-3..3), self.ql);
        let b = -a * s + e;

        (PublicKey { b, a }, SecretKey { s })
    }
}

#[derive(Debug, Clone)]
pub struct PublicKey<const N: usize> {
    pub b: Polynomial<i64, N>,
    pub a: Polynomial<i64, N>,
}

impl<const N: usize> PublicKey<N> {
    pub fn encrypt(&self, plaintext: Plaintext<N>) -> Ciphertext<N> {
        let modulo = self.b.modulo;
        let v = Polynomial::new(rand_array(-1..2), modulo);
        let e0 = Polynomial::new(rand_array(-3..3), modulo);
        let e1 = Polynomial::new(rand_array(-3..3), modulo);

        let c0 = v * self.b + plaintext.m + e0;
        let c1 = v * self.a + e1;

        Ciphertext { c0, c1 }
    }
}

#[derive(Debug, Clone)]
pub struct SecretKey<const N: usize> {
    pub s: Polynomial<i64, N>,
}

impl<const N: usize> SecretKey<N> {
    pub fn decrypt(&self, ciphertext: Ciphertext<N>) -> Plaintext<N> {
        let m = ciphertext.c0 + ciphertext.c1 * self.s;
        Plaintext { m }
    }
}

#[derive(Debug, Clone)]
pub struct Plaintext<const N: usize> {
    pub m: Polynomial<i64, N>,
}

#[derive(Debug, Clone)]
pub struct Ciphertext<const N: usize> {
    pub c0: Polynomial<i64, N>,
    pub c1: Polynomial<i64, N>,
}
