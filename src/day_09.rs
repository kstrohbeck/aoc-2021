use nom::{combinator::map, IResult};
use std::ops;

pub fn star_1(data: String) {
    let heights = parse(&data);

    let mut risk_level_sum: u64 = 0;
    for coord in heights.all_coords() {
        let center = heights[coord];
        let is_basin = heights
            .neighbor_coords(coord)
            .map(|c| heights[c])
            .all(|h| h > center);

        if is_basin {
            risk_level_sum += (center as u64) + 1;
        }
    }
    println!("{}", risk_level_sum);
}

pub fn star_2(data: String) {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Cell {
        Peak,
        Unset,
        Basin(usize),
    }

    impl Cell {
        fn basin(self) -> Option<usize> {
            match self {
                Self::Peak | Self::Unset => None,
                Self::Basin(b) => Some(b),
            }
        }
    }

    let mut heights = parse(&data).map(|h| if h >= 9 { Cell::Peak } else { Cell::Unset });
    let mut basin_counts = Vec::new();
    let mut joins = HashMap::new();
    let mut rev_joins = HashMap::new();

    for coord in heights.all_coords() {
        if heights[coord] == Cell::Peak {
            continue;
        }
        let up = heights
            .up(coord)
            .and_then(|c| heights.get(c))
            .and_then(|c| c.basin());
        let left = heights
            .left(coord)
            .and_then(|c| heights.get(c))
            .and_then(|c| c.basin());
        let basin = match (up, left) {
            (Some(x), Some(y)) if x == y => x,
            (Some(x), Some(y)) => {
                let (lesser, greater) = if x < y { (x, y) } else { (y, x) };
                joins
                    .entry(lesser)
                    .or_insert_with(HashSet::new)
                    .insert(greater);
                rev_joins
                    .entry(greater)
                    .or_insert_with(HashSet::new)
                    .insert(lesser);
                lesser
            }
            (Some(x), None) | (None, Some(x)) => x,
            (None, None) => basin_counts.len(),
        };
        while basin_counts.len() <= basin {
            basin_counts.push(0);
        }
        basin_counts[basin] += 1;
        heights[coord] = Cell::Basin(basin);
    }

    fn sum_of_joins(
        basin_counts: &[usize],
        joins: &HashMap<usize, HashSet<usize>>,
        basin: usize,
        already_counted: &mut HashSet<usize>,
    ) -> usize {
        if already_counted.contains(&basin) {
            return 0;
        }
        let mut count = basin_counts[basin];
        already_counted.insert(basin);
        if let Some(join_set) = joins.get(&basin) {
            for b in join_set {
                count += sum_of_joins(basin_counts, joins, *b, already_counted);
            }
        }
        count
    }

    let mut basin_sizes = (0..basin_counts.len())
        .filter(|b| rev_joins.get(b).is_none())
        .map(|b| {
            let mut already_counted = HashSet::new();
            sum_of_joins(&basin_counts, &joins, b, &mut already_counted)
        })
        .collect::<Vec<_>>();

    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    let product = basin_sizes.iter().take(3).product::<usize>();
    println!("{}", product);
}

fn parse(input: &str) -> Vec2d<u8> {
    vec2d_u8(input).unwrap().1
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Vec2d<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Vec2d<T> {
    pub fn repeat(value: T, width: usize, height: usize) -> Self
    where
        T: Copy,
    {
        Self {
            data: vec![value; width * height],
            width,
        }
    }

    pub fn get(&self, coord: (usize, usize)) -> Option<&T> {
        self.data.get(coord_to_idx(coord, self.width))
    }

    pub fn map<F, U>(self, f: F) -> Vec2d<U>
    where
        F: FnMut(T) -> U,
    {
        Vec2d {
            data: self.data.into_iter().map(f).collect(),
            width: self.width,
        }
    }

    pub fn all_coords(&self) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width;
        (0..self.data.len()).map(move |idx| idx_to_coord(idx, width))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn left(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if col == 0 {
            return None;
        }
        Some((col - 1, row))
    }

    pub fn right(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if col >= self.width - 1 {
            return None;
        }
        Some((col + 1, row))
    }

    pub fn up(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if row == 0 {
            return None;
        }
        Some((col, row - 1))
    }

    pub fn down(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if row >= self.height() - 1 {
            return None;
        }
        Some((col, row + 1))
    }

    pub fn neighbor_coords(
        &self,
        coord: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        [Self::left, Self::right, Self::up, Self::down]
            .iter()
            .map(move |f| f(self, coord))
            .flatten()
    }
}

impl<T> ops::Index<(usize, usize)> for Vec2d<T> {
    type Output = T;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        self.data.index(coord_to_idx(coord, self.width))
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Self::Output {
        self.data.index_mut(coord_to_idx(coord, self.width))
    }
}

fn coord_to_idx((col, row): (usize, usize), width: usize) -> usize {
    row * width + col
}

fn idx_to_coord(idx: usize, width: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Vec2dBuilder<T> {
    data: Vec<T>,
    is_first_row: bool,
    width: usize,
}

impl<T> Vec2dBuilder<T> {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        if self.is_first_row {
            self.width += 1;
        }
    }

    fn finish_row(&mut self) {
        self.is_first_row = false;
        // TODO: If len is not a multiple of width, set some kind of error flag.
    }

    fn build(self) -> Vec2d<T> {
        Vec2d {
            data: self.data,
            width: self.width,
        }
    }
}

impl<T> Default for Vec2dBuilder<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            is_first_row: true,
            width: 0,
        }
    }
}

fn vec2d_u8(input: &str) -> IResult<&str, Vec2d<u8>> {
    use nom::multi::fold_many0;

    map(
        fold_many0(digit_or_newline, Vec2dBuilder::new, |mut b, d| {
            match d {
                DigitOrNewline::Digit(x) => b.push(x),
                DigitOrNewline::Newline => b.finish_row(),
            }
            b
        }),
        Vec2dBuilder::build,
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DigitOrNewline {
    Digit(u8),
    Newline,
}

fn digit_or_newline(input: &str) -> IResult<&str, DigitOrNewline> {
    use nom::{branch::alt, character::complete::line_ending, combinator::value};

    alt((
        map(digit, DigitOrNewline::Digit),
        value(DigitOrNewline::Newline, line_ending),
    ))(input)
}

fn digit(input: &str) -> IResult<&str, u8> {
    use nom::{character::complete::anychar, combinator::map_opt};

    map_opt(anychar, |c| c.to_digit(10).map(|x| x as u8))(input)
}
