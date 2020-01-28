pub use
{
    async_trait::async_trait,
    async_std::io::Read,
    async_std::prelude::*,
    std::convert::TryInto
};

#[async_trait]
pub trait FromReader<E, R> where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    //return self and a the number of bytes read as a usize
    async fn from_reader(reader : &mut R) -> std::result::Result<Self, E>;
}