use super::poly::Polynomial;

#[derive(Debug, Clone, Copy)]
pub struct Plaintext<const N: usize> {
    pub m: Polynomial<i64, N>,
}

#[derive(Debug, Clone, Copy)]
pub struct Ciphertext<const N: usize> {
    pub c0: Polynomial<i64, N>,
    pub c1: Polynomial<i64, N>,
}
