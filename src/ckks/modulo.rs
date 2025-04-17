use num_integer::Integer;

// Remainder in range (-modulo/2, modulo/2]
pub fn cmod<T: Integer + Copy>(x: T, modulo: T) -> T {
    if is_in_range(x, modulo) {
        return x;
    }

    println!("MODULO OVER!!");

    let t = x % modulo;

    let two = T::one() + T::one();
    let half_modulo = modulo / two;

    if t < half_modulo { t } else { t - modulo }
}

pub fn is_in_range<T: Integer + Copy>(x: T, modulo: T) -> bool {
    let zero = T::zero();
    let two = T::one() + T::one();
    let half_modulo = modulo / two;

    // Check if x is in the range (-modulo/2, modulo/2]
    x + half_modulo > zero && x - half_modulo <= zero
}
