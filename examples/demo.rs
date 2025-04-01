// RustでCKKSを実装する

use ckks_rs_playground::ckks;
use num_complex::Complex64;

fn main() {
    const M: usize = 1 << 3;
    const N: usize = M >> 1;
    const LIMIT: u32 = 3;
    const P: i64 = 97;
    const Q0: i64 = 7;
    const SCALE: i64 = 10000;
    const DELTA: i64 = 64;

    let z: [Complex64; N] = [
        Complex64::new(413.0, 0.0),
        Complex64::new(784.3, 55.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
    ];
    println!("z: {:?}", z);

    let plaintext = ckks::plaintext::Plaintext::encode_from(z, DELTA);
    println!("plaintext: {:?}", plaintext);
    let plaintext_decoded = plaintext.decode(DELTA);
    println!("decoded: {:?}", plaintext_decoded);
    println!(
        "diff: {:?}\n",
        z.iter()
            .zip(plaintext_decoded.iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>()
    );

    let (public_key, secret_key, evaluation_key) = ckks::generate_keys(LIMIT, P, Q0, SCALE);
    println!("public key: {:?}", public_key);
    println!("secret key: {:?}", secret_key);

    let ciphertext = ckks::encrypt(plaintext, public_key, evaluation_key, SCALE);
    println!("ciphertext: {:?}", ciphertext);

    let decrypted = ckks::decrypt(ciphertext, secret_key);
    println!("decrypted: {:?}", decrypted);
    println!("diff: {:?}\n", decrypted.m + -plaintext.m);

    let decoded = decrypted.decode(DELTA);
    println!("decoded: {:?}", decoded);
    println!(
        "diff: {:?}\n",
        z.iter()
            .zip(decoded.iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>()
    );

    let plaintext_added = plaintext + plaintext;
    let ciphertext_added = ckks::encrypt(plaintext_added, public_key, evaluation_key, SCALE);
    let decrypted_added = ckks::decrypt(ciphertext_added, secret_key);
    println!("plaintext_added: {:?}", plaintext_added);
    println!("decrypted_added: {:?}", decrypted_added);
    println!("diff: {:?}\n", decrypted_added.m + -plaintext_added.m);

    let plaintext_multiplied = plaintext * plaintext;
    let ciphertext_multiplied = ciphertext * ciphertext;
    let decrypted_multiplied = ckks::decrypt(ciphertext_multiplied, secret_key);
    println!("plaintext_multiplied: {:?}", plaintext_multiplied);
    println!("decrypted_multiplied: {:?}", decrypted_multiplied);
    println!(
        "diff: {:?}\n",
        decrypted_multiplied.m + -plaintext_multiplied.m
    );

    // Output:
    // z: [Complex { re: 413.0, im: 0.0 }, Complex { re: 784.3, im: 55.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]
    // plaintext: Plaintext { m: Polynomial { coeffs: [38314, -7157, -1760, 9646], modulo: 9223372036854775807 } }
    // decoded: [Complex { re: 413.00757430816316, im: -0.00017533791690027556 }, Complex { re: 784.3049256918368, im: 54.99982466208304 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]
    // diff: [Complex { re: -0.007574308163157184, im: 0.00017533791690027556 }, Complex { re: -0.004925691836888291, im: 0.00017533791695711898 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]
    //
    // public key: PublicKey { b: Polynomial { coeffs: [19, -8, -23, 9], modulo: 6388711 }, a: Polynomial { coeffs: [-41, 34, -61, 44], modulo: 6388711 } }
    // secret key: SecretKey { s: Polynomial { coeffs: [-1, 0, 1, 0], modulo: 6388711 } }
    // ciphertext: Ciphertext { c0: Polynomial { coeffs: [38329, -7159, -1786, 9657], modulo: 6388711 }, c1: Polynomial { coeffs: [-122, 136, -142, 147], modulo: 6388711 }, evaluation_key: EveluationKey { b: Polynomial { coeffs: [19949, -23, -19953, 26], modulo: 63887110000 }, a: Polynomial { coeffs: [21, -18, 69, 6], modulo: 63887110000 } }, scale: 10000 }
    // decrypted: Plaintext { m: Polynomial { coeffs: [38309, -7148, -1766, 9646], modulo: 6388711 } }
    // diff: Polynomial { coeffs: [-5, 9, -6, 0], modulo: 6388711 }
    //
    // decoded: [Complex { re: 413.0288861992675, im: 0.0055115531874605495 }, Complex { re: 784.1273638007325, im: 55.193011553187404 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]
    // diff: [Complex { re: -0.028886199267503798, im: -0.0055115531874605495 }, Complex { re: 0.17263619926745832, im: -0.1930115531874037 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]
    //
    // plaintext_added: Plaintext { m: Polynomial { coeffs: [76628, -14314, -3520, 19292], modulo: 9223372036854775807 } }
    // decrypted_added: Plaintext { m: Polynomial { coeffs: [76618, -14296, -3532, 19292], modulo: 6388711 } }
    // diff: Polynomial { coeffs: [-10, 18, -12, 0], modulo: 6388711 }
    //
    // plaintext_multiplied: Plaintext { m: Polynomial { coeffs: [1332987352, -582380516, 9402685, 764346328], modulo: 9223372036854775807 } }
    // decrypted_multiplied: Plaintext { m: Polynomial { coeffs: [-2442136, -361555, -3945032, -2342236], modulo: 6388711 } }
    // diff: Polynomial { coeffs: [-188889, 646260, -570295, -43244], modulo: 6388711 }
}
