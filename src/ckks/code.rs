use super::poly::Poly;
use num_complex::Complex64;
use std::f64::consts::PI;

// σ: ℂ[X] -> ℂ^N
// σ(P) = [P(ξ), P(ξ^3), ..., P(ξ^{2N-1})]
pub fn canonical_embedding<const N: usize>(p: Poly<Complex64, N>) -> [Complex64; N] {
    // ξ = e^(2πi/(2*N))
    let xi = PI / N as f64;

    let mut result = [Complex64::default(); N];
    for (i, res) in result.iter_mut().enumerate() {
        let theta = xi * (2.0 * (i as f64) + 1.0);
        let x = Complex64::from_polar(1.0, theta);
        *res = p.evaluate(x);
    }

    result
}

// σ^{-1}: ℂ^N -> ℂ[X]
pub fn canonical_embedding_inv<const N: usize>(z: [Complex64; N]) -> Poly<Complex64, N> {
    // ξ = e^(2πi/(2*N))
    let xi = PI / N as f64;

    let coeffs: [Complex64; N] = (0..N)
        .map(|i| {
            let sum: Complex64 = z
                .iter()
                .enumerate()
                .map(|(j, &zj)| {
                    let theta = (i as f64) * xi * (2.0 * (j as f64) + 1.0);
                    zj * Complex64::from_polar(1.0, theta).conj()
                })
                .sum();

            sum / N as f64
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Poly::new(coeffs)
}

// 前半半分を取り出す
pub fn project<const N: usize>(z: [Complex64; N]) -> [Complex64; N / 2] {
    z.iter()
        .take(N / 2)
        .cloned()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

// 前半部分から元の値に戻す
// [z_1, z_2, ..., z_N/2] -> [z_1, z_2, ..., z_N/2, conj(z_N/2), conj(z_N/2-1), ..., conj(z_1)]
pub fn project_inv<const N: usize>(first: [Complex64; N / 2]) -> [Complex64; N] {
    let mut second = first.map(|x| x.conj());
    second.reverse();

    [first, second].concat().try_into().unwrap()
}
