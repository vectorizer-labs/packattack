use super::*;

pub trait FromBytes<E, B> : Sized + Bitsize
{
    fn from_bytes(bytes : B) -> std::result::Result<Self, E>;
}

impl Bitsize for () 
{
    const SIZE_IN_BITS : usize = 8; 
}

impl FromBytes<(), [u8; 1]> for ()
{
    #[inline]
    fn from_bytes(_byte : [u8; 1]) -> std::result::Result<Self, ()> { Ok(()) }
}

impl Bitsize for Vec<u8>
{
    const SIZE_IN_BITS : usize = 0;
}

impl FromBytes<(), &mut &[u8]> for Vec<u8> where Self : Sized
{
    #[inline]
    fn from_bytes(bytes : &mut &[u8]) -> std::result::Result<Vec<u8>,()>
    {
        Ok(bytes.to_vec())
    }
}

/*
//T has to implement FromReader because the slice len needs to be updated inside the call
//to from_reader so we know how many bytes remain
impl<T> FromBytes<(), &mut &[u8]> for Vec<T> where Self : Sized, T : Sized + super::from_reader::FromReader<(),R>, R : Read + std::marker::Unpin + std::marker::Send + std::iter::ExactSizeIterator
{
    fn from_bytes(bytes : R) -> std::result::Result<Vec<T>,()>
    {
        let mut vars : Vec<T> = Vec::new();

        while bytes.len() > 0
        {
            vars.push(block_on(<T>::from_reader(&mut bytes).await?));
        }

        Ok(vars)
    }
}*/

/*
#[inline]
fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
    if buf.len() > self.len() {
        return Err(Error::new(ErrorKind::UnexpectedEof,
                                "failed to fill whole buffer"));
    }
    let (a, b) = self.split_at(buf.len());

    // First check if the amount of bytes we want to read is small:
    // `copy_from_slice` will generally expand to a call to `memcpy`, and
    // for a single byte the overhead is significant.
    if buf.len() == 1 {
        buf[0] = a[0];
    } else {
        buf.copy_from_slice(a);
    }

    *self = b;
    Ok(())
}
*/