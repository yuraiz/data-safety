mod gost;
mod my_hasher;
mod sha1;

use std::path::PathBuf;
use std::{fs::File, io::*};

use anyhow::{Context, Result};
use clap::Parser;

use my_hasher::MyHasher;

use sha1::Sha1Context;

#[derive(Debug, Parser)]
struct Args {
    #[arg(help = "File to compute hash for")]
    input: PathBuf,

    #[arg(short, long, help = "Use gost insead of sha1")]
    gost: bool,
}

fn compute_hash<H, O>(mut input: File, mut hash_context: H) -> Result<O>
where
    H: MyHasher<Output = O>,
{
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

fn main() -> Result<()> {
    let args = Args::parse();

    let input = File::open(args.input).context("Couldn't open input file")?;

    if args.gost {
        let hash = compute_hash(input, gost::GostContext::new(Default::default()))?;
        println!("{}", sha1::format_big_hex(hash));
    } else {
        let hash = compute_hash(input, Sha1Context::new())?;
        println!("{}", sha1::format_big_hex(hash));
    }

    Ok(())
}
