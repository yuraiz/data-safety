use crate::rabin::{decrypt_block, encrypt_block, PrivateKeyInfo};

/// Tries to find a char inside of a set of blocks
fn find_char(blocks: [u64; 4]) -> char {
    fn try_into_char(block: u64) -> Option<char> {
        let num: u32 = block.try_into().ok()?;
        char::from_u32(num)
    }

    blocks
        .into_iter()
        .flat_map(try_into_char)
        .next()
        .unwrap_or(char::REPLACEMENT_CHARACTER)
}

pub fn encrypt<R: std::io::Read, W: std::io::Write>(
    input: R,
    output: W,
    key: u128,
) -> std::io::Result<()> {
    use std::io::*;

    let input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    let encrypted_endl = &encrypt_block('\n' as u64, key).to_le_bytes();

    for line in input.lines().flatten() {
        for ch in line.chars() {
            let encrypted = encrypt_block(ch as u64, key);
            output.write_all(&encrypted.to_le_bytes())?;
        }
        output.write_all(encrypted_endl)?;
    }

    Ok(())
}

pub fn decrypt<R: std::io::Read, W: std::io::Write>(
    mut input: R,
    output: W,
    key_info: PrivateKeyInfo,
) -> std::io::Result<()> {
    use std::io::*;

    let mut buff = block_buffer::BlockBuffer::<u128>::new(1024);
    let mut output = BufWriter::new(output);

    buff.read_bytes_from(&mut input)?;

    let mut ch_buff = [0; 4];

    for input in buff.as_blocks() {
        let blocks = decrypt_block(*input, key_info);
        let ch = find_char(blocks);
        output.write_all(ch.encode_utf8(&mut ch_buff).as_bytes())?;
    }

    Ok(())
}
