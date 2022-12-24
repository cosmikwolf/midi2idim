use nom::{
    IResult,
    branch::alt,
    multi::{many0, many1},
    combinator::{map_res, recognize},
    sequence::{preceded, terminated},
    character::complete::{char, one_of},
    bytes::complete::tag,
  };

pub fn hex_string_decode(input: &str) -> IResult<&str, i64> {
  map_res(
    preceded(
      alt((tag("0x"), tag("0X"))),
      recognize(
        many1(
          terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
        )
      )
    ),
    |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 16)
  )(input)
}  