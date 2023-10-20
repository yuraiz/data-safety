mod v256;
use v256::V256;

use crate::MyHasher;

use lab1::*;

fn step_hash_function(h_in: V256, m: V256) -> V256 {
    fn key_gen(h_in: V256, m: V256) -> [V256; 4] {
        fn transform_a(y: V256) -> V256 {
            let [y4, y3, y2, y1] = y.into();
            [y1 ^ y2, y4, y3, y2].into()
        }

        // Precomputed coefficients
        #[rustfmt::skip]
        fn transform_p(v: V256) -> V256 {
            let [y32, y31, y30, y29, y28, y27, y26, y25, y24, y23, y22, y21,
                 y20, y19, y18, y17, y16, y15, y14, y13, y12, y11, y10, y9,
                 y8, y7, y6, y5, y4, y3, y2, y1] = v.into();

            [y32, y24, y16, y8, y31, y23, y15, y7, y30, y22,
             y14, y6, y29, y21, y13, y5, y28,y20, y12, y4, y27,
             y19, y11, y3, y26, y18, y10, y2, y25, y17, y9, y1,].into()
        }

        // Other constants are zeroes
        const C3: V256 = V256::new(
            0xff00ffff000000ffff0000ff00ffff00,
            0x00ff00ff00ff00ffff00ff00ff00ff00,
        );

        let mut u = h_in;
        let mut v = m;
        let mut w = u ^ v;
        let k1 = transform_p(w);

        let mut j = 0;

        let mut next_key = || {
            u = transform_a(u) ^ [V256::zero(), C3, V256::zero()][j];
            j += 1;

            v = transform_a(transform_a(v));
            w = u ^ v;

            transform_p(w)
        };

        let k2 = next_key();
        let k3 = next_key();
        let k4 = next_key();

        [k1, k2, k3, k4]
    }

    // Key generation
    let keys = key_gen(h_in, m);

    // Encrypting transform
    let h: [u64; 4] = h_in.into();

    let s: V256 = [3, 2, 1, 0]
        .map(|i| simple_swap(h[i], keys[i].into()))
        .into();

    // Mixing transform

    fn psi(y: V256) -> V256 {
        let mut y: [u16; 16] = y.into();

        let y1 = y[0] ^ y[1] ^ y[2] ^ y[3] ^ y[12] ^ y[15];

        y.rotate_left(1);
        y[15] = y1;

        y.into()
    }

    fn psi_rounds(v: V256, rounds: usize) -> V256 {
        let mut value = v;
        for _ in 0..rounds {
            value = psi(value);
        }
        value
    }

    psi_rounds(psi(psi_rounds(s, 12) ^ m) ^ h_in, 61)
}

#[derive(Debug, Default)]
pub struct GostContext {
    h: V256,
    sum: V256,
    len: usize,
}

impl GostContext {
    pub fn new(h: V256) -> Self {
        Self {
            h,
            ..Default::default()
        }
    }

    fn internal_iter(&mut self, m: V256) {
        self.h = step_hash_function(self.h, m);
        self.len += 256;
        self.sum += m;
    }

    fn finalize(mut self, m: &[u8]) -> V256 {
        self.len += m.len() * u8::BITS as usize;

        let mut m_buff = [0; 32];

        m_buff[32 - m.len()..].copy_from_slice(m);

        let m = V256::from(m_buff);

        // something something
        self.sum += m;
        self.h = step_hash_function(self.h, m);
        self.h = step_hash_function(self.h, V256::new(self.len as u128, 0));
        self.h = step_hash_function(self.h, self.sum);
        self.h
    }
}

impl MyHasher for GostContext {
    type Output = [u64; 4];

    const CHUNK_SIZE: usize = std::mem::size_of::<V256>();

    fn process_chunk(&mut self, chunk: &[u8]) {
        let m: [u8; 32] = chunk.try_into().unwrap();
        let m = V256::from(m);
        self.internal_iter(m);
    }

    fn finish(self, remainder: &[u8]) -> Self::Output {
        self.finalize(remainder).into()
    }
}
