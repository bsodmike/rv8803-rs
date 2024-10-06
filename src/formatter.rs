/// Format bytes using a formatter.
use core::fmt::{self};
use core::str;

pub struct ByteMutWriter<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> ByteMutWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        ByteMutWriter { buf, cursor: 0 }
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.buf[0..self.cursor]).expect("Unable to create &str from buf")
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn clear(&mut self) {
        self.cursor = 0;
    }

    pub fn len(&self) -> usize {
        self.cursor
    }

    pub fn empty(&self) -> bool {
        self.cursor == 0
    }

    pub fn full(&self) -> bool {
        self.capacity() == self.cursor
    }
}

impl fmt::Write for ByteMutWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let cap = self.capacity();
        self.buf[self.cursor..cap]
            .iter_mut()
            .zip(s.as_bytes().iter())
            .for_each(|(i, &b)| {
                *i = b;
            });

        self.cursor = usize::min(cap, self.cursor + s.as_bytes().len());
        Ok(())
    }
}
