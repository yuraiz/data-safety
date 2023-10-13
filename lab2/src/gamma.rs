use crate::crypt::crypt;

pub fn gamma_crypt(init: u128, message: &mut [u128], key: [u32; 8]) -> u128 {
    let mut last_encrypted = init;

    message.iter_mut().for_each(|block| {
        let gamma = crypt(last_encrypted, key);
        *block ^= gamma;
        last_encrypted = *block;
    });

    last_encrypted
}

pub fn gamma_decrypt(init: u128, message: &mut [u128], key: [u32; 8]) -> u128 {
    let mut last = init;

    message.iter_mut().for_each(|block| {
        let gamma = crypt(last, key);
        last = *block;
        *block ^= gamma;
    });

    last
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gamma() {
        let init = 0xBADF00D;
        let key = [2341, 325, 532, 12, 5325, 234, 52, 2];
        let message = [2342, 352, 6, 6436];

        let mut copy = message.clone();

        gamma_crypt(init, &mut copy, key);
        assert_ne!(message, copy);
        gamma_decrypt(init, &mut copy, key);
        assert_eq!(message, copy);
    }
}
