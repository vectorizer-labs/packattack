#[macro_use] extern crate failure;

pub mod from_reader;
pub mod from_bytes;
pub mod from_bits;
pub mod error;
pub mod integers;

pub use {
    packattack_derive::*,
    from_reader::*,
    from_bytes::*,
    error::*
};

pub trait Bitsize
{
    const SIZE_IN_BITS : usize = 0;
}

pub trait Bytesize
{
    const SIZE_IN_BYTES : usize = 0;
}