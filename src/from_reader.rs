pub use
{
    async_trait::async_trait,
    async_std::io::Read,
    async_std::prelude::*
}

#[async_trait]
pub trait FromReader<E, R> 
    where Self : Sized,
    R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut R) -> std::result::Result<Self, E>;
}

/*
#[async_trait]
pub trait FromReaderWithLength<E, R> 
    where Self : Sized,
    R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader_with_length(reader : &mut BitReader<R>, len : usize) -> std::result::Result<Self, E>;
}


#[async_trait]
impl<R> FromReader<BitReaderError, R> for () where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(_reader : &mut BitReader<R>) -> Result<()>
    {
        Ok(())
    }
}

#[async_trait]
impl<R> FromReader<BitReaderError, R> for bool where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut BitReader<R>) -> Result<bool>
    {
        Ok(reader.read_be_bits(1).await? != 0)
    }
}

#[async_trait]
impl<R> FromReader<BitReaderError, R> for u8 where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut BitReader<R>) -> Result<u8>
    {
        Ok(reader.read_aligned_be::<u8>().await?)
    }
}

#[async_trait]
impl<R> FromReader<BitReaderError, R> for u16 where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut BitReader<R>) -> Result<u16>
    {
        Ok(reader.read_aligned_be::<u16>().await?)
    }
}

#[async_trait]
impl<R> FromReader<BitReaderError, R> for u32 where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut BitReader<R>) -> Result<u32>
    {
        Ok(reader.read_aligned_be::<u32>().await?)
    }
}

#[async_trait]
impl<R> FromReaderWithLength<BitReaderError, R> for Vec<u8> where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader_with_length(reader : &mut BitReader<R>, len : usize) -> Result<Vec<u8>>
    {
        Ok(reader.read_u8_slice_aligned(len).await?)
    }
}*/