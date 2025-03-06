use std::ops::{Add, Mul, Neg, Rem, Sub};

#[derive(Debug, Clone, Copy)]
// P(X) = coeffs[0] + coeffs[1]*X + ... + coeffs[N-1]*X^(N-1)
pub struct Polynomial<T, const N: usize> {
    pub coeffs: [T; N],
    pub modulo: i64,
}

impl<T, const N: usize> Polynomial<T, N> {
    pub fn new(coeffs: [T; N], modulo: i64) -> Self {
        Self { coeffs, modulo }
    }

    // [x/2, x/2)の範囲に収める
    fn cmod(&self, x: T) -> T
    where
        T: Copy + From<i64> + PartialOrd + Sub<Output = T> + Rem<Output = T>,
    {
        let modulo = T::from(self.modulo);
        let t = x % modulo;

        if t < T::from(self.modulo / 2) {
            t
        } else {
            t - modulo
        }
    }
}

impl<T, const N: usize> Default for Polynomial<T, N>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            coeffs: core::array::from_fn(|_| T::default()),
            modulo: 1000000007,
        }
    }
}

impl<T, const N: usize> Neg for Polynomial<T, N>
where
    T: Copy
        + Default
        + From<i64>
        + PartialOrd
        + Sub<Output = T>
        + Rem<Output = T>
        + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let new_coeffs: [T; N] = self.coeffs.map(|c| self.cmod(-c));

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Add for Polynomial<T, N>
where
    T: Copy
        + Default
        + From<i64>
        + PartialOrd
        + Sub<Output = T>
        + Rem<Output = T>
        + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Polynomial<T, N>) -> Self::Output {
        let mut new_coeffs: [T; N] = [T::default(); N];
        for (i, (a, b)) in self.coeffs.iter().zip(rhs.coeffs.iter()).enumerate() {
            new_coeffs[i] = self.cmod(*a + *b);
        }

        Self::new(new_coeffs, self.modulo)
    }
}

impl<T, const N: usize> Mul for Polynomial<T, N>
where
    T: Copy
        + Default
        + From<i64>
        + PartialOrd
        + Sub<Output = T>
        + Rem<Output = T>
        + Add<Output = T>
        + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Polynomial<T, N>) -> Self::Output {
        let mut product = vec![T::default(); 2 * N];

        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                product[i + j] = self.cmod(product[i + j] + self.cmod(*a * *b));
            }
        }

        // X^N + 1で割ってN-1次式にする
        let mut new_coeffs: [T; N] = [T::default(); N];
        for i in 0..N {
            new_coeffs[i] = self.cmod(product[i] + product[i + N]);
        }

        Self::new(new_coeffs, self.modulo)
    }
}
