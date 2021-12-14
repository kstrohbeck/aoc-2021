use itertools::Itertools;
use nom::{
    character::complete::line_ending,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

pub fn star_1(data: String) {
    let (template, rules) = parse(&data);
    let count = apply_rules(template, rules, 10);
    println!("{}", count);
}

pub fn star_2(data: String) {
    let (template, rules) = parse(&data);
    let count = apply_rules(template, rules, 40);
    println!("{}", count);
}

fn apply_rules(template: &str, rules: HashMap<(char, char), char>, num_times: usize) -> u64 {
    let last_char = template.chars().last().unwrap();
    let mut pair_counts = rules
        .keys()
        .zip(std::iter::repeat(0))
        .collect::<HashMap<_, _>>();

    for (l, r) in template.chars().tuple_windows() {
        *pair_counts.get_mut(&(l, r)).unwrap() += 1;
    }

    for _ in 0..40 {
        let mut new_pair_counts = rules
            .keys()
            .zip(std::iter::repeat(0))
            .collect::<HashMap<_, _>>();
        for ((l, r), c) in &rules {
            let count = pair_counts[&(*l, *r)];
            *new_pair_counts.get_mut(&(*l, *c)).unwrap() += count;
            *new_pair_counts.get_mut(&(*c, *r)).unwrap() += count;
        }

        pair_counts = new_pair_counts;
    }

    let mut letters = HashMap::new();
    for ((l, _), count) in pair_counts {
        *letters.entry(l).or_insert(0) += count;
    }
    *letters.entry(&last_char).or_insert(0) += 1;

    let mut max_count = 0;
    let mut min_count = u64::MAX;

    for &count in letters.values() {
        if count > max_count {
            max_count = count;
        }
        if count < min_count {
            min_count = count;
        }
    }

    max_count - min_count
}

fn parse(input: &str) -> (&str, HashMap<(char, char), char>) {
    super::utils::parse(items, input)
}

fn items(input: &str) -> IResult<&str, (&str, HashMap<(char, char), char>)> {
    use nom::character::complete::multispace1;

    separated_pair(template, multispace1, rules)(input)
}

fn template(input: &str) -> IResult<&str, &str> {
    use nom::character::complete::not_line_ending;

    terminated(not_line_ending, line_ending)(input)
}

fn rules(input: &str) -> IResult<&str, HashMap<(char, char), char>> {
    use nom::{combinator::opt, multi::fold_many1};

    fold_many1(
        terminated(rule, opt(line_ending)),
        HashMap::new,
        |mut map, (left, right)| {
            map.insert(left, right);
            map
        },
    )(input)
}

fn rule(input: &str) -> IResult<&str, ((char, char), char)> {
    use nom::{bytes::complete::tag, character::complete::anychar, sequence::pair};

    separated_pair(pair(anychar, anychar), tag(" -> "), anychar)(input)
}
