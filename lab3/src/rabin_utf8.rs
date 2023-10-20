use crate::rabin::{self, PrivateKeyInfo};
use anyhow::Ok;
use num::BigInt;
use std::io::*;

fn next_substring<R: std::io::Read>(buf_reader: &mut BufReader<R>) -> anyhow::Result<String> {
    let bytes = buf_reader.fill_buf()?;

    let mut len = bytes.len().min(8);

    while std::str::from_utf8(&bytes[..len]).is_err() {
        len -= 1;
    }

    let bytes = bytes[..len].to_owned();

    buf_reader.consume(len);

    Ok(String::from_utf8(bytes)?)
}

pub fn encrypt<R: std::io::Read, W: std::io::Write>(
    input: R,
    output: W,
    key: BigInt,
) -> anyhow::Result<()> {
    let mut input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    loop {
        let substring = next_substring(&mut input)?;

        if substring.is_empty() {
            break;
        }

        let number = BigInt::from_bytes_le(num::bigint::Sign::Plus, substring.as_bytes());

        let number = rabin::encrypt(number, &key);

        let bytes = number.to_bytes_le().1;

        output.write_all(&(bytes.len() as u64).to_le_bytes())?;
        output.write_all(&bytes)?;
    }

    Ok(())
}

pub fn decrypt<R: std::io::Read, W: std::io::Write>(
    input: R,
    output: W,
    key_info: PrivateKeyInfo,
) -> anyhow::Result<()> {
    let mut input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    let mut buff = [0; 2048];

    let mut iteration = || -> anyhow::Result<bool> {
        let mut len_buf = 0u64.to_le_bytes();
        match input.read_exact(&mut len_buf) {
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Ok(true);
            }
            other => other?,
        };

        let len = u64::from_le_bytes(len_buf) as usize;

        input.read_exact(&mut buff[..len])?;
        let number = BigInt::from_bytes_le(num::bigint::Sign::Plus, &buff[..len]);

        let numbers = rabin::decrypt(number, &key_info);

        let valid_text = numbers
            .into_iter()
            .flat_map(|number| {
                let vec = number.to_bytes_le().1;
                String::from_utf8(vec).ok()
            })
            .next()
            .ok_or(anyhow::anyhow!("No valid decryption results"))?;

        output.write_all(valid_text.as_ref())?;

        Ok(false)
    };

    loop {
        if iteration()? {
            break Ok(());
        }
    }
}
