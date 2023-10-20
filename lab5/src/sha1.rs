const CHUNK_SIZE: usize = 16 * size_of::<u32>();

use std::{fmt::LowerHex, mem::size_of, num::Wrapping};

fn rol(num: Wrapping<u32>, c: u32) -> Wrapping<u32> {
    Wrapping(num.0.rotate_left(c))
}

const H_INIT: [Wrapping<u32>; 5] = [
    Wrapping(0x67452301),
    Wrapping(0xEFCDAB89),
    Wrapping(0x98BADCFE),
    Wrapping(0x10325476),
    Wrapping(0xC3D2E1F0),
];

type AuxFn = fn(Wrapping<u32>, Wrapping<u32>, Wrapping<u32>) -> Wrapping<u32>;

const AUX_FUNC_TABLE: [AuxFn; 4] = {
    let ch = |x: Wrapping<u32>, y, z| (((x) & (y)) | ((!x) & (z)));
    let parity = |x, y, z| ((x) ^ (y) ^ (z));
    let maj = |x, y, z| (((x) & (y)) | ((x) & (z)) | ((y) & (z)));

    [ch, parity, maj, parity]
};

const K_TABLE: [Wrapping<u32>; 4] = [
    Wrapping(0x5A827999),
    Wrapping(0x6ED9EBA1),
    Wrapping(0x8F1BBCDC),
    Wrapping(0xCA62C1D6),
];

pub struct Sha1Context {
    h: [Wrapping<u32>; 5],
    processed_bytes: usize,
}

impl Sha1Context {
    pub fn new() -> Self {
        let h = H_INIT;
        Self {
            h,
            processed_bytes: 0,
        }
    }

    pub fn process_chunk(&mut self, chunk: &[u8]) {
        self.processed_bytes += CHUNK_SIZE;

        let mut w = [0u32; 80];

        for (i, uint32) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes(uint32.try_into().unwrap());
        }

        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1)
        }

        let h = &mut self.h;

        let [mut a, mut b, mut c, mut d, mut e] = h;

        for (i, wi) in w.into_iter().enumerate() {
            let (fun, k) = {
                let index = i / 20;
                (AUX_FUNC_TABLE[index], K_TABLE[index])
            };

            (e, d, c, b, a) = (
                d,
                c,
                rol(b, 30),
                a,
                rol(a, 5) + fun(b, c, d) + e + k + Wrapping(wi),
            );
        }

        h[0] += a;
        h[1] += b;
        h[2] += c;
        h[3] += d;
        h[4] += e;
    }

    pub fn finish(mut self, remainder: &[u8]) -> [u32; 5] {
        self.processed_bytes += remainder.len();

        let bit_len = self.processed_bytes as u64 * u8::BITS as u64;

        let rem_len = remainder.len();

        let mut rem_buff = [0u8; 16 * 4];
        rem_buff[..rem_len].copy_from_slice(remainder);
        rem_buff[rem_len] = 128;

        if rem_len >= (CHUNK_SIZE - size_of::<u64>()) {
            self.process_chunks(&rem_buff);
            rem_buff.fill(0);
        }

        rem_buff[(CHUNK_SIZE - size_of::<u64>())..].copy_from_slice(&bit_len.to_be_bytes());

        self.process_chunk(&rem_buff);

        self.h.map(|w| w.0)
    }

    pub fn process_chunks<'a>(&mut self, message: &'a [u8]) -> &'a [u8] {
        let chunks = message.chunks_exact(CHUNK_SIZE);
        let remainder = chunks.remainder();

        chunks.for_each(|c| self.process_chunk(c));

        remainder
    }

    #[allow(unused)]
    pub fn process_to_end(mut self, message: &[u8]) -> [u32; 5] {
        let remainder = self.process_chunks(message);
        self.finish(remainder)
    }
}

pub fn format_big_hex<T: LowerHex, const N: usize>(h: [T; N]) -> String {
    h.map(|n| format!("{n:08x}")).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sha1(value: &[u8], expected_hash: &str) {
        let h = Sha1Context::new().process_to_end(value);
        let hash = format_big_hex(h);

        assert_eq!(hash, expected_hash)
    }

    #[test]
    fn empty() {
        assert_sha1(b"", "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn dog() {
        assert_sha1(
            b"The quick brown fox jumps over the lazy dog",
            "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
        );
    }

    #[test]
    fn cog() {
        assert_sha1(
            b"The quick brown fox jumps over the lazy cog",
            "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3",
        );
    }

    fn assert_same_as_command(value: &[u8]) {
        use std::io::*;
        use std::process::*;

        let mut cmd = Command::new("shasum")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdin = cmd.stdin.as_mut().unwrap();

        stdin.write_all(value).unwrap();
        stdin.flush().unwrap();

        let output = String::from_utf8(cmd.wait_with_output().unwrap().stdout).unwrap();

        let expected_hash = &output.split(' ').next().unwrap();

        assert_sha1(value, expected_hash)
    }

    #[test]
    fn toml() {
        let toml = include_bytes!("../Cargo.toml");
        assert_same_as_command(toml);
    }

    #[test]
    fn src() {
        let files = [
            include_bytes!("./main.rs").as_slice(),
            include_bytes!("./sha1.rs").as_slice(),
        ];

        files.into_iter().for_each(assert_same_as_command)
    }

    #[test]
    fn lock() {
        let file = include_bytes!("../Cargo.lock");
        assert_same_as_command(file);
    }
}
