mod block_aligned_buff;
mod cfm;
mod simple_swap;

use std::{io::*, path::PathBuf};

use block_aligned_buff::BlockAlignedBuff;
use cfm::*;
use clap::Parser;

const MAX_BUFF_SIZE: u64 = 1 << 30;
const INIT_BLOCK: u64 = 0xBADF00D;
const DEFAULT_KEY: [u32; 8] = [34, 42, 12, 53, 23, 23, 54, 34];

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(short, long)]
    decrypt: bool,

    #[arg(value_name = "FILE")]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long)]
    key_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut file = std::fs::File::open(args.input)?;

    let mut out_file = std::fs::File::create(args.output)?;

    let key = if let Some(key_file_path) = args.key_file {
        let mut key_file = std::fs::File::open(key_file_path)?;

        let mut buff = [0u8; 32];

        key_file.read(&mut buff)?;

        buff.chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap()
    } else {
        DEFAULT_KEY
    };

    let transform_func = if args.decrypt {
        cipher_feedback_mode_block_decrypt
    } else {
        cipher_feedback_mode_block
    };

    let len = file.metadata().map(|m| m.len()).unwrap_or(MAX_BUFF_SIZE);
    let buff_size = len.min(MAX_BUFF_SIZE) as usize;

    let mut buff = BlockAlignedBuff::new(buff_size);

    let mut prev = INIT_BLOCK;
    loop {
        if buff.read_bytes(&mut file)? == 0 {
            break;
        }

        prev = transform_func(prev, &mut buff, key);

        out_file.write(buff.as_ref())?;
    }

    out_file.set_len(len)?;

    Ok(())
}
