mod ckks;
mod poly;
mod random;

// RustでCKKSを実装する

fn main() {
    const M: usize = 1 << 3;
    const N: usize = M >> 1;

    let plaintext = ckks::Plaintext {
        m: poly::Polynomial::<i64, N>::new(random::rand_array(-1000..1000), 1000000007),
    };
    println!("plaintext: {:?}", plaintext);

    let key_generator = ckks::KeyGenerator::new(100, 1000000007, 100);
    let (public_key, secret_key) = key_generator.generate_keys();
    println!("public key: {:?}", public_key);
    println!("secret key: {:?}", secret_key);

    let ciphertext = public_key.encrypt(plaintext);
    println!("ciphertext: {:?}", ciphertext);

    let decrypted = secret_key.decrypt(ciphertext);
    println!("decrypted: {:?}", decrypted);
}
