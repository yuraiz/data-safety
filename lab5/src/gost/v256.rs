use std::ops::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct V256([u128; 2]);

impl V256 {
    pub const fn zero() -> Self {
        Self([0, 0])
    }

    pub const fn new(lo: u128, hi: u128) -> Self {
        Self([lo, hi])
    }
}

impl From<[u128; 2]> for V256 {
    fn from(value: [u128; 2]) -> Self {
        Self(value)
    }
}

impl From<V256> for [u128; 2] {
    fn from(value: V256) -> Self {
        value.0
    }
}

macro_rules! transmute_impl {
    ($type:ty) => {
        impl From<$type> for V256 {
            fn from(value: $type) -> Self {
                use std::mem::transmute;
                unsafe { Self(transmute(value)) }
            }
        }

        impl From<V256> for $type {
            fn from(value: V256) -> Self {
                use std::mem::transmute;
                unsafe { transmute(value.0) }
            }
        }
    };
}

transmute_impl!([u64; 4]);
transmute_impl!([u32; 8]);
transmute_impl!([u16; 16]);
transmute_impl!([u8; 32]);

macro_rules! bit_op_impl {
    ($trait:ty, $fn:ident, $op:tt) => {
        impl $trait for V256 {
            type Output = V256;

            fn $fn(self, rhs: Self) -> Self::Output {
                let [a0, a1] = self.0;
                let [b0, b1] = rhs.0;
                Self([a0 $op b0, a1 $op b1])
            }
        }
    };
}

bit_op_impl!(BitOr, bitor, |);
bit_op_impl!(BitAnd, bitand, &);
bit_op_impl!(BitXor, bitxor, ^);

impl Add for V256 {
    type Output = V256;

    fn add(self, rhs: Self) -> Self::Output {
        let [a0, a1] = self.0;
        let [b0, b1] = rhs.0;

        let (r0, overflowed) = a0.overflowing_add(b0);
        let r1 = if overflowed {
            a1.wrapping_add(b1).wrapping_add(1)
        } else {
            a1.wrapping_add(b1)
        };

        Self([r0, r1])
    }
}

impl AddAssign for V256 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
