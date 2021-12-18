use nom::{
    combinator::map,
    error::{Error, ErrorKind, ParseError},
    Err as NomErr, IResult, InputIter, InputLength, Slice,
};
use std::ops::{AddAssign, RangeFrom, RangeTo, Shl, Shr};

pub fn star_1(data: String) {
    let packet = parse(&data);
    let sum = packet.version_sum();
    println!("{}", sum);
}

pub fn star_2(data: String) {
    let packet = parse(&data);
    let eval = packet.eval();
    println!("{}", eval);
}

fn parse(data: &str) -> Packet {
    use super::utils::parse;

    let hex = parse(hex_string, data);
    parse(packet, BitSlice::from(&hex[..]))
}

fn hex_string(mut input: &str) -> IResult<&str, Vec<u8>> {
    use nom::multi::many1;

    many1(hex_u8)(input)
}

fn hex_u8(input: &str) -> IResult<&str, u8> {
    use nom::{bytes::complete::take_while_m_n, combinator::map_res};
    map_res(take_while_m_n(2, 2, |c: char| c.is_digit(16)), |i| {
        u8::from_str_radix(i, 16)
    })(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    data: PacketData,
}

impl Packet {
    fn literal(version: u8, num: usize) -> Self {
        Self {
            version,
            data: PacketData::Literal(num),
        }
    }

    fn operator(version: u8, op_type: OperatorType, subpackets: Vec<Packet>) -> Self {
        Self {
            version,
            data: PacketData::Operator {
                op_type,
                subpackets,
            },
        }
    }

    fn version_sum(&self) -> usize {
        let child_sum = match &self.data {
            PacketData::Literal(_) => 0,
            PacketData::Operator {
                op_type,
                subpackets,
            } => subpackets.iter().map(Packet::version_sum).sum(),
        };
        child_sum + (self.version as usize)
    }

    fn eval(&self) -> usize {
        self.data.eval()
    }
}

fn packet(input: BitSlice<&[u8]>) -> IResult<BitSlice<&[u8]>, Packet> {
    use nom::sequence::pair;

    map(pair(num(3), packet_data), |(version, data)| Packet {
        version,
        data,
    })(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    Literal(usize),
    Operator {
        op_type: OperatorType,
        subpackets: Vec<Packet>,
    },
}

impl PacketData {
    fn eval(&self) -> usize {
        match self {
            Self::Literal(n) => *n,
            Self::Operator {
                op_type,
                subpackets,
            } => op_type.eval(subpackets),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl OperatorType {
    fn eval(self, subpackets: &[Packet]) -> usize {
        match self {
            Self::Sum => subpackets.iter().map(Packet::eval).sum(),
            Self::Product => subpackets.iter().map(Packet::eval).product(),
            Self::Minimum => subpackets.iter().map(Packet::eval).min().unwrap(),
            Self::Maximum => subpackets.iter().map(Packet::eval).max().unwrap(),
            Self::GreaterThan => {
                if subpackets[0].eval() > subpackets[1].eval() {
                    1
                } else {
                    0
                }
            }
            Self::LessThan => {
                if subpackets[0].eval() < subpackets[1].eval() {
                    1
                } else {
                    0
                }
            }
            Self::EqualTo => {
                if subpackets[0].eval() == subpackets[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl From<u8> for OperatorType {
    fn from(id: u8) -> Self {
        match id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => panic!(),
        }
    }
}

fn packet_data(input: BitSlice<&[u8]>) -> IResult<BitSlice<&[u8]>, PacketData> {
    println!("packet data input: {:?}", input);
    let (input, type_id): (_, u8) = num(3)(input)?;
    if type_id == 4 {
        map(literal, PacketData::Literal)(input)
    } else {
        let op_type = OperatorType::from(type_id);
        let (input, subpackets) = subpackets(input)?;
        Ok((
            input,
            PacketData::Operator {
                op_type,
                subpackets,
            },
        ))
    }
}

fn literal(mut input: BitSlice<&[u8]>) -> IResult<BitSlice<&[u8]>, usize> {
    let mut acc = 0;
    loop {
        let (inp, bit): (_, u8) = num(1)(input)?;
        let (inp, data): (_, usize) = num(4)(inp)?;
        input = inp;
        acc = (acc << 4) + data;
        if bit == 0 {
            break;
        }
    }
    Ok((input, acc))
}

fn subpackets(input: BitSlice<&[u8]>) -> IResult<BitSlice<&[u8]>, Vec<Packet>> {
    let (input, length_type_bit): (_, u8) = num(1)(input)?;
    if length_type_bit == 0 {
        length_type_0_subpackets(input)
    } else {
        length_type_1_subpackets(input)
    }
}

fn length_type_0_subpackets(input: BitSlice<&[u8]>) -> IResult<BitSlice<&[u8]>, Vec<Packet>> {
    use nom::multi::many1;

    let (input, len) = num(15)(input)?;
    let (input, subslice) = slice(len)(input)?;
    let (_, packets) = many1(packet)(subslice)?;
    Ok((input, packets))
}

fn length_type_1_subpackets(input: BitSlice<&[u8]>) -> IResult<BitSlice<&[u8]>, Vec<Packet>> {
    use nom::multi::length_count;

    length_count::<_, _, usize, _, _, _>(num(11), packet)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitSlice<I> {
    input: I,
    start_offset: usize,
    end_offset: usize,
}

impl<I> From<I> for BitSlice<I> {
    fn from(input: I) -> Self {
        Self {
            input,
            start_offset: 0,
            end_offset: 0,
        }
    }
}

impl<I> InputLength for BitSlice<I>
where
    I: InputLength,
{
    fn input_len(&self) -> usize {
        self.input.input_len() * 8 - (self.start_offset + self.end_offset)
    }
}

fn slice<I, E: ParseError<BitSlice<I>>>(
    count: usize,
) -> impl Fn(BitSlice<I>) -> IResult<BitSlice<I>, BitSlice<I>, E>
where
    I: Slice<RangeTo<usize>> + Slice<RangeFrom<usize>> + InputLength,
{
    move |slice: BitSlice<I>| {
        if slice.input_len() < count {
            return Err(NomErr::Error(E::from_error_kind(slice, ErrorKind::Eof)));
        }

        let split_offset = (count + slice.start_offset) % 8;
        let num_bytes = (count + slice.start_offset) / 8;
        let head_bytes = num_bytes + if split_offset != 0 { 1 } else { 0 };

        let head = BitSlice {
            input: slice.input.slice(..head_bytes),
            start_offset: slice.start_offset,
            end_offset: if split_offset == 0 {
                0
            } else {
                8 - split_offset
            },
        };

        let rest = BitSlice {
            input: slice.input.slice(num_bytes..),
            start_offset: split_offset,
            end_offset: slice.end_offset,
        };

        Ok((rest, head))
    }
}

fn num<I, O, E: ParseError<BitSlice<I>>>(
    count: usize,
) -> impl Fn(BitSlice<I>) -> IResult<BitSlice<I>, O, E>
where
    I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
    O: From<u8> + AddAssign + Shl<usize, Output = O> + Shr<usize, Output = O>,
{
    move |slice: BitSlice<I>| {
        if count == 0 {
            return Ok((slice, 0u8.into()));
        }

        if slice.input_len() < count {
            return Err(NomErr::Error(E::from_error_kind(slice, ErrorKind::Eof)));
        }

        let mut acc = 0u8.into();
        let mut offset = slice.start_offset;
        let mut remaining = count;
        let mut new_offset = 0;

        let num_bytes = (count + slice.start_offset) / 8;

        for byte in slice.input.iter_elements().take(num_bytes + 1) {
            if remaining == 0 {
                break;
            }

            let val: O = if offset == 0 {
                byte.into()
            } else {
                ((byte << offset) as u8 >> offset).into()
            };

            if remaining < 8 - offset {
                acc += val >> (8 - offset - remaining);
                new_offset = remaining + offset;
                break;
            }

            acc += val << (remaining - (8 - offset));
            remaining -= 8 - offset;
            offset = 0;
        }

        Ok((
            BitSlice {
                input: slice.input.slice(num_bytes..),
                start_offset: new_offset,
                end_offset: slice.end_offset,
            },
            acc,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_string_parses() {
        let input = "D2FE28";
        assert_eq!(Ok(("", vec![0xd2, 0xfe, 0x28])), hex_string(input));
    }

    #[test]
    fn literal_parses() {
        let input = BitSlice::from(&[0xd2, 0xfe, 0x28][..]);
        let expected = Packet::literal(6, 2021);
        let actual = packet(input).unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn length_type_0_parses() {
        let input = BitSlice::from(&[0x38, 0x00, 0x6F, 0x45, 0x29, 0x12, 0x00][..]);
        let expected = Packet::operator(
            1,
            OperatorType::LessThan,
            vec![Packet::literal(6, 10), Packet::literal(2, 20)],
        );
        let actual = packet(input).unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn length_type_1_parses() {
        let input = BitSlice::from(&[0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60][..]);
        let expected = Packet::operator(
            7,
            OperatorType::Maximum,
            vec![
                Packet::literal(2, 1),
                Packet::literal(4, 2),
                Packet::literal(1, 3),
            ],
        );
        let actual = packet(input).unwrap().1;
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_sums() {
    //     let inputs = [
    //         ("C200B40A82", 3),
    //         ("04005AC33890", 54),
    //         ("880086C3E88112", 7),
    //         ("CE00C43D881120", 9),
    //         ("D8005AC2A8F0", 1),
    //         ("F600BC2D8F", 0),
    //         ("9C005AC2F8F0", 0),
    //         ("9C0141080250320F1802104A08", 1),
    //     ];

    //     for (input, expected) in inputs {
    //         let packet = parse(input);
    //         assert_eq!(expected, packet.eval());
    //     }
    // }

    #[test]
    fn less_than_byte_num_parses() {
        let input = BitSlice {
            input: &[0b11000000][..],
            start_offset: 0,
            end_offset: 0,
        };
        let (rest, n) = num::<&[u8], u8, ()>(3)(input).unwrap();
        assert_eq!(0b110, n);
        assert_eq!(
            BitSlice {
                input: &[0b11000000][..],
                start_offset: 3,
                end_offset: 0,
            },
            rest
        );
    }

    #[test]
    fn slice_offset() {
        let input = BitSlice {
            input: &[0, 1, 2, 3][..],
            start_offset: 6,
            end_offset: 3,
        };
        let (rest, head) = slice::<&[u8], ()>(9)(input).unwrap();
        assert_eq!(
            BitSlice {
                input: &[0, 1][..],
                start_offset: 6,
                end_offset: 1,
            },
            head
        );
        assert_eq!(
            BitSlice {
                input: &[1, 2, 3][..],
                start_offset: 7,
                end_offset: 3,
            },
            rest
        );
    }

    #[test]
    fn zero_slice_offset() {
        let input = BitSlice {
            input: &[0, 1, 2, 3][..],
            start_offset: 6,
            end_offset: 3,
        };
        let (rest, head) = slice::<&[u8], ()>(10)(input).unwrap();
        assert_eq!(
            BitSlice {
                input: &[0, 1][..],
                start_offset: 6,
                end_offset: 0,
            },
            head
        );
        assert_eq!(
            BitSlice {
                input: &[2, 3][..],
                start_offset: 0,
                end_offset: 3,
            },
            rest
        );
    }
}
