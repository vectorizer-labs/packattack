#[macro_use] extern crate failure;

pub mod from_reader;
pub mod from_bytes;
pub mod error;
pub mod integers;

pub use {
    packattack_derive::*,
    from_reader::*,
    from_bytes::*,
    error::*
};