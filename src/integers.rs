use std::mem::size_of;
use super::from_bytes::*;

//Const generics are kinda broken right now so we define these
//constants as a work around for now
const SIZE_U8 : usize = size_of::<u8>();
const SIZE_U16 : usize  = size_of::<u16>();
const SIZE_U32 : usize = size_of::<u32>();
const SIZE_U64 : usize = size_of::<u64>();
const SIZE_U128 : usize = size_of::<u128>();

const SIZE_I8 : usize = size_of::<i8>();
const SIZE_I16 : usize = size_of::<i16>();
const SIZE_I32 : usize = size_of::<i32>();
const SIZE_I64 : usize = size_of::<i64>();
const SIZE_I128 : usize = size_of::<i128>();

const SIZE_USIZE : usize = size_of::<usize>();
const SIZE_ISIZE : usize = size_of::<isize>();

//Default implementations of from bytes for integers 
//BigEndian only
macro_rules! be_impl {
    ($ty: ty, $size: tt) => {

        impl Bitsize<[u8; $size]> for $ty 
        {
            const BUFFER : [u8; $size] = [0; $size];
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



be_impl!(u8, SIZE_U8);

be_impl!(u16, SIZE_U16);
be_impl!(u32, SIZE_U32);
be_impl!(u64, SIZE_U64);
be_impl!(u128, SIZE_U128);

be_impl!(i8,  SIZE_I8);
be_impl!(i16, SIZE_I16);
be_impl!(i32, SIZE_I32);
be_impl!(i64, SIZE_I64);
be_impl!(i128, SIZE_I128);

be_impl!(usize, SIZE_USIZE);
be_impl!(isize, SIZE_ISIZE);