pub trait MyHasher: Sized {
    type Output: std::fmt::Debug;

    const CHUNK_SIZE: usize;

    fn process_chunk(&mut self, chunk: &[u8]);

    fn finish(self, remainder: &[u8]) -> Self::Output;

    fn process_chunks<'a>(&mut self, message: &'a [u8]) -> &'a [u8] {
        let chunks = message.chunks_exact(Self::CHUNK_SIZE);
        let remainder = chunks.remainder();

        chunks.for_each(|c| self.process_chunk(c));

        remainder
    }

    fn process_to_end(mut self, message: &[u8]) -> Self::Output {
        let remainder = self.process_chunks(message);
        self.finish(remainder)
    }
}
