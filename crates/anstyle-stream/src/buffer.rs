/// In-memory [`RawStream`][crate::RawStream]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Buffer(Vec<u8>);

impl Buffer {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Buffer {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "auto")]
impl is_terminal::IsTerminal for Buffer {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

impl std::io::Write for Buffer {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.extend(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}