use super::poly::ModPoly;
use num_complex::Complex64;
use std::f64::consts::PI;

// σ: ℂ[X] -> ℂ^N
// σ(P) = [P(ξ), P(ξ^3), ..., P(ξ^{2N-1})]
pub fn canonical_embedding<T, const N: usize>(p: ModPoly<T, N>) -> [num_complex::Complex64; N]
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
pub fn canonical_embedding_inv<const N: usize>(
    z: [num_complex::Complex64; N],
) -> ModPoly<num_complex::Complex64, N> {
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

    ModPoly::new(coeffs, i64::MAX)
}

// 前半半分を取り出す
pub fn project<const N: usize>(z: [num_complex::Complex64; N]) -> [num_complex::Complex64; N / 2] {
    z.iter()
        .take(N / 2)
        .cloned()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

// 前半部分から元の値に戻す
// [z_1, z_2, ..., z_N/2] -> [z_1, z_2, ..., z_N/2, conj(z_N/2), conj(z_N/2-1), ..., conj(z_1)]
pub fn project_inv<const N: usize>(
    first: [num_complex::Complex64; N / 2],
) -> [num_complex::Complex64; N] {
    let mut second = first.map(|x| x.conj());
    second.reverse();

    [first, second].concat().try_into().unwrap()
}
