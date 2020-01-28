use std::mem::size_of;
use super::from_bytes::*;

//Default implementations of from bytes for integers 
//BigEndian only
macro_rules! be_impl {
    ($ty: ty, $size: tt) => {

        impl Bitsize for $ty 
        {
            const SIZE_IN_BITS : usize = ($size * 8); 
        }

        impl FromBytes<(), [u8; $size]> for $ty where Self : Sized
        {
            #[inline]
            fn from_bytes(input : [u8; $size]) -> std::result::Result<$ty,()>
            {
                Ok(<$ty>::from_be_bytes(input))
            }
        }
    }
}

be_impl!(u8, (size_of::<u8>()));
be_impl!(u16, (size_of::<u16>()));
be_impl!(u32, (size_of::<u32>()));
be_impl!(u64, (size_of::<u64>()));
be_impl!(u128, (size_of::<u128>()));

be_impl!(i8,  (size_of::<i8>()));
be_impl!(i16, (size_of::<i16>()));
be_impl!(i32, (size_of::<i32>()));
be_impl!(i64, (size_of::<i64>()));
be_impl!(i128, (size_of::<i128>()));

be_impl!(usize, (size_of::<usize>()));
be_impl!(isize, (size_of::<isize>()));