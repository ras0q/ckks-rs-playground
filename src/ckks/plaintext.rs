use std::{
    f64::consts::PI,
    ops::{Add, Mul},
};

use num_complex::Complex64;

use super::poly::Polynomial;

#[derive(Debug, Clone, Copy)]
pub struct Plaintext<const N: usize> {
    pub m: Polynomial<i64, N>,
    pub delta: i64,
}

impl<const N: usize> Plaintext<N> {
    pub fn new(m: Polynomial<i64, N>) -> Self {
        Self { m, delta: 1 }
    }

    pub fn encode_from(z: [Complex64; N], delta: i64) -> Self {
        let encoded = canonical_embedding_inv(project_inv(z));
        // imが0のはず
        assert!(encoded.coeffs.iter().all(|x| x.im.abs() < 1e-6));

        let coeffs = encoded.coeffs.map(|x| (x.re * delta as f64).round() as i64);

        Self {
            m: Polynomial::new(coeffs, encoded.modulo),
            delta,
        }
    }

    pub fn decode_to(&self) -> [Complex64; N] {
        let p: Polynomial<f64, N> = Polynomial::new(self.m.coeffs.map(|x| x as f64), self.m.modulo);

        project(canonical_embedding(p).map(|x| x / self.delta as f64))
    }
}

// σ: ℂ[X] -> ℂ^N
// σ(P) = [P(ξ), P(ξ^3), ..., P(ξ^{2N-1})]
fn canonical_embedding<T, const N: usize>(p: Polynomial<T, N>) -> [num_complex::Complex64; N]
where
    T: Copy + Into<f64> + std::fmt::Debug,
{
    // ξ = e^(2πi/(2*N))
    let theta = PI / N as f64;

    (0..N)
        .map(|i| {
            let theta = theta * (2.0 * (i as f64) + 1.0) % (2.0 * PI);
            let x = num_complex::Complex::from_polar(1.0, theta);
            p.coeffs
                .iter()
                .enumerate()
                .map(|(j, &coeff)| coeff.into() * x.powu(j as u32))
                .sum()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

// σ^{-1}: ℂ^N -> ℂ[X]
fn canonical_embedding_inv<const N: usize>(
    z: [num_complex::Complex64; N],
) -> Polynomial<num_complex::Complex64, N> {
    // ξ = e^(2πi/(2*N))
    let theta = PI / N as f64;

    let coeffs: [num_complex::Complex64; N] = (0..N)
        .map(|i| {
            // let theta = theta * (2.0 * ((i + 1) as f64) - 1.0) % (2.0 * PI);

            let sum: Complex64 = z
                .iter()
                .enumerate()
                .map(|(j, &zj)| {
                    zj * num_complex::Complex::from_polar(
                        1.0,
                        theta * (2.0 * (j as f64) + 1.0) % (2.0 * PI),
                    )
                    .powu(i as u32)
                    .conj()
                })
                .sum();

            sum / N as f64
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Polynomial::new(coeffs, i64::MAX)
}

// 前半半分を取り出す
// 配列の長さは保持し、後半部分は0で埋める
fn project<const N: usize>(z: [num_complex::Complex64; N]) -> [num_complex::Complex64; N] {
    z.iter()
        .enumerate()
        .map(|(i, &x)| if i < N / 2 { x } else { 0.0.into() })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

// 前半部分から元の値に戻す
fn project_inv<const N: usize>(first: [num_complex::Complex64; N]) -> [num_complex::Complex64; N] {
    let mut second = first.iter().map(|x| x.conj()).collect::<Vec<_>>();
    second.reverse();

    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a + b)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

impl<const N: usize> Add for Plaintext<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            m: self.m + rhs.m,
            delta: self.delta,
        }
    }
}

impl<const N: usize> Mul for Plaintext<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            m: self.m * rhs.m,
            delta: self.delta * rhs.delta,
        }
    }
}
