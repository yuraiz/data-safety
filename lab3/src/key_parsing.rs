use std::fs::File;
use std::io::Read;

use num::BigInt;

use crate::rabin::PrivateKeyInfo;

pub(crate) fn parse_public_key(mut file: File) -> anyhow::Result<BigInt> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let key = buf.parse()?;
    Ok(key)
}

pub(crate) fn parse_private_key(mut file: File) -> anyhow::Result<PrivateKeyInfo> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let numbers: Vec<BigInt> = buf.split(' ').flat_map(|s| s.parse()).take(3).collect();

    let Ok([p, q]): Result<[BigInt; 2], _> = numbers.try_into() else {
        anyhow::bail!("the private key must contain of two 64-bit integers");
    };

    Ok(PrivateKeyInfo::new(p, q))
}
