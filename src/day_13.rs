use super::utils::Vec2d;
use nom::{
    character::complete::{char as char_, u64 as u64_},
    combinator::map,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashSet;

pub fn star_1(data: String) {
    let values = parse(&data);
    let mut folded = HashSet::new();

    let fold = values.folds[0];
    for coord in values.coords {
        folded.insert(coord.folded(fold));
    }

    println!("{}", folded.len());
}

pub fn star_2(data: String) {
    let values = parse(&data);
    let mut folded = HashSet::new();

    for &(mut coord) in &values.coords {
        for &fold in &values.folds {
            coord = coord.folded(fold);
        }
        folded.insert(coord);
    }

    print_coords(&folded);
}

fn print_coords(coords: &HashSet<Coord>) {
    let mut max_x = 0;
    let mut max_y = 0;
    for coord in coords {
        max_x = max_x.max(coord.x);
        max_y = max_y.max(coord.y);
    }
    let mut image = Vec2d::repeat(' ', (max_x + 1) as usize, (max_y + 1) as usize);

    for coord in coords {
        image[(coord.x as usize, coord.y as usize)] = '#';
    }

    let line_ends = image.all_coords().map(|(x, _)| x == max_x as usize);

    for (c, is_end) in image.iter().zip(line_ends) {
        print!("{}", c);
        if is_end {
            println!();
        }
    }
}

fn parse(input: &str) -> Values {
    super::utils::parse(values, input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Values {
    coords: Vec<Coord>,
    folds: Vec<Fold>,
}

fn values(input: &str) -> IResult<&str, Values> {
    use nom::{character::complete::multispace1, multi::separated_list1};

    map(
        separated_pair(
            separated_list1(multispace1, coord),
            multispace1,
            separated_list1(multispace1, fold),
        ),
        |(coords, folds)| Values { coords, folds },
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u64,
    y: u64,
}

impl Coord {
    fn folded(self, fold: Fold) -> Self {
        fn folded(val: u64, fold: u64) -> u64 {
            if val <= fold {
                val
            } else {
                2 * fold - val
            }
        }

        match fold.axis {
            Axis::X => Self {
                x: folded(self.x, fold.pos),
                y: self.y,
            },
            Axis::Y => Self {
                x: self.x,
                y: folded(self.y, fold.pos),
            },
        }
    }
}

impl From<(u64, u64)> for Coord {
    fn from((x, y): (u64, u64)) -> Self {
        Self { x, y }
    }
}

fn coord(input: &str) -> IResult<&str, Coord> {
    map(separated_pair(u64_, char_(','), u64_), Coord::from)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fold {
    axis: Axis,
    pos: u64,
}

fn fold(input: &str) -> IResult<&str, Fold> {
    use nom::{bytes::complete::tag, sequence::preceded};

    map(
        preceded(tag("fold along "), separated_pair(axis, char_('='), u64_)),
        |(axis, pos)| Fold { axis, pos },
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

impl TryFrom<char> for Axis {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' | 'X' => Ok(Self::X),
            'y' | 'Y' => Ok(Self::Y),
            _ => Err(()),
        }
    }
}

fn axis(input: &str) -> IResult<&str, Axis> {
    use nom::{character::complete::anychar, combinator::map_res};

    map_res(anychar, Axis::try_from)(input)
}
