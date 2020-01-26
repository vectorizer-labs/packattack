#[derive(Fail, Debug)]
pub enum PackattackParserError
{
    #[fail(display = "No match was found for the type {}",_0)]
    NoEnumMatch(String)
}