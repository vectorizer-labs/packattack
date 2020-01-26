pub trait Bitsize<B>
{
    //this is the data type that will be read and passed to the function
    //should be zeroed out but doesn't really matter because the contents will be overwritten
    const BUFFER : B;

    const SIZE_IN_BITS : usize = 0;
}

pub trait FromBytes<E, B> : Sized + Bitsize<B>
{
    fn from_bytes(bytes : B) -> std::result::Result<Self, E>;
}

impl Bitsize<[u8; 1]> for bool 
{ 
    const BUFFER : [u8; 1] = [0; 1];
    const SIZE_IN_BITS : usize = 1; 
}

impl FromBytes<(), [u8; 1]> for bool
{
    fn from_bytes(bytes : [u8; 1]) -> std::result::Result<Self, ()> { Ok(bytes[0] != 0) }
}

impl Bitsize<[u8; 1]> for () 
{ 
    const BUFFER : [u8; 1] = [0; 1];
    const SIZE_IN_BITS : usize = 8; 
}

impl FromBytes<(), [u8; 1]> for ()
{
    fn from_bytes(_byte : [u8; 1]) -> std::result::Result<Self, ()> { Ok(()) }
} 