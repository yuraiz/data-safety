use std::num::Wrapping;

pub fn u128_to_wrapping_u32(src: u128) -> [Wrapping<u32>; 4] {
    unsafe { std::mem::transmute(src) }
}

pub fn wrapping_u32_to_u128(src: [Wrapping<u32>; 4]) -> u128 {
    unsafe { std::mem::transmute(src) }
}
