use crate::error::MikuError;

pub trait AsBytes {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, MikuError> where Self: Sized;
}
