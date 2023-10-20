use num::{BigInt, Integer};

pub fn modpow(mut base: BigInt, mut exp: BigInt, modulus: &BigInt) -> BigInt {
    base = base.div_rem(modulus).1;

    let mut result = BigInt::from(1);

    while exp > 0.into() {
        if exp.bit(0) {
            result = (result * &base).div_rem(modulus).1;
        }
        base = (&base * &base).div_rem(modulus).1;
        exp >>= 1;
    }
    result
}
