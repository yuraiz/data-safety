use std::ops::Deref;
use std::ops::DerefMut;

pub struct BlockAlignedBuff {
    buff: Box<[u64]>,
    filled: usize,
}

impl BlockAlignedBuff {
    pub fn new(bytes: usize) -> Self {
        let blocks = (bytes + 8 - bytes % 8) / 8;
        let buff = vec![0; blocks].into_boxed_slice();
        Self { buff, filled: 0 }
    }

    pub fn read_bytes<R: std::io::Read>(&mut self, read: &mut R) -> std::io::Result<usize> {
        let mut bytes = unsafe { self.buff.align_to_mut().1 };

        let len = read.read(&mut bytes)?;

        if (len % 8) != 0 {
            bytes[len..(len + 8 - len % 8)].fill(0);
        }

        self.filled = len / 8 + (len % 8).min(1);

        Ok(len)
    }
}

impl AsRef<[u8]> for BlockAlignedBuff {
    fn as_ref(&self) -> &[u8] {
        unsafe { self.buff[..self.filled].align_to().1 }
    }
}

impl AsRef<[u64]> for BlockAlignedBuff {
    fn as_ref(&self) -> &[u64] {
        &self.buff[..self.filled]
    }
}

impl AsMut<[u64]> for BlockAlignedBuff {
    fn as_mut(&mut self) -> &mut [u64] {
        &mut self.buff[..self.filled]
    }
}

impl Deref for BlockAlignedBuff {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl DerefMut for BlockAlignedBuff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
