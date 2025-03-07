// RustでCKKSを実装する

mod ckks;

fn main() {
    const M: usize = 1 << 3;
    const N: usize = M >> 1;
    const MODULO: i64 = 1000000007;
    const SCALE: i64 = 999999937;

    let plaintext = ckks::ciphertext::Plaintext {
        m: ckks::poly::Polynomial::<i64, N>::new(ckks::random::rand_array(-1000..1000), MODULO),
    };
    println!("plaintext: {:?}", plaintext);

    let key_generator = ckks::keys::KeyGenerator::new(3, 97, 7, SCALE);
    let (public_key, secret_key, evaluation_key) = key_generator.generate_keys();
    println!("public key: {:?}", public_key);
    println!("secret key: {:?}", secret_key);

    let ciphertext = public_key.encrypt(plaintext, evaluation_key, SCALE);
    println!("ciphertext: {:?}", ciphertext);

    let decrypted = secret_key.decrypt(ciphertext);
    println!("decrypted: {:?}", decrypted);

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
    // plaintext: Plaintext { m: Polynomial { coeffs: [609, -590, 183, -649], modulo: 1000000007 } }
    // public key: PublicKey { b: Polynomial { coeffs: [-14, -26, 52, -90], modulo: 6388711 }, a: Polynomial { coeffs: [-91, -16, -28, 55], modulo: 6388711 } }
    // secret key: SecretKey { s: Polynomial { coeffs: [0, 0, 0, -1], modulo: 6388711 } }
    // ciphertext: Ciphertext { c0: Polynomial { coeffs: [713, -653, 350, -777], modulo: 6388711 }, c1: Polynomial { coeffs: [-133, 105, -67, 173], modulo: 6388711 }, evaluation_key: EveluationKey { b: Polynomial { coeffs: [1, -39, 999999957, 87], modulo: 6388710597511207 }, a: Polynomial { coeffs: [88, 0, -37, 22], modulo: 6388710597511207 } }, scale: 999999937 }
    // decrypted: Plaintext { m: Polynomial { coeffs: [608, -586, 177, -644], modulo: 6388711 } }
    // plaintext_added: Plaintext { m: Polynomial { coeffs: [1218, -1180, 366, -1298], modulo: 1000000007 } }
    // decrypted_added: Plaintext { m: Polynomial { coeffs: [1216, -1172, 354, -1288], modulo: 6388711 } }
    // diff: Polynomial { coeffs: [-2, 8, -12, 10], modulo: 6388711 }
    // plaintext_multiplied: Plaintext { m: Polynomial { coeffs: [1170190, -956154, 992195, -1006422], modulo: 1000000007 } }
    // decrypted_multiplied: Plaintext { m: Polynomial { coeffs: [1096985, -880464, 914856, -939436], modulo: 6388711 } }
    // diff: Polynomial { coeffs: [-73205, 75690, -77339, 66986], modulo: 6388711 }
}
