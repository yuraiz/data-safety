use super::simple_swap::*;

pub fn cipher_feedback_mode_block(init: u64, input: &mut [u64], key: [u32; 8]) -> u64 {
    let mut last_encrypted: u64 = init;

    input.iter_mut().for_each(|block| {
        last_encrypted = *block ^ simple_swap(last_encrypted, key);
        *block = last_encrypted;
    });

    last_encrypted
}

pub fn cipher_feedback_mode_block_decrypt(init: u64, input: &mut [u64], key: [u32; 8]) -> u64 {
    let mut last = init;

    input.iter_mut().for_each(|block| {
        let open = *block ^ simple_swap(last, key);
        last = *block;
        *block = open;
    });

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfm_reversible() {
        let key = [83, 3, 6, 24, 525, 646, 233, 32];
        let init = 0xBADF00D;

        let initial = [42, 45, 38];
        let mut copy = initial.clone();

        cipher_feedback_mode_block(init, &mut copy, key);
        cipher_feedback_mode_block_decrypt(init, &mut copy, key);
        assert_eq!(initial, copy);
    }
}
