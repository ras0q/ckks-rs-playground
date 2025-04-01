use ciphertext::Ciphertext;
use keys::{EvaluationKey, PublicKey, SecretKey};
use plaintext::Plaintext;
use poly::Polynomial;

pub mod ciphertext;
pub mod keys;
pub mod plaintext;
pub mod poly;

pub fn generate_keys<const N: usize>(
    limit: u32,
    p: i64,
    q0: i64,
    scale: i64,
) -> (PublicKey<N>, SecretKey<N>, EvaluationKey<N>) {
    let ql = (p.pow(limit)) * q0;
    let secret_key = SecretKey::generate(ql);
    let public_key = PublicKey::generate(secret_key, ql);
    let evaluation_key = EvaluationKey::generate(secret_key, ql, scale);

    (public_key, secret_key, evaluation_key)
}

pub fn encrypt<const N: usize>(
    plaintext: Plaintext<N>,
    public_key: PublicKey<N>,
    evaluation_key: EvaluationKey<N>,
) -> Ciphertext<N> {
    let modulo = public_key.b.modulo;
    let v = Polynomial::new_random(-1..2, modulo);
    let e0 = Polynomial::new_random(-3..3, modulo);
    let e1 = Polynomial::new_random(-3..3, modulo);

    let c0 = v * public_key.b + plaintext.m + e0;
    let c1 = v * public_key.a + e1;

    Ciphertext::new(c0, c1, evaluation_key, plaintext.scale)
}

pub fn decrypt<const N: usize>(
    ciphertext: Ciphertext<N>,
    secret_key: SecretKey<N>,
) -> Plaintext<N> {
    let m = ciphertext.c0 + ciphertext.c1 * secret_key.s;
    Plaintext::new(m, ciphertext.scale)
}
