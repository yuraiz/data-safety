pub fn modpow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    base %= modulus;

    let mut result = 1;

    while exp > 0 {
        if (exp & 1) != 0 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

pub fn mulmod(a: u128, b: u128, n: u128) -> u128 {
    let mut a = a % n;
    let mut b = b % n;

    if let Some(res) = a.checked_mul(b) {
        return res % n;
    }

    let mut res = 0;

    while b > 0 {
        // If b is odd, add 'a' to result
        if b % 2 == 1 {
            res = (res + a) % n;
        }

        // Multiply 'a' with 2
        a = (a * 2) % n;

        // Divide b by 2
        b /= 2;
    }

    res
}
