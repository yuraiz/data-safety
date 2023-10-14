use std::ops::Deref;
use std::ops::DerefMut;

pub unsafe trait Block: Default + Copy {}

unsafe impl Block for u16 {}
unsafe impl Block for u32 {}
unsafe impl Block for u64 {}
unsafe impl Block for u128 {}

// A buffer you can write bytes to and interpret them as blocks
pub struct BlockBuffer<T: Block> {
    buff: Box<[T]>,
    filled: usize,
}

impl<T: Block> BlockBuffer<T> {
    const BLOCK_SIZE: usize = std::mem::size_of::<T>();

    /// Creates a new buffer allocating at least `size` bytes
    pub fn new(size: usize) -> Self {
        let blocks = (size + 16 - size % 16) / 16;
        let buff = vec![T::default(); blocks].into_boxed_slice();
        Self { buff, filled: 0 }
    }

    /// Reads bytes writing them to the buffer and fills remaining bytes with zeroes
    pub fn read_bytes_from<R: std::io::Read>(&mut self, read: &mut R) -> std::io::Result<usize> {
        let bytes = unsafe { self.buff.align_to_mut().1 };

        let len = read.read(bytes)?;

        let bs = Self::BLOCK_SIZE;

        if (len % bs) != 0 {
            bytes[len..(len + bs - len % bs)].fill(0);
        }

        self.filled = len / bs + (len % bs).min(1);

        Ok(len)
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { self.as_blocks().align_to().1 }
    }

    pub fn as_blocks(&self) -> &[T] {
        &self.buff[..self.filled]
    }

    pub fn as_blocks_mut(&mut self) -> &mut [T] {
        &mut self.buff[..self.filled]
    }
}

impl<T: Block> AsRef<[u8]> for BlockBuffer<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<T: Block> AsRef<[T]> for BlockBuffer<T> {
    fn as_ref(&self) -> &[T] {
        &self.as_blocks()
    }
}

impl<T: Block> AsMut<[T]> for BlockBuffer<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_blocks_mut()
    }
}

impl<T: Block> Deref for BlockBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: Block> DerefMut for BlockBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
