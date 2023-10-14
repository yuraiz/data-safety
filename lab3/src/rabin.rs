use crate::mics::*;

#[derive(Debug, Clone, Copy)]
pub struct PrivateKeyInfo {
    p: u64,
    q: u64,
    n: u128,
    yp: i128,
    yq: i128,
}

impl PrivateKeyInfo {
    pub fn new(p: u64, q: u64) -> Self {
        let (yp, yq) = bezouts_coeffs(p as i128, q as i128);
        let n = p as u128 * q as u128;
        Self { p, q, n, yp, yq }
    }
}

fn bezouts_coeffs(a: i128, b: i128) -> (i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 1 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (s, t)
}

fn compute_m1_m2(c: u128, p: u64, q: u64) -> (u64, u64) {
    let mp = modpow(c, (p as u128 + 1) / 4, p as u128);
    let mq = modpow(c, (q as u128 + 1) / 4, q as u128);

    (mp as u64, mq as u64)
}

fn compute_decryption_results(
    PrivateKeyInfo { p, q, n, yp, yq }: PrivateKeyInfo,
    mp: u64,
    mq: u64,
) -> [u128; 4] {
    let yp = if yp < 0 {
        n - yp.unsigned_abs()
    } else {
        yp as u128
    } % n;

    let yq = if yq < 0 {
        n - yq.unsigned_abs()
    } else {
        yq as u128
    } % n;

    let p = p as u128;
    let mp = mp as u128;
    let q = q as u128;
    let mq = mq as u128;

    let rem_n = |number: u128| number.rem_euclid(n);

    let mul = |a: u128, b: u128| mulmod(a, b, n);

    let first = mul(mul(yp, p), mq);
    let second = mul(mul(yq, q), mp);

    let x1 = rem_n(first + second);
    let x2 = n - x1;
    let x3 = rem_n(first + (n - second));
    let x4 = n - x3;

    [x1, x2, x3, x4]
}

pub fn decrypt_block(block: u128, key_info: PrivateKeyInfo) -> [u64; 4] {
    let (mp, mq) = compute_m1_m2(block, key_info.p, key_info.q);
    compute_decryption_results(key_info, mp, mq).map(|n| n as _)
}

pub fn encrypt_block(block: u64, key: u128) -> u128 {
    let block = (block as u128) % key;
    (block * block) % key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coeffs() {
        assert_eq!(bezouts_coeffs(23, 7), (-3, 10));
    }

    #[test]
    fn coeffs_overflow() {
        dbg!(bezouts_coeffs(119543903707171, 180252380737439));
    }

    #[test]
    fn m1_m2() {
        assert_eq!(compute_m1_m2(93, 23, 7), (1, 4))
    }

    #[test]
    fn decryption_results() {
        let key_info = PrivateKeyInfo::new(23, 7);
        assert_eq!(
            compute_decryption_results(key_info, 1, 4),
            [116, 45, 137, 24]
        )
    }
}
