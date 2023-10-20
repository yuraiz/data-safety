use std::io::*;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use sha1::Sha1Context;

mod sha1;

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    input: PathBuf,
}

fn main() -> Result<()> {
    // println!("Hello, world!");
    let args = Args::parse();

    let mut input = std::fs::File::open(args.input).context("Couldn't open input file")?;

    let len = input.metadata().map(|m| m.len()).unwrap_or(u64::MAX);

    let buff_size = len.min(1u64 << 32) as usize;

    let mut buff = vec![0; buff_size];

    let mut context = Sha1Context::new();

    let hash = loop {
        let read = input.read(&mut buff)?;

        let remainder = context.process_chunks(&buff[..read]);

        if read == 0 || !remainder.is_empty() {
            break context.finish(remainder);
        }
    };

    println!("{}", sha1::format_big_hex(hash));

    Ok(())
}
