use std::fs::File;
use std::io::Read;

use crate::rabin::PrivateKeyInfo;

pub(crate) fn parse_public_key(mut file: File) -> anyhow::Result<u128> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let key: u128 = buf.parse()?;
    if key < u64::MAX as u128 {
        anyhow::bail!("The key is too small, don't use it");
    }
    Ok(key)
}

pub(crate) fn parse_private_key(mut file: File) -> anyhow::Result<PrivateKeyInfo> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let numbers: Vec<u64> = buf.split(' ').flat_map(|s| s.parse()).take(3).collect();

    let Ok([p, q]): Result<[u64; 2], _> = numbers.try_into() else {
        anyhow::bail!("the private key must contain of two 64-bit integers");
    };

    Ok(PrivateKeyInfo::new(p, q))
}
