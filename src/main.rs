// RustでCKKSを実装する

use num_complex::Complex64;

mod ckks;

fn main() {
    const M: usize = 1 << 3;
    const N: usize = M >> 1;
    const SCALE: i64 = 999999937;
    const DELTA: i64 = 64;

    let z: [Complex64; N] = [
        Complex64::new(413.0, 0.0),
        Complex64::new(784.3, 55.0),
        Complex64::new(-74.2, 43.0),
        Complex64::new(0.0, 10.0),
    ];
    println!("z: {:?}", z);

    let plaintext = ckks::plaintext::Plaintext::encode_from(z, DELTA);
    println!("plaintext: {:?}", plaintext);

    let key_generator = ckks::keys::KeyGenerator::new(3, 97, 7, SCALE);
    let (public_key, secret_key, evaluation_key) = key_generator.generate_keys();
    println!("public key: {:?}", public_key);
    println!("secret key: {:?}", secret_key);

    let ciphertext = public_key.encrypt(plaintext, evaluation_key, SCALE);
    println!("ciphertext: {:?}", ciphertext);

    let decrypted = secret_key.decrypt(ciphertext);
    println!("decrypted: {:?}", decrypted);

    let decoded = decrypted.decode_to();
    println!("decoded: {:?}", decoded);

    let plaintext_added = plaintext + plaintext;
    let ciphertext_added = ciphertext + ciphertext;
    let decrypted_added = secret_key.decrypt(ciphertext_added);
    println!("plaintext_added: {:?}", plaintext_added);
    println!("decrypted_added: {:?}", decrypted_added);
    println!("diff: {:?}", decrypted_added.m + -plaintext_added.m);

    let plaintext_multiplied = plaintext * plaintext;
    let ciphertext_multiplied = ciphertext * ciphertext;
    let decrypted_multiplied = secret_key.decrypt(ciphertext_multiplied);
    println!("plaintext_multiplied: {:?}", plaintext_multiplied);
    println!("decrypted_multiplied: {:?}", decrypted_multiplied);
    println!(
        "diff: {:?}",
        decrypted_multiplied.m + -plaintext_multiplied.m
    );

    // Output:
    // z: [Complex { re: 413.0, im: 0.0 }, Complex { re: 784.3, im: 55.0 }, Complex { re: -74.2, im: 43.0 }, Complex { re: 0.0, im: 10.0 }]
    // plaintext: Plaintext { m: Polynomial { coeffs: [35939, -6677, -704, 6768], modulo: 9223372036854775807 }, delta: 64 }
    // public key: PublicKey { b: Polynomial { coeffs: [-33, 70, -28, 152], modulo: 6388711 }, a: Polynomial { coeffs: [68, 60, 62, -24], modulo: 6388711 } }
    // secret key: SecretKey { s: Polynomial { coeffs: [1, -1, 0, -1], modulo: 6388711 } }
    // ciphertext: Ciphertext { c0: Polynomial { coeffs: [36014, -6625, -558, 6651], modulo: 6388711 }, c1: Polynomial { coeffs: [52, -21, -17, 154], modulo: 6388711 }, evaluation_key: EveluationKey { b: Polynomial { coeffs: [2999999984, -1999999921, 1999999898, -1999999902], modulo: 6388710597511207 }, a: Polynomial { coeffs: [-66, 63, 85, 47], modulo: 6388710597511207 } }, scale: 999999937 }
    // decrypted: Plaintext { m: Polynomial { coeffs: [35933, -6681, -708, 6770], modulo: 6388711 }, delta: 1 }
    // decoded: [Complex { re: 26421.70668625975, im: -645.0674964743957 }, Complex { re: 45444.29331374026, im: 770.9325035256015 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]
    // plaintext_added: Plaintext { m: Polynomial { coeffs: [71878, -13354, -1408, 13536], modulo: 9223372036854775807 }, delta: 64 }
    // decrypted_added: Plaintext { m: Polynomial { coeffs: [71866, -13362, -1416, 13540], modulo: 6388711 }, delta: 1 }
    // diff: Polynomial { coeffs: [-12, -8, -8, 4], modulo: 6388711 }
    // plaintext_multiplied: Plaintext { m: Polynomial { coeffs: [1201727465, -489458750, 39786041, 495871520], modulo: 9223372036854775807 }, delta: 4096 }
    // decrypted_multiplied: Plaintext { m: Polynomial { coeffs: [-6261099, -4154402, -5175041, -2323864], modulo: 6388711 }, delta: 1 }
    // diff: Polynomial { coeffs: [-522185, -237688, -240105, -6264637], modulo: 6388711 }
}
