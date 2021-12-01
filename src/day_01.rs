use itertools::Itertools;

pub fn star_1(data: String) {
    let values = parse(&data);
    let answer = values
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    println!("{}", answer);
}

pub fn star_2(data: String) {
    let values = parse(&data);
    let answer = values
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();
    println!("{:?}", answer);
}

fn parse(data: &str) -> Vec<u32> {
    data.lines()
        .map(str::trim)
        .filter_map(|line| line.parse::<u32>().ok())
        .collect()
}
