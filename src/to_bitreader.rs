#[async_trait]
pub trait ToBitReader<E, R> 
    where Self : Sized,
    R : Read + std::marker::Unpin + std::marker::Send
{
    async fn to_bitreader(self, reader : &mut BitReader<R>) -> std::result::Result<(), E>;
}