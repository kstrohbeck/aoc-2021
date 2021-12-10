use nom::IResult;
use std::ops::Not;

pub fn star_1(data: String) {
    let nums = parse(&data);
    let mut accum = vec![0_i64; nums[0].len() as usize];
    for num in nums {
        for (i, bit) in num.bits().enumerate() {
            accum[i] += match bit {
                0 => -1,
                _ => 1,
            };
        }
    }
    let gamma = accum
        .iter()
        .map(|b| if *b > 0 { 1 } else { 0 })
        .fold(Number::default(), Number::push_bit);
    let epsilon = !gamma;
    let gamma: u64 = gamma.into();
    let epsilon: u64 = epsilon.into();

    println!("{}", gamma * epsilon);
}

pub fn star_2(data: String) {
    fn part(nums: &[Number], flip: bool) -> u64 {
        let mut nums = nums.to_owned();
        for i in 0..nums[0].len() {
            let (zero, one): (Vec<Number>, _) = nums.iter().partition(|num| num.bit(i) == 0);
            let pick_zero = zero.len() > one.len();
            nums = if pick_zero == flip { one } else { zero };
            if nums.len() <= 1 {
                break;
            }
        }
        nums[0].into()
    }

    let nums = parse(&data);

    let oxy = part(&nums, false);
    let co2 = part(&nums, true);

    println!("{}", oxy * co2);
}

fn parse(data: &str) -> Vec<Number> {
    super::utils::parse(numbers, data)
}

fn numbers(input: &str) -> IResult<&str, Vec<Number>> {
    super::utils::lines(number)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Number {
    bits: u64,
    len: u8,
}

impl Number {
    pub fn len(self) -> u8 {
        self.len
    }

    pub fn bit(self, bit: u8) -> u64 {
        let offset = self.len - bit - 1;
        (self.bits & (1 << offset)) >> offset
    }

    pub fn bits(self) -> impl Iterator<Item = u64> {
        (0..self.len).map(move |bit| self.bit(bit))
    }

    fn mask(self) -> u64 {
        (1 << self.len) - 1
    }

    pub fn push_bit(self, bit: u64) -> Self {
        Self {
            bits: (self.bits << 1) + bit,
            len: self.len + 1,
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self { bits: 0, len: 0 }
    }
}

impl From<Number> for u64 {
    fn from(number: Number) -> Self {
        number.bits
    }
}

impl Not for Number {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            bits: (!self.bits) & self.mask(),
            len: self.len,
        }
    }
}

fn number(input: &str) -> IResult<&str, Number> {
    use nom::multi::fold_many1;

    fold_many1(bit, Number::default, Number::push_bit)(input)
}

fn bit(input: &str) -> IResult<&str, u64> {
    use nom::{branch::alt, bytes::complete::tag, combinator::value};

    alt((value(0, tag("0")), value(1, tag("1"))))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_are_correct() {
        let num = Number {
            bits: 0b1011100,
            len: 7,
        };
        assert_eq!(vec![1, 0, 1, 1, 1, 0, 0], num.bits().collect::<Vec<_>>());
    }

    #[test]
    fn mask_is_correct() {
        let num = Number {
            bits: 0b1011100,
            len: 7,
        };
        assert_eq!(0b1111111, num.mask());
    }

    #[test]
    fn not_flips_bits() {
        let num = Number {
            bits: 0b1011100,
            len: 7,
        };
        assert_eq!(
            Number {
                bits: 0b0100011,
                len: 7
            },
            !num
        );
    }

    #[test]
    fn number_is_parsed() {
        let input = "1011100\n";
        let num = number(input);
        assert_eq!(
            Ok((
                "\n",
                Number {
                    bits: 0b1011100,
                    len: 7
                }
            )),
            num
        );
    }
}
