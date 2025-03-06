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

    let plaintext_multiplied = plaintext * plaintext;
    let ciphertext_multiplied = ciphertext * ciphertext;
    let decrypted_multiplied = secret_key.decrypt(ciphertext_multiplied);
    println!("plaintext_multiplied: {:?}", plaintext_multiplied);
    println!("decrypted_multiplied: {:?}", decrypted_multiplied);
}
