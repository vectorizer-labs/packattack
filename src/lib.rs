pub use {
    async_trait::async_trait,
    bitreader_async::BitReader,
    async_std::io::Read
};

use bitreader_async::error::*;

#[async_trait]
pub trait FromBitReader<E, R> 
    where Self : Sized,
    R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> std::result::Result<Self, E>;
}

#[async_trait]
pub trait FromBitReaderWithLength<E, R> 
    where Self : Sized,
    R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader_with_length(reader : &mut BitReader<R>, len : usize) -> std::result::Result<Self, E>;
}

#[async_trait]
impl<R> FromBitReader<BitReaderError, R> for () where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(_reader : &mut BitReader<R>) -> Result<()>
    {
        Ok(())
    }
}

#[async_trait]
impl<R> FromBitReader<BitReaderError, R> for bool where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<bool>
    {
        Ok(reader.read_be_bits(1).await? != 0)
    }
}

#[async_trait]
impl<R> FromBitReader<BitReaderError, R> for u8 where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<u8>
    {
        Ok(reader.read_aligned_be::<u8>().await?)
    }
}

#[async_trait]
impl<R> FromBitReader<BitReaderError, R> for u16 where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<u16>
    {
        Ok(reader.read_aligned_be::<u16>().await?)
    }
}

#[async_trait]
impl<R> FromBitReader<BitReaderError, R> for u32 where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<u32>
    {
        Ok(reader.read_aligned_be::<u32>().await?)
    }
}

#[async_trait]
impl<R> FromBitReaderWithLength<BitReaderError, R> for Vec<u8> where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader_with_length(reader : &mut BitReader<R>, len : usize) -> Result<Vec<u8>>
    {
        Ok(reader.read_u8_slice_aligned(len).await?)
    }
}