use nom::{bytes::complete::tag, combinator::map, sequence::separated_pair, IResult};
use std::{collections::HashMap, iter::IntoIterator};

pub fn star_1(data: String) {
    let lines = parse(&data);
    let iter = lines.into_iter().filter(|line| !line.is_diagonal());
    let intersections = intersections(iter);
    println!("{}", intersections);
}

pub fn star_2(data: String) {
    let lines = parse(&data);
    let iter = lines.into_iter();
    let intersections = intersections(iter);
    println!("{}", intersections);
}

fn intersections<I>(lines: I) -> usize
where
    I: Iterator<Item = Line>,
{
    let all_coords = lines.flat_map(Line::all_coords);

    let mut coord_map = HashMap::new();
    for coord in all_coords {
        *coord_map.entry(coord).or_insert(0) += 1;
    }

    coord_map.values().filter(|n| **n > 1).count()
}

fn parse(input: &str) -> Vec<Line> {
    lines(input).unwrap().1
}

fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    use nom::{character::complete::multispace1, multi::separated_list0};

    separated_list0(multispace1, line)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    fn is_diagonal(self) -> bool {
        (self.start.x != self.end.x) && (self.start.y != self.end.y)
    }

    fn all_coords(self) -> impl Iterator<Item = Coord> {
        use std::{iter, ops::RangeInclusive};

        #[derive(Debug, Clone)]
        enum Iter {
            Less(RangeInclusive<u64>),
            Equal(iter::Repeat<u64>),
            Greater(iter::Rev<RangeInclusive<u64>>),
        }

        impl Iter {
            fn new(a: u64, b: u64) -> Self {
                use std::cmp::Ordering;

                match a.cmp(&b) {
                    Ordering::Less => Self::Less(a..=b),
                    Ordering::Equal => Self::Equal(std::iter::repeat(a)),
                    Ordering::Greater => Self::Greater((b..=a).rev()),
                }
            }
        }

        impl Iterator for Iter {
            type Item = u64;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Less(i) => i.next(),
                    Self::Equal(i) => i.next(),
                    Self::Greater(i) => i.next(),
                }
            }
        }

        let range_x = Iter::new(self.start.x, self.end.x);
        let range_y = Iter::new(self.start.y, self.end.y);
        range_x.zip(range_y).map(|(x, y)| Coord { x, y })
    }
}

fn line(input: &str) -> IResult<&str, Line> {
    use nom::character::complete::space0;

    let arrow = separated_pair(space0, tag("->"), space0);

    map(separated_pair(coord, arrow, coord), |(start, end)| Line {
        start,
        end,
    })(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u64,
    y: u64,
}

fn coord(input: &str) -> IResult<&str, Coord> {
    use nom::character::complete::u64 as u64_;

    map(separated_pair(u64_, tag(","), u64_), |(x, y)| Coord {
        x,
        y,
    })(input)
}
