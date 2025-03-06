use rand::Rng;

pub fn rand_array<const N: usize>(range: std::ops::Range<i64>) -> [i64; N] {
    let mut rng = rand::rng();
    std::array::from_fn(|_| rng.random_range(range.clone()))
}
