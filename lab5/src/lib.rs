#![allow(unused)]

use std::fs::File;

mod gost;
mod my_hasher;
mod sha1;

use anyhow::Result;
use my_hasher::MyHasher;
use std::io::Read;

pub fn gost_hash(mut input: File) -> Result<[u64; 4]> {
    let mut hash_context = gost::GostContext::new(Default::default());
    let len = input.metadata().map(|m| m.len()).unwrap_or(u64::MAX);

    let buff_size = len.min(1u64 << 32) as usize;

    let mut buff = vec![0; buff_size];

    let hash = loop {
        let read = input.read(&mut buff)?;

        let remainder = hash_context.process_chunks(&buff[..read]);

        if read == 0 || !remainder.is_empty() {
            break hash_context.finish(remainder);
        }
    };

    Ok(hash)
}
