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

fn apply_rules(template: &str, rules: Rules, num_times: usize) -> u64 {
    let rules = rules.0;
    let last_char = template.chars().last().unwrap();

    let mut pair_counts = HashMap::new();

    for (l, r) in template.chars().tuple_windows() {
        *pair_counts.entry((l, r)).or_insert(0) += 1;
    }

    let mut new_pair_counts = HashMap::new();

    for _ in 0..num_times {
        new_pair_counts.clear();

        for (&(l, r), &count) in &pair_counts {
            let c = rules[&(l, r)];
            *new_pair_counts.entry((l, c)).or_insert(0) += count;
            *new_pair_counts.entry((c, r)).or_insert(0) += count;
        }

        std::mem::swap(&mut pair_counts, &mut new_pair_counts);
    }

    let mut letters = HashMap::new();
    for ((l, _), count) in pair_counts {
        *letters.entry(l).or_insert(0) += count;
    }
    *letters.entry(last_char).or_insert(0) += 1;

    let (min_count, max_count) = letters
        .values()
        .fold((u64::MAX, 0), |(min, max), &c| (min.min(c), max.max(c)));

    max_count - min_count
}

fn parse(input: &str) -> (&str, Rules) {
    super::utils::parse(items, input)
}

fn items(input: &str) -> IResult<&str, (&str, Rules)> {
    use nom::character::complete::multispace1;

    separated_pair(template, multispace1, rules)(input)
}

fn template(input: &str) -> IResult<&str, &str> {
    use nom::character::complete::not_line_ending;

    terminated(not_line_ending, line_ending)(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rules(HashMap<(char, char), char>);

impl Rules {
    fn rule_pairs(&self, pair: (char, char)) -> Option<((char, char), (char, char))> {
        self.0.get(&pair).map(|&c| ((pair.0, c), (c, pair.1)))
    }
}

fn rules(input: &str) -> IResult<&str, Rules> {
    use nom::{
        combinator::{map, opt},
        multi::fold_many1,
    };

    map(
        fold_many1(
            terminated(rule, opt(line_ending)),
            HashMap::new,
            |mut map, (left, right)| {
                map.insert(left, right);
                map
            },
        ),
        Rules,
    )(input)
}

fn rule(input: &str) -> IResult<&str, ((char, char), char)> {
    use nom::{bytes::complete::tag, character::complete::anychar, sequence::pair};

    separated_pair(pair(anychar, anychar), tag(" -> "), anychar)(input)
}
