use itertools::iproduct;
use nom::{combinator::map, IResult};
use std::{fmt, iter::Sum, mem, ops::Add};

pub fn star_1(data: String) {
    let sf_nums = parse(&data);
    let mag = sf_nums.into_iter().sum::<SfNum>().magnitude();
    println!("{}", mag);
}

pub fn star_2(data: String) {
    let sf_nums = parse(&data);
    let max = iproduct!(sf_nums.iter(), sf_nums.iter())
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (a.clone() + b.clone()).magnitude())
        .max()
        .unwrap();
    println!("{}", max);
}

fn parse(input: &str) -> Vec<SfNum> {
    super::utils::parse(sf_nums, input)
}

fn sf_nums(input: &str) -> IResult<&str, Vec<SfNum>> {
    use nom::{character::complete::line_ending, multi::separated_list0};

    separated_list0(line_ending, sf_num)(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SfNum {
    Pair(Box<SfNum>, Box<SfNum>),
    Literal(u64),
}

impl SfNum {
    fn pair(left: SfNum, right: SfNum) -> Self {
        Self::Pair(Box::new(left), Box::new(right))
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
            Self::Literal(x) => *x,
        }
    }

    fn as_literal(&mut self) -> Option<&mut u64> {
        match self {
            Self::Literal(x) => Some(x),
            Self::Pair(_, _) => None,
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if !self.split() {
                break;
            }
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<u64>, Option<u64>)> {
        let (l, r) = match self {
            Self::Pair(l, r) => (l, r),
            Self::Literal(_) => return None,
        };

        if depth == 4 {
            let l = *l.as_literal().unwrap();
            let r = *r.as_literal().unwrap();

            mem::replace(self, Self::Literal(0));
            return Some((Some(l), Some(r)));
        }

        if let Some((a, b)) = l.explode(depth + 1) {
            if let Some(b) = b {
                r.add_leftmost(b);
            }
            return Some((a, None));
        }

        if let Some((a, b)) = r.explode(depth + 1) {
            if let Some(a) = a {
                l.add_rightmost(a);
            }
            return Some((None, b));
        }

        return None;
    }

    fn add_rightmost(&mut self, value: u64) {
        match self {
            Self::Pair(_, r) => r.add_rightmost(value),
            Self::Literal(x) => *x += value,
        }
    }

    fn add_leftmost(&mut self, value: u64) {
        match self {
            Self::Pair(l, _) => l.add_leftmost(value),
            Self::Literal(x) => *x += value,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Pair(l, r) => {
                if l.split() {
                    return true;
                }
                r.split()
            }
            Self::Literal(x) => {
                if *x >= 10 {
                    let lower = *x / 2;
                    let higher = lower + if *x % 2 == 0 { 0 } else { 1 };
                    let _ = mem::replace(
                        self,
                        Self::pair(Self::Literal(lower), Self::Literal(higher)),
                    );
                    return true;
                }
                false
            }
        }
    }
}

impl fmt::Display for SfNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pair(l, r) => write!(f, "[{},{}]", l, r),
            Self::Literal(x) => write!(f, "{}", x),
        }
    }
}

impl Add for SfNum {
    type Output = SfNum;

    fn add(self, rhs: Self) -> Self {
        let mut added = Self::pair(self, rhs);
        added.reduce();
        added
    }
}

impl Sum for SfNum {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.reduce(|acc, n| acc + n).unwrap_or(Self::Literal(0))
    }
}

fn sf_num(input: &str) -> IResult<&str, SfNum> {
    use nom::branch::alt;

    alt((sf_pair, sf_lit))(input)
}

fn sf_pair(input: &str) -> IResult<&str, SfNum> {
    use nom::{
        character::complete::char as char_,
        sequence::{delimited, separated_pair},
    };

    map(
        delimited(
            char_('['),
            separated_pair(sf_num, char_(','), sf_num),
            char_(']'),
        ),
        |(l, r)| SfNum::pair(l, r),
    )(input)
}

fn sf_lit(input: &str) -> IResult<&str, SfNum> {
    use nom::character::complete::u64 as u64_;

    map(u64_, SfNum::Literal)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explodes_left() {
        let mut actual = sf_num("[[[[[9,8],1],2],3],4]").unwrap().1;
        actual.explode(0);
        let expected = sf_num("[[[[0,9],2],3],4]").unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn explodes_right() {
        let mut actual = sf_num("[7,[6,[5,[4,[3,2]]]]]").unwrap().1;
        actual.explode(0);
        let expected = sf_num("[7,[6,[5,[7,0]]]]").unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn explodes_in_middle() {
        let mut actual = sf_num("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap().1;
        actual.explode(0);
        let expected = sf_num("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sf_nums_add() {
        let a = sf_num("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap().1;
        let b = sf_num("[1,1]").unwrap().1;
        let actual = a + b;
        let expected = sf_num("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sf_nums_sum() {
        let actual = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]
        .iter()
        .map(|n| sf_num(n).unwrap().1)
        .sum();
        let expected = sf_num("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            .unwrap()
            .1;
        assert_eq!(expected, actual);
    }
}
