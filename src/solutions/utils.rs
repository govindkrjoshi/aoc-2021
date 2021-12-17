use nom::{
    character::complete::digit1,
    combinator::{map_res, recognize},
    IResult,
};

#[allow(dead_code)]
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(recognize(digit1), str::parse)(input)
}
