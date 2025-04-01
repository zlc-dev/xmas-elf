pub trait Reader {
    fn len(&self) -> usize;

    fn read(&self, offset: usize, len: usize) -> &[u8];
}

impl Reader for [u8] {
    fn len(&self) -> usize {
        (self as &[u8]).len()
    }

    fn read(&self, offset: usize, len: usize) -> &[u8] {
        &self[offset..offset+len]
    }
}
