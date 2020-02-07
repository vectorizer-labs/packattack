use super::*;

pub trait FromBits<E> : Sized + Bitsize
{
    fn from_bits(byte : u8) -> std::result::Result<Self, E>;
}

impl Bitsize for bool 
{ 
    const SIZE_IN_BITS : usize = 1; 
}

impl FromBits<()> for bool
{
    #[inline]
    fn from_bits(byte : u8) -> std::result::Result<Self, E>; { Ok(byte != 0) }
}