use crate::mics::*;

use num::{bigint::BigInt, Integer, Zero};

#[derive(Debug, Clone)]
pub struct PrivateKeyInfo {
    p: BigInt,
    q: BigInt,
    n: BigInt,
    yp: BigInt,
    yq: BigInt,
}

impl PrivateKeyInfo {
    pub fn new(p: BigInt, q: BigInt) -> Self {
        let (yp, yq) = bezouts_coeffs(p.clone(), q.clone());
        let n = &p * &q;
        Self { p, q, n, yp, yq }
    }
}

fn bezouts_coeffs(a: BigInt, b: BigInt) -> (BigInt, BigInt) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (BigInt::from(1), BigInt::zero());
    let (mut old_t, mut t) = (BigInt::zero(), BigInt::from(1));

    while r != 1.into() {
        let quotient = &old_r / &r;
        (old_r, r) = (r.clone(), old_r - &quotient * &r);
        (old_s, s) = (s.clone(), old_s - &quotient * &s);
        (old_t, t) = (t.clone(), old_t - &quotient * &t);
    }

    (s, t)
}

fn compute_m1_m2(c: BigInt, p: &BigInt, q: &BigInt) -> (BigInt, BigInt) {
    let mp = modpow(c.clone(), (p.clone() + 1) / 4, p);
    let mq = modpow(c, (q.clone() + 1) / 4, q);

    (mp, mq)
}

fn compute_decryption_results(
    PrivateKeyInfo { p, q, n, yp, yq }: &PrivateKeyInfo,
    mp: &BigInt,
    mq: &BigInt,
) -> [BigInt; 4] {
    let first = yp * p * mq;
    let second = yq * q * mp;

    let x1 = (&first + &second).mod_floor(n);
    let x2 = n - &x1;
    let x3 = (first - second).mod_floor(n);
    let x4 = n - &x3;

    [x1, x2, x3, x4]
}

pub fn decrypt(number: BigInt, key_info: &PrivateKeyInfo) -> [BigInt; 4] {
    let (mp, mq) = compute_m1_m2(number, &key_info.p, &key_info.q);
    compute_decryption_results(key_info, &mp, &mq).map(|n| n as _)
}

pub fn encrypt(number: BigInt, key: &BigInt) -> BigInt {
    (&number * &number) % key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coeffs() {
        assert_eq!(
            bezouts_coeffs(23.into(), 7.into()),
            ((-3).into(), 10.into())
        );
    }

    #[test]
    fn coeffs_overflow() {
        dbg!(bezouts_coeffs(
            BigInt::from_bytes_le(
                num::bigint::Sign::Plus,
                &119543903707171i128.to_le_bytes()[..]
            ),
            BigInt::from_bytes_le(
                num::bigint::Sign::Plus,
                &180252380737439u128.to_le_bytes()[..]
            ),
        ));
    }

    #[test]
    fn m1_m2() {
        assert_eq!(
            compute_m1_m2(93.into(), &23.into(), &7.into()),
            (1.into(), 4.into())
        )
    }

    #[test]
    fn decryption_results() {
        let key_info = PrivateKeyInfo::new(23.into(), 7.into());
        assert_eq!(
            compute_decryption_results(&key_info, &1.into(), &4.into()),
            [116.into(), 45.into(), 137.into(), 24.into()]
        )
    }
}
