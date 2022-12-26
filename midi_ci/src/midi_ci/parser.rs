use nom::{
    bytes::complete::{tag, take_until1},
    combinator::value,
    error::ParseError,
    sequence::tuple,
    IResult,
};

use crate::midi_ci::defs::midi_ci_msg;

pub fn universal_sysex<'a, E: ParseError<&'a [u8]>>(i: &[u8]) -> IResult<&[u8], (&[u8],&[u8],&[u8]), E> {
    tuple((
        tag(&[
            midi_ci_msg::SYSTEM_EXCLUSIVE_START,
            midi_ci_msg::UNIVERSAL_SYSTEM_EXCLUSIVE,
        ]),
        take_until1(&[midi_ci_msg::END_UNIVERSAL_SYSTEM_EXCLUSIVE, 2]),
        tag(&[midi_ci_msg::END_UNIVERSAL_SYSTEM_EXCLUSIVE]),
    ))(i)
}
