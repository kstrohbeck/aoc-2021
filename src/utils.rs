use nom::IResult;

pub fn u32_(input: &str) -> IResult<&str, u32> {
    use nom::{character::complete::digit1, combinator::map_res};

    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

pub fn u64_(input: &str) -> IResult<&str, u64> {
    use nom::{character::complete::digit1, combinator::map_res};

    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

pub fn usize_(input: &str) -> IResult<&str, usize> {
    use nom::{character::complete::digit1, combinator::map_res};

    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}
