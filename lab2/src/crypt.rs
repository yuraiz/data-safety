use std::{mem::swap, num::Wrapping};

use crate::transform::wrapping::*;

use crate::misc::{u128_to_wrapping_u32, wrapping_u32_to_u128};

pub fn crypt(word: u128, key: [u32; 8]) -> u128 {
    let [mut a, mut b, mut c, mut d] = u128_to_wrapping_u32(word);
    let mut e;

    let t_key = |index: usize| Wrapping(key[index % 8]);

    for i in 1..=8usize {
        let i7 = i * 7;

        b ^= g5(a + t_key(i7 - 6));
        c ^= g21(d + t_key(i7 - 5));
        a -= g13(b + t_key(i7 - 4));

        e = g21(b + c + t_key(i7 - 3)) + Wrapping(i as u32);

        b += e;
        c -= e;

        d += g13(c + t_key(i7 - 2));
        b ^= g21(a + t_key(i7 - 1));
        c ^= g5(d + t_key(i7));

        swap(&mut a, &mut b);
        swap(&mut c, &mut d);
        swap(&mut b, &mut c);
    }

    wrapping_u32_to_u128([b, d, a, c])
}
