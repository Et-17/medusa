fn modpow(b: i32, e: i32, m: i32) -> i32 {
    let mut y: u128 = 1;
    let mut b: u128 = b as u128;
    let mut e: u128 = e as u128;
    let m: u128 = m as u128;
    while e > 0 {
        if e & 1 >= 1 {
            y = (y * b) % m;
        }
        b = (b * b) % m;
        e >>= 1;
    }
    return y as i32;
}

fn summation(n: i32, a: i32) -> f64 {
    let mut left = 0.0;
    for k in 0..=n {
        let r = 8 * k + a;
        left = (left + modpow(16, n - k, r) as f64 / r as f64).fract();
    }

    // let mut right = 0.0;
    // for k in (n + 1).. {
    //     let rnew = right + 16f64.powi(n - k) / (8 * k + a) as f64;
    //     if right == rnew {
    //         break;
    //     }
    //     right = rnew;
    // }

    return left;
}

pub fn digit(n: i32) -> u8 {
    let mut sum = 4.0 * summation(n, 1);
    sum -= 2.0 * summation(n, 4);
    sum -= summation(n, 5);
    sum -= summation(n, 6);
    return (sum.rem_euclid(1.0) * 16.0) as u8;
}
