// examples/demo2.rs
// RustでCKKSを実装する
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ckks_rs_playground::ckks;
use num_complex::Complex64;

macro_rules! measure_and_print {
    ($name:expr, $body:block) => {{
        println!("--- {} ---", $name);
        // let start = std::time::Instant::now();
        let result = $body;
        // let duration = start.elapsed();
        println!("result: {:?}", result);
        // println!("elapsed: {:.2?}\\n", duration);
        println!();
        result
    }};
    ($name:expr, $val:expr) => {{
        println!("--- {} ---", $name);
        println!("result: {:?}\\n", $val);
        $val
    }};
}

fn main() {
    // ckks.py に合わせたパラメータ設定
    const M: usize = 8;
    const N: usize = M >> 1; // N = M / 2
    const LIMIT: u32 = 3; // L = 3
    const P: i64 = 1000;
    const Q0: i64 = 1000;
    const DELTA: i64 = 1000;
    // SCALE は ckks.py に明示的な変数がないが、内部計算で使われる qL = (p^L) * q0 に関連
    // qL = (1000^3) * 1000 = 10^12
    // ckks::encode などで使われる SCALE は qL のビット長に近い値が適切か？
    // ckks.py の実装詳細に依存するため、適切な値を探る必要がある。
    // ここでは仮に 40 を設定 (log2(10^12) approx 39.8)
    const SCALE: i64 = 1000; // 仮の値

    // ckks.py の出力に合わせた入力データ
    let z1 = measure_and_print!("z1", [Complex64::new(0.0, 8.0), Complex64::new(1.0, 0.0)]);
    let z2 = measure_and_print!("z2", [Complex64::new(5.0, 2.0), Complex64::new(7.0, 6.0)]);

    // Encode
    let m1 = measure_and_print!("Encode z1 (m1)", { ckks::encode::<N>(z1, DELTA) });
    let m2 = measure_and_print!("Encode z2 (m2)", { ckks::encode::<N>(z2, DELTA) });

    // Decode (検証用)
    let dcd1 = measure_and_print!("Decode m1 (dcd1)", { ckks::decode(m1, DELTA) });
    let dcd2 = measure_and_print!("Decode m2 (dcd2)", { ckks::decode(m2, DELTA) });

    // Key Generation
    let (public_key, secret_key, evaluation_key) = measure_and_print!("Generate keys", {
        ckks::generate_keys::<N>(LIMIT, P, Q0, SCALE)
    });

    // Encrypt
    let c1 = measure_and_print!("Encrypt m1 (c1)", {
        ckks::encrypt(m1, public_key, evaluation_key)
    });
    let c2 = measure_and_print!("Encrypt m2 (c2)", {
        ckks::encrypt(m2, public_key, evaluation_key)
    });

    // Decrypt (検証用)
    let dec_c1 = measure_and_print!("Decrypt c1 (dec c1)", { ckks::decrypt(c1, secret_key) });
    let dec_c2 = measure_and_print!("Decrypt c2 (dec c2)", { ckks::decrypt(c2, secret_key) });

    // Decode decrypted (検証用)
    let dcd_dec_c1 = measure_and_print!("Decode dec_c1 (dcd c1)", { ckks::decode(dec_c1, DELTA) });
    let dcd_dec_c2 = measure_and_print!("Decode dec_c2 (dcd c2)", { ckks::decode(dec_c2, DELTA) });

    // Add
    let c_add = measure_and_print!("Add c1 + c2 (add)", { c1 + c2 });
    let dec_add = measure_and_print!("Decrypt c_add (dec add)", {
        ckks::decrypt(c_add, secret_key)
    });
    let dcd_add = measure_and_print!("Decode dec_add (dcd add)", { ckks::decode(dec_add, DELTA) });
    measure_and_print!(
        "Original z1 + z2 (org add)",
        z1.iter()
            .zip(z2.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>()
    );

    // Multiply
    // 注意: Rust実装の乗算はリニアライズキー(evaluation_key)を内部で使う想定かもしれない
    // ckks.py では mul 関数に ek を渡している
    // ckks-rs-playground の Mul トレイト実装を確認する必要がある
    let c_mul = measure_and_print!("Multiply c1 * c2 (mul)", { c1 * c2 }); // 要確認: evaluation_key の使用
    let dec_mul = measure_and_print!("Decrypt c_mul (dec mul)", {
        ckks::decrypt(c_mul, secret_key)
    });
    // ckks.py では Dcd(Dec(sk, c)) / g_.delta となっている
    let dcd_mul = measure_and_print!("Decode dec_mul (dcd mul)", {
        ckks::decode(dec_mul, c_mul.scale) // スケールファクターを考慮
    });
    measure_and_print!(
        "Original z1 * z2 (org mul)",
        z1.iter()
            .zip(z2.iter())
            .map(|(a, b)| a * b)
            .collect::<Vec<_>>()
    );

    // // Multiply again (c_mul * c2)
    // let c3 = measure_and_print!("Multiply c_mul * c2 (c3)", { c_mul * c2 }); // 要確認: evaluation_key の使用
    // let dec_c3 = measure_and_print!("Decrypt c3 (dec c3)", { ckks::decrypt(c3, secret_key) });
    // // ckks.py では Dcd(Dec(sk, c3))/g_.delta**2 となっている
    // let dcd_c3 = measure_and_print!("Decode dec_c3 (dcd c3)", {
    //     ckks::decode(dec_c3, DELTA * DELTA * DELTA) // スケールファクターを考慮
    // });
    // measure_and_print!(
    //     "Original z1 * z2 * z2 (org c3)",
    //     z1.iter()
    //         .zip(z2.iter())
    //         .zip(z2.iter())
    //         .map(|((a, b), c)| a * b * c)
    //         .collect::<Vec<_>>()
    // );
}
