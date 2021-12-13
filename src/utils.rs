use nom::{error::ParseError, Compare, IResult, InputIter, InputLength, Parser, Slice};
use std::ops::{self, Range, RangeFrom, RangeTo};

pub fn lines<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + InputLength + InputIter + Compare<&'static str>,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    use nom::{character::complete::line_ending, multi::separated_list0};

    separated_list0(line_ending, parser)
}

pub fn parse<I, O, E, F>(mut parser: F, input: I) -> O
where
    F: Parser<I, O, E>,
    nom::Err<E>: std::fmt::Debug,
{
    parser.parse(input).unwrap().1
}

macro_rules! sep_arrays {
    ($( $name:ident, $num:literal ),*) => {
        $(
            pub fn $name<I, O, O2, E, F, G>(
                mut sep: G,
                mut parser: F,
            ) -> impl FnMut(I) -> ::nom::IResult<I, [O; $num], E>
            where
                F: nom::Parser<I, O, E>,
                G: nom::Parser<I, O2, E>,
                E: nom::error::ParseError<I>,
            {
                use std::mem::MaybeUninit;

                move |mut input| {
                    // Safe because we aren't actually able to access any uninitialized memory.
                    let mut array: [MaybeUninit<O>; $num] = unsafe { MaybeUninit::uninit().assume_init() };

                    for (i, cell) in array.iter_mut().enumerate() {
                        if i != 0 {
                            let (inp, _) = sep.parse(input)?;
                            input = inp;
                        }
                        let (inp, u) = parser.parse(input)?;
                        input = inp;
                        cell.write(u);
                    }

                    // Safe because:
                    // - [MaybeUninit<T>; n] and [T; n] are guaranteed to be the same size
                    // - each MaybeUninit<T> in the array has been initialized
                    let array = unsafe { array.as_ptr().cast::<[O; $num]>().read() };

                    Ok((input, array))
                }
            }
        )*
    };
}

sep_arrays! {
    sep_array_4, 4,
    sep_array_5, 5,
    sep_array_10, 10
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec2d<T> {
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
            .filter_map(move |f| f(self, coord))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

// impl<'a, T> IntoIterator for &'a Vec2d<T> {
//     type Item = &'a T;
//     //type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
//     type IntoIter = std::slice::Iter<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.data.iter()
//     }
// }

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
pub struct Vec2dBuilder<T> {
    data: Vec<T>,
    is_first_row: bool,
    width: usize,
}

impl<T> Vec2dBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        if self.is_first_row {
            self.width += 1;
        }
    }

    pub fn finish_row(&mut self) {
        self.is_first_row = false;
        // TODO: If len is not a multiple of width, set some kind of error flag.
    }

    pub fn build(self) -> Vec2d<T> {
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
