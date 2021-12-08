use nom::IResult;
use std::ops;

pub fn star_1(data: String) {
    let entries = parse(&data);
    let count: usize = entries
        .iter()
        .map(|entry| entry.output.iter().filter(|d| d.is_simple()).count())
        .sum();
    println!("{}", count);
}

pub fn star_2(data: String) {
    let entries = parse(&data);
    let sum = entries.iter().map(|e| e.get_output().unwrap()).sum::<u64>();
    println!("{}", sum);
}

fn parse(input: &str) -> Vec<Entry> {
    entries(input).unwrap().1
}

fn entries(input: &str) -> IResult<&str, Vec<Entry>> {
    use nom::{character::complete::multispace0, multi::separated_list0};

    separated_list0(multispace0, entry)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    signals: [Digit; 10],
    output: [Digit; 4],
}

impl Entry {
    pub fn get_output(self) -> Result<u64, &'static str> {
        let mut one = None;
        let mut four = None;
        let mut five_seg_a = None;
        let mut five_seg_b = None;
        let mut five_seg_c = None;
        let mut six_seg_a = None;
        let mut six_seg_b = None;
        let mut six_seg_c = None;
        let mut seven = None;
        let mut eight = None;

        macro_rules! set_or_err {
            ($a:ident, $v:expr, $err:expr) => {
                if $a == None {
                    $a = Some($v);
                } else {
                    return Err($err);
                }
            };
        }

        macro_rules! set_cascade_or_err {
            ($a:ident, $b:ident, $c:ident, $v:expr, $err:expr) => {
                if $a == None {
                    $a = Some($v);
                } else if $b == None {
                    $b = Some($v);
                } else if $c == None {
                    $c = Some($v);
                } else {
                    return Err($err);
                }
            };
        }

        for digit in self.signals {
            match digit.num_segments() {
                2 => set_or_err!(one, digit, "more than one 1"),
                3 => set_or_err!(seven, digit, "more than one 7"),
                4 => set_or_err!(four, digit, "more than one 4"),
                5 => set_cascade_or_err!(
                    five_seg_a,
                    five_seg_b,
                    five_seg_c,
                    digit,
                    "more than 3 5-segments"
                ),
                6 => set_cascade_or_err!(
                    six_seg_a,
                    six_seg_b,
                    six_seg_c,
                    digit,
                    "more than 3 6-segments"
                ),
                7 => set_or_err!(eight, digit, "more than one 8"),
                _ => return Err("invalid digit"),
            }
        }

        let one = one.ok_or("no 1 given")?;
        let four = four.ok_or("no 4 given")?;
        let five_seg_a = five_seg_a.ok_or("no 5-segment given")?;
        let five_seg_b = five_seg_b.ok_or("only one 5-segment given")?;
        let five_seg_c = five_seg_c.ok_or("only two 5-segments given")?;
        let six_seg_a = six_seg_a.ok_or("no 6-segment given")?;
        let six_seg_b = six_seg_b.ok_or("only one 6-segment given")?;
        let six_seg_c = six_seg_c.ok_or("only two 6-segments given")?;
        let seven = seven.ok_or("no 7 given")?;
        let eight = eight.ok_or("no 8 given")?;

        let fives = five_seg_a & five_seg_b & five_seg_c;
        let sixes = six_seg_a & six_seg_b & six_seg_c;

        let a = !one & seven;
        let c = one & !sixes;
        let d = four & fives;
        let e = !four & !sixes;
        let f = one & sixes;

        let g = !a & fives & sixes;
        let b = four & !one & !d;

        macro_rules! seg {
            ($x:ident) => {
                $x.try_to_segment().ok_or_else(|| "Digit not a segment")?
            };
        }

        let seg_map = [
            (seg!(a), Segment::A),
            (seg!(b), Segment::B),
            (seg!(c), Segment::C),
            (seg!(d), Segment::D),
            (seg!(e), Segment::E),
            (seg!(f), Segment::F),
            (seg!(g), Segment::G),
        ];

        let mut sum: u64 = 0;
        for digit in self.output {
            sum *= 10;
            sum += digit
                .transform(&seg_map)
                .try_to_u8()
                .ok_or_else(|| "Digit not a real digit")? as u64;
        }
        Ok(sum)
    }
}

fn entry(input: &str) -> IResult<&str, Entry> {
    use super::utils::{sep_array_10, sep_array_4};
    use nom::{
        character::complete::{char as char_, space0, space1},
        combinator::map,
        sequence::separated_pair,
    };

    let signals = sep_array_10(space1, digit);
    let sep = separated_pair(space0, char_('|'), space0);
    let output = sep_array_4(space1, digit);
    map(separated_pair(signals, sep, output), |(signals, output)| {
        Entry { signals, output }
    })(input)
}

// Layout: 0gfedcba
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digit(u8);

impl Digit {
    const EMPTY: Self = Self(0b00000000);
    const FULL: Self = Self(0b01111111);
    const ZERO: Self = Self(0b01110111);
    const ONE: Self = Self(0b00100100);
    const TWO: Self = Self(0b01011101);
    const THREE: Self = Self(0b01101101);
    const FOUR: Self = Self(0b00101110);
    const FIVE: Self = Self(0b01101011);
    const SIX: Self = Self(0b01111011);
    const SEVEN: Self = Self(0b00100101);
    const EIGHT: Self = Self(0b01111111);
    const NINE: Self = Self(0b01101111);

    pub const fn num_segments(self) -> u8 {
        self.0.count_ones() as u8
    }

    pub const fn is_simple(self) -> bool {
        matches!(self.num_segments(), 2 | 3 | 4 | 7)
    }

    pub const fn is_set(self, segment: Segment) -> bool {
        (self.0 & segment.mask()) > 0
    }

    pub const fn with_set(self, segment: Segment) -> Self {
        Self(self.0 | segment.mask())
    }

    pub const fn try_to_segment(self) -> Option<Segment> {
        if self.num_segments() != 1 {
            return None;
        }

        Segment::try_from_u8(self.0.trailing_zeros() as u8)
    }

    pub fn transform(self, seg_map: &[(Segment, Segment)]) -> Self {
        let mut mapped = Self::EMPTY;
        for (old, new) in seg_map {
            if self.is_set(*old) {
                mapped = mapped.with_set(*new);
            }
        }
        mapped
    }

    pub const fn try_to_u8(self) -> Option<u8> {
        match self {
            Self::ZERO => Some(0),
            Self::ONE => Some(1),
            Self::TWO => Some(2),
            Self::THREE => Some(3),
            Self::FOUR => Some(4),
            Self::FIVE => Some(5),
            Self::SIX => Some(6),
            Self::SEVEN => Some(7),
            Self::EIGHT => Some(8),
            Self::NINE => Some(9),
            _ => None,
        }
    }
}

impl Default for Digit {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl ops::BitAnd for Digit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ops::BitOr for Digit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::Not for Digit {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & 0b01111111)
    }
}

fn digit(input: &str) -> IResult<&str, Digit> {
    use nom::multi::fold_many1;

    fold_many1(segment, Digit::default, |digit, segment| {
        digit.with_set(segment)
    })(input)
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Segment {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Segment {
    pub const fn mask(self) -> u8 {
        1 << (self as u8)
    }

    pub const fn try_from_u8(x: u8) -> Option<Self> {
        match x {
            0 => Some(Self::A),
            1 => Some(Self::B),
            2 => Some(Self::C),
            3 => Some(Self::D),
            4 => Some(Self::E),
            5 => Some(Self::F),
            6 => Some(Self::G),
            _ => None,
        }
    }

    pub const fn try_from_char(c: char) -> Option<Self> {
        match c {
            'a' | 'A' => Some(Self::A),
            'b' | 'B' => Some(Self::B),
            'c' | 'C' => Some(Self::C),
            'd' | 'D' => Some(Self::D),
            'e' | 'E' => Some(Self::E),
            'f' | 'F' => Some(Self::F),
            'g' | 'G' => Some(Self::G),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Segment {
    type Error = ();

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        Self::try_from_u8(x).ok_or(())
    }
}

impl TryFrom<char> for Segment {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::try_from_char(c).ok_or(())
    }
}

fn segment(input: &str) -> IResult<&str, Segment> {
    use nom::{character::complete::one_of, combinator::map_res};

    map_res(one_of("aAbBcCdDeEfFgG"), Segment::try_from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unset_segment_becomes_set() {
        let digit = Digit(0b00010011);
        let digit = digit.with_set(Segment::D);
        assert!(digit.is_set(Segment::D));
    }

    #[test]
    fn digit_to_segment_is_correct() {
        let digit = Digit(0b00010000);
        assert_eq!(Some(Segment::E), digit.try_to_segment());
    }
}
