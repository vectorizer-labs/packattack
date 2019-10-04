use super::*;
//These types are measured relative to the byte they are in 
//Whatever

//we need to increment count in the from bytes method everytime
//one of these hits the 8th bit

//TODO: add #[inline] and see if it helps at all or is just a waste 
impl FromByte for bool
{
    #[inline(always)]
    fn read_from_byte(byte : &u8, mask : u8) -> bool
    {
        (byte & mask) != 0
    }
    
    const bitmask : u8 = 0b10000000;
    const size_in_bits : u8 = 1;
}