use super::utils::{Vec2d, Vec2dBuilder};
use nom::{combinator::map, IResult};
use std::cmp::Ordering;

pub fn star_1(data: String) {
    let grid = parse(&data);
    let shortest = shortest_path(&grid, (0, 0), (grid.width() - 1, grid.height() - 1)).unwrap();
    println!("{}", shortest);
}

pub fn star_2(data: String) {
    let tile = parse(&data);
    let mut grid = Vec2d::repeat(0, tile.width() * 5, tile.height() * 5);
    for (x, y) in grid.all_coords() {
        let tile_x = x % tile.width();
        let x_offset = x / tile.width();
        let tile_y = y % tile.height();
        let y_offset = y / tile.height();
        let offset = (x_offset + y_offset) as u8;
        let risk = (tile[(tile_x, tile_y)] + offset - 1) % 9 + 1;
        grid[(x, y)] = risk;
    }
    let shortest = shortest_path(&grid, (0, 0), (grid.width() - 1, grid.height() - 1)).unwrap();
    println!("{}", shortest);
}

// Taken from https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn shortest_path(grid: &Vec2d<u8>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    use std::collections::BinaryHeap;

    let mut dist = Vec2d::repeat(usize::MAX, grid.width(), grid.height());
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for nbr in grid.neighbor_coords(position) {
            let next = State {
                cost: cost + grid[nbr] as usize,
                position: nbr,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl State {
    fn new(position: (usize, usize)) -> Self {
        Self {
            cost: usize::MAX,
            position,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: Dedupe this & day_09.

fn parse(input: &str) -> Vec2d<u8> {
    super::utils::parse(vec2d_u8, input)
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
