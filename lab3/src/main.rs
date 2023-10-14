mod key_parsing;
mod mics;
mod rabin;
mod rabin_utf8;

use std::{fs::File, path::PathBuf};

use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    decrypt: bool,

    key: PathBuf,

    input: PathBuf,
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let key = File::open(args.key).context("Couldn't open the key file")?;
    let input = File::open(args.input).context("Couldn't open the input file")?;
    let output = File::create(args.output.clone()).context("Couldn't create the output file")?;

    if args.decrypt {
        let key_info = key_parsing::parse_private_key(key)?;
        rabin_utf8::decrypt(input, output, key_info).context("Error during decripton")?;
    } else {
        let key = key_parsing::parse_public_key(key)?;
        rabin_utf8::encrypt(input, output, key).context("Error during encryption")?;
    }

    Ok(())
}
