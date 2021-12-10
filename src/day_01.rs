use itertools::Itertools;
use nom::IResult;

pub fn star_1(data: String) {
    let values = parse(&data);
    let count = values.iter().tuple_windows().filter(|(a, b)| a < b).count();
    println!("{}", count);
}

pub fn star_2(data: String) {
    let values = parse(&data);
    let sums = values.iter().tuple_windows().map(|(a, b, c)| a + b + c);
    let count = sums.tuple_windows().filter(|(a, b)| a < b).count();
    println!("{}", count);
}

fn parse(data: &str) -> Vec<u32> {
    super::utils::parse(numbers, data)
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    use super::utils::lines;
    use nom::character::complete::{u32 as u32_};

    lines(u32_)(input)
}
