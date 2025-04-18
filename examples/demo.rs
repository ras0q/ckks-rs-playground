// RustでCKKSを実装する
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ckks_rs_playground::ckks;
use num_complex::Complex64;

macro_rules! measure {
    ($name:expr, $body:block) => {{
        let start = std::time::Instant::now();
        let result = $body;
        let duration = start.elapsed();
        println!("[{}, {:.2?}]\nresult: {:.3?}\n", $name, duration, result);
        result
    }};
}

fn diff<T: std::fmt::Debug + std::ops::Sub<Output = T> + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().zip(b.iter()).map(|(x, y)| *x - *y).collect()
}

fn main() {
    const M: usize = 1 << 3;
    const N: usize = M >> 1;
    const LIMIT: u32 = 3;
    const P: i64 = 1000;
    const Q0: i64 = 1000;
    const SCALE: i64 = 1000;
    const DELTA: i64 = 1000;

    let z = measure!("Generate complex vector", {
        [Complex64::new(20.0, 0.0), Complex64::new(30.3, 55.0)]
    });

    let plaintext = measure!("Encode plaintext", { ckks::encode::<i64, N>(z, DELTA) });
    let plaintext_decoded = measure!("Decode plaintext", { ckks::decode(plaintext) });
    measure!("diff (noop)", { diff(&z, &plaintext_decoded) });

    let (public_key, secret_key, evaluation_key) = measure!("Generate keys", {
        ckks::generate_keys(LIMIT, P, Q0, SCALE)
    });

    let ciphertext = measure!("Encrypt plaintext", {
        ckks::encrypt(plaintext, public_key, evaluation_key)
    });

    let decrypted = measure!("Decrypt ciphertext", {
        ckks::decrypt(ciphertext, secret_key)
    });
    measure!("diff (encdec)", { diff(&z, &ckks::decode(decrypted)) });

    let plaintext_added = measure!("Add plaintexts", { plaintext + plaintext });
    let ciphertext_added = measure!("Add ciphertexts", { ciphertext + ciphertext });
    let decrypted_added = measure!("Decrypt added ciphertext", {
        ckks::decrypt(ciphertext_added, secret_key)
    });
    let z_added = measure!("Add complex vectors", { z.map(|c| c * 2.0) });
    let plaintext_added_decoded =
        measure!("Decode added plaintext", { ckks::decode(plaintext_added) });
    measure!("diff (add)", { diff(&z_added, &plaintext_added_decoded) });
    let decrypted_added_decoded =
        measure!("Decode added ciphertext", { ckks::decode(decrypted_added) });
    measure!("diff (add)", { diff(&z_added, &decrypted_added_decoded) });

    let plaintext_multiplied = measure!("Multiply plaintexts", { plaintext * plaintext });
    let ciphertext_multiplied = measure!("Multiply ciphertexts", { ciphertext * ciphertext });
    let decrypted_multiplied = measure!("Decrypt multiplied ciphertext", {
        ckks::decrypt(ciphertext_multiplied, secret_key)
    });
    let z_multiplied = measure!("Multiply complex vectors", { z.map(|c| c * c) });
    let plaintext_multiplied_decoded = measure!("Decode multiplied plaintext", {
        ckks::decode(plaintext_multiplied)
    });
    measure!("diff (mul)", {
        diff(&z_multiplied, &plaintext_multiplied_decoded)
    });
    let decrypted_multiplied_decoded = measure!("Decode multiplied ciphertext", {
        ckks::decode(decrypted_multiplied)
    });
    measure!("diff (mul)", {
        diff(&z_multiplied, &decrypted_multiplied_decoded)
    });
}
