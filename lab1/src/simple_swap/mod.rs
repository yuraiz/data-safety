mod sbox;
use sbox::*;

fn f(input: u32, subkey: u32) -> u32 {
    let added = input.wrapping_add(subkey);
    let mixed = substitute(added, S_BOX);
    mixed.rotate_left(11)
}

fn simple_swap_core(input: u64, key: [u32; 8], decrypt: bool) -> u64 {
    let reversed_key = {
        let mut k = key;
        k.reverse();
        k
    };

    let central_key = if decrypt { reversed_key } else { key };

    let mut reg_a = (input >> 32) as u32;
    let mut reg_b = input as u32;

    let mut iteration = |subkey: u32| {
        let round = reg_b ^ f(reg_a, subkey);
        reg_b = reg_a;
        reg_a = round;
    };

    for subkey in key {
        iteration(subkey);
    }

    for _ in 0..2 {
        for subkey in central_key {
            iteration(subkey);
        }
    }

    for &subkey in &reversed_key[..7] {
        iteration(subkey)
    }

    reg_b = reg_b ^ f(reg_a, key[0]);

    ((reg_a as u64) << 32) | reg_b as u64
}

pub fn simple_swap(input: u64, key: [u32; 8]) -> u64 {
    simple_swap_core(input, key, false)
}

#[allow(unused)]
pub fn simple_swap_decrypt(input: u64, key: [u32; 8]) -> u64 {
    simple_swap_core(input, key, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_swap_reversible() {
        let key = [83, 3, 6, 24, 525, 646, 233, 32];
        let input = 42;
        assert_eq!(simple_swap_decrypt(simple_swap(42, key), key), input);
    }
}
