use ciphertext::Ciphertext;
use code::{canonical_embedding, canonical_embedding_inv, project, project_inv};
use keys::{EvaluationKey, PublicKey, SecretKey};
use num_complex::Complex64;
use plaintext::Plaintext;
use poly::{ModPoly, Poly};

pub mod ciphertext;
pub mod code;
pub mod keys;
pub mod modulo;
pub mod plaintext;
pub mod poly;

// ℂ^{N/2} -> ℤ[X]/(X^N + 1)
pub fn encode<const N: usize>(z: [Complex64; N / 2], delta: i64, scale: i64) -> Plaintext<N> {
    let encoded = canonical_embedding_inv(project_inv(z));
    // imが0のはず
    assert!(encoded.coeffs.iter().all(|x| x.im.abs() < 1e-6));

    let coeffs = encoded.coeffs.map(|x| (x.re * delta as f64).round() as i64);

    Plaintext::new(Poly::new(coeffs), scale)
}

// ℤ[X]/(X^N + 1) -> ℂ^{N/2}
pub fn decode<const N: usize>(plaintext: Plaintext<N>, delta: i64) -> [Complex64; N / 2] {
    let p = Poly::new(
        plaintext
            .m
            .coeffs
            .map(|x| Complex64::new(x as f64 / delta as f64, 0.0)),
    );

    project(canonical_embedding(p))
}

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

// ℤ[X]/(X^N + 1) -> ((ℤ/qℤ)[X]/(X^N + 1))^2
pub fn encrypt<const N: usize>(
    plaintext: Plaintext<N>,
    public_key: PublicKey<N>,
    evaluation_key: EvaluationKey<N>,
) -> Ciphertext<N> {
    let modulo = public_key.b.modulo;
    let m = ModPoly::new(plaintext.m.coeffs, modulo);
    let v = ModPoly::new_random(-1..2, modulo);
    let e0 = ModPoly::new_random(-3..3, modulo);
    let e1 = ModPoly::new_random(-3..3, modulo);

    let c0 = v * public_key.b + m + e0;
    let c1 = v * public_key.a + e1;

    Ciphertext::new(c0, c1, evaluation_key, plaintext.scale)
}

// ((ℤ/qℤ)[X]/(X^N + 1))^2 -> ℤ[X]/(X^N + 1)
pub fn decrypt<const N: usize>(
    ciphertext: Ciphertext<N>,
    secret_key: SecretKey<N>,
) -> Plaintext<N> {
    let m = ciphertext.c0 + ciphertext.c1 * secret_key.s;
    let m = Poly::new(m.coeffs);
    Plaintext::new(m, ciphertext.scale)
}
