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
    const P: i64 = 97;
    const Q0: i64 = 7;
    const SCALE: i64 = 10000;
    const DELTA: i64 = 64;

    let z = measure!("Generate complex vector", {
        [Complex64::new(413.0, 0.0), Complex64::new(784.3, 55.0)]
    });

    let plaintext = measure!("Encode plaintext", {
        ckks::plaintext::Plaintext::<N>::encode_from(z, DELTA, SCALE)
    });
    let plaintext_decoded = measure!("Decode plaintext", { plaintext.decode(DELTA) });
    measure!("diff", { diff(&z, &plaintext_decoded) });

    let (public_key, secret_key, evaluation_key) = measure!("Generate keys", {
        ckks::generate_keys(LIMIT, P, Q0, SCALE)
    });

    let ciphertext = measure!("Encrypt plaintext", {
        ckks::encrypt(plaintext, public_key, evaluation_key)
    });

    let decrypted = measure!("Decrypt ciphertext", {
        ckks::decrypt(ciphertext, secret_key)
    });
    measure!("diff", { diff(&z, &decrypted.decode(DELTA)) });

    let decoded = measure!("Decode decrypted", { decrypted.decode(DELTA) });
    measure!("diff", { diff(&z, &decoded) });

    let plaintext_added = measure!("Add plaintexts", { plaintext + plaintext });
    let ciphertext_added = measure!("Encrypt added plaintext", {
        ckks::encrypt(plaintext_added, public_key, evaluation_key)
    });
    let decrypted_added = measure!("Decrypt added ciphertext", {
        ckks::decrypt(ciphertext_added, secret_key)
    });
    measure!("diff", {
        diff(
            &plaintext_added.decode(DELTA),
            &decrypted_added.decode(DELTA),
        )
    });

    let plaintext_multiplied = measure!("Multiply plaintexts", { plaintext * plaintext });
    let ciphertext_multiplied = measure!("Multiply ciphertexts", { ciphertext * ciphertext });
    let decrypted_multiplied = measure!("Decrypt multiplied ciphertext", {
        ckks::decrypt(ciphertext_multiplied, secret_key)
    });
    measure!("diff", {
        diff(
            &plaintext_multiplied.decode(DELTA),
            &decrypted_multiplied.decode(DELTA),
        )
    });
}
