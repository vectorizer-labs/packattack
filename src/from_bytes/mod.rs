pub trait ByteSize
{
    const SIZE_IN_BYTES : usize = 0;
}

pub trait FromBytes<E, T> : Sized + ByteSize
{
    fn from_bytes(slice : T) -> std::result::Result<Self, E>;
}

pub mod integers;
pub use integers::*;