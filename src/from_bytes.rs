use super::*;

impl FromBytes for u16
{
    fn read_from_bytes(bytes: &[u8], count : &mut usize) -> u16
    {
        let double_byte = u16::from_be_bytes([bytes[*count],bytes[*count + 1]]);
        //increment the counter to show we've read
        *count += 2;
        double_byte
    }
}

impl FromBytes for String
{ 
    fn read_from_bytes(bytes: &[u8], count : &mut usize) -> String
    {
        let length : usize = <u16>::read_from_bytes(bytes, count) as usize;
        let streng = String::from_utf8(bytes[*count..(*count + length)].to_vec()).unwrap();
        
        //increment the counter to show we've read
        *count += length as usize;

        //I know its streng and not string. It makes me happy. Don't change it.
        streng
    }
}

impl FromBytes for u8
{
    #[inline(always)]
    fn read_from_bytes(bytes: &[u8], count : &mut usize) -> u8
    {
        bytes[*count]
    }
}