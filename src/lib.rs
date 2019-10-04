mod from_bytes;
mod from_byte;

pub trait FromBytes: Sized
{
    fn read_from_bytes(bytes: &[u8], count : &mut usize) -> Self;
}

pub trait FromByte: Sized
{
    fn read_from_byte(byte : &u8, mask : u8) -> Self;

    const bitmask : u8;
    const size_in_bits: u8;
}