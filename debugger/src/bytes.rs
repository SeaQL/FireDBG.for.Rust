use sea_streamer::Buffer;
use std::str::Utf8Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
/// Blob of Bytes. Right now it is backed by a `Vec<u8>`.
///
/// But if this becomes a bottleneck we should back it with a custom allocator.
pub struct Bytes(Vec<u8>);

impl Default for Bytes {
    fn default() -> Self {
        Self::new()
    }
}

impl Bytes {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, i: usize) -> u8 {
        self.0[i]
    }

    pub fn slice(&self, p: usize, q: usize) -> &[u8] {
        &self.0[p..q]
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.0.push(byte);
    }

    pub fn push_bytes(&mut self, mut bytes: Self) {
        self.0.append(&mut bytes.0);
    }

    pub fn push_slice(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }

    pub fn push_string(&mut self, string: String) {
        self.0.append(&mut string.into_bytes());
    }

    pub fn push_str(&mut self, str: &str) {
        self.0.extend_from_slice(str.as_bytes());
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

macro_rules! impl_into_bytes {
    ($ty: ty) => {
        impl From<$ty> for Bytes {
            fn from(v: $ty) -> Self {
                v.to_ne_bytes().to_vec().into()
            }
        }
    };
}

impl_into_bytes!(u8);
impl_into_bytes!(i8);
impl_into_bytes!(u16);
impl_into_bytes!(i16);
impl_into_bytes!(u32);
impl_into_bytes!(i32);
impl_into_bytes!(u64);
impl_into_bytes!(i64);
impl_into_bytes!(u128);
impl_into_bytes!(i128);
impl_into_bytes!(usize);
impl_into_bytes!(isize);
impl_into_bytes!(f32);
impl_into_bytes!(f64);

impl Buffer for Bytes {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.0)
    }
}
