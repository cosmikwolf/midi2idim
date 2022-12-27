
use nom::error::Error;
use nom::bits;
use nom::IResult;
use nom::sequence::tuple;

pub fn parse_u7lsb_to_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let bits: (&[u8], (u8, u8, u8, u8)) = bits::<_, _, Error<(&[u8], usize)>, _, _>(tuple((
        bits::streaming::take(8usize),
        bits::streaming::take(8usize),
        bits::streaming::take(8usize),
        bits::streaming::take(8usize),
    )))(input)?;
    Ok((
        input,
        u32::from_be_bytes([0, 0, 0, bits.1 .0]).rotate_left(0)
            + u32::from_be_bytes([0, 0, 0, bits.1 .1]).rotate_left(7)
            + u32::from_be_bytes([0, 0, 0, bits.1 .2]).rotate_left(14)
            + u32::from_be_bytes([0, 0, 0, bits.1 .3]).rotate_left(21),
    ))
}
pub fn parse_u7lsb_to_u16(input: &[u8]) -> IResult<&[u8], u16> {
    let bits: (&[u8], (u8, u8)) = bits::<_, _, Error<(&[u8], usize)>, _, _>(tuple((
        bits::streaming::take(8usize),
        bits::streaming::take(8usize),
    )))(input)?;
    Ok((
        input,
        u16::from_be_bytes([0, bits.1 .0]).rotate_left(0)
            + u16::from_be_bytes([ 0, bits.1 .1]).rotate_left(7)
    ))
}