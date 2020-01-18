pub trait Bitsize
{
    const SIZE_IN_BITS : usize = 0;
}

pub trait FromBits<E> : Sized
{
    fn from_bits(byte : u8) -> std::result::Result<Self, ()>;
}

impl Bitsize for bool { const SIZE_IN_BITS : usize = 1; }
impl FromBits<()> for bool
{
    fn from_bits(byte : u8) -> std::result::Result<Self, ()> { Ok(byte != 0) }
}

impl Bitsize for () { const SIZE_IN_BITS : usize = 0; }
impl FromBits<()> for ()
{
    fn from_bits(_byte : u8) -> std::result::Result<Self, ()> { Ok(()) }
} 