pub trait FromSlice<E> where Self : Sized
{
    //return self and a the number of bytes read as a usize
    fn from_reader(slice : &mut &[u8]) -> std::result::Result<Self, E>;
}