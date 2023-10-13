use super::subs::subs_byte;

fn transform(input: u32, n: u32) -> u32 {
    let mut bytes = input.to_le_bytes();
    bytes.iter_mut().for_each(|byte| *byte = subs_byte(*byte));
    u32::from_le_bytes(bytes).rotate_left(n)
}

pub mod wrapping {
    use super::transform;
    use std::num::Wrapping;

    pub fn g5(v: Wrapping<u32>) -> Wrapping<u32> {
        Wrapping(transform(v.0, 5))
    }

    pub fn g13(v: Wrapping<u32>) -> Wrapping<u32> {
        Wrapping(transform(v.0, 13))
    }

    pub fn g21(v: Wrapping<u32>) -> Wrapping<u32> {
        Wrapping(transform(v.0, 21))
    }
}
