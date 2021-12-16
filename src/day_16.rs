use nom::{bits::complete::take, IResult};

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
    parse(packet, (&hex[..], 0)).0
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

fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), (Packet, usize)> {
    let (input, version) = take(3u8)(input)?;
    let (input, (data, len)) = packet_data(input)?;
    Ok((input, (Packet { version, data }, len + 3)))
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

fn packet_data(input: (&[u8], usize)) -> IResult<(&[u8], usize), (PacketData, usize)> {
    let (input, type_id): (_, u8) = take(3u8)(input)?;
    if type_id == 4 {
        let (input, (val, len)) = literal(input)?;
        Ok((input, (PacketData::Literal(val), len + 3)))
    } else {
        let op_type = OperatorType::from(type_id);
        let (input, (subpackets, len)) = subpackets(input)?;
        Ok((
            input,
            (
                PacketData::Operator {
                    op_type,
                    subpackets,
                },
                len + 3,
            ),
        ))
    }
}

fn literal(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), (usize, usize)> {
    let mut num = 0;
    let mut len = 0;
    loop {
        let (inp, bit): (_, u8) = take(1u8)(input)?;
        let (inp, data): (_, usize) = take(4u8)(inp)?;
        input = inp;
        num = (num << 4) + data;
        len += 5;
        if bit == 0 {
            break;
        }
    }
    Ok((input, (num, len)))
}

fn subpackets(input: (&[u8], usize)) -> IResult<(&[u8], usize), (Vec<Packet>, usize)> {
    let (input, length_type_bit): (_, u8) = take(1u8)(input)?;
    if length_type_bit == 0 {
        let (input, (subpackets, len)) = length_type_0_subpackets(input)?;
        Ok((input, (subpackets, len + 1)))
    } else {
        let (input, (subpackets, len)) = length_type_1_subpackets(input)?;
        Ok((input, (subpackets, len + 1)))
    }
}

fn length_type_0_subpackets(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), (Vec<Packet>, usize)> {
    let (mut input, len): (_, usize) = take(15u8)(input)?;
    let mut packets = Vec::new();
    let mut used = 0;
    loop {
        let (inp, (packet, p_len)) = packet(input)?;
        packets.push(packet);
        used += p_len;
        input = inp;
        if used >= len {
            break;
        }
    }
    Ok((input, (packets, used + 15)))
}

fn length_type_1_subpackets(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), (Vec<Packet>, usize)> {
    let (mut input, num): (_, usize) = take(11u8)(input)?;
    let mut packets = Vec::new();
    let mut used = 0;
    for _ in 0..num {
        let (inp, (packet, p_len)) = packet(input)?;
        packets.push(packet);
        used += p_len;
        input = inp;
    }
    Ok((input, (packets, used + 11)))
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
        let input = (&[0xd2, 0xfe, 0x28][..], 0);
        let expected = Packet::literal(6, 2021);
        assert_eq!(Ok(((&[0x28][..], 5), (expected, 21))), packet(input));
    }

    #[test]
    fn length_type_0_parses() {
        let input = (&[0x38, 0x00, 0x6F, 0x45, 0x29, 0x12, 0x00][..], 0);
        let expected = Packet::operator(
            1,
            OperatorType::LessThan,
            vec![Packet::literal(6, 10), Packet::literal(2, 20)],
        );
        assert_eq!(Ok(((&[0x00][..], 1), (expected, 49))), packet(input));
    }

    #[test]
    fn length_type_1_parses() {
        let input = (&[0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60][..], 0);
        let expected = Packet::operator(
            7,
            OperatorType::Maximum,
            vec![
                Packet::literal(2, 1),
                Packet::literal(4, 2),
                Packet::literal(1, 3),
            ],
        );
        assert_eq!(Ok(((&[0x60][..], 3), (expected, 51))), packet(input));
    }

    #[test]
    fn test_sums() {
        let inputs = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (input, expected) in inputs {
            let packet = parse(input);
            assert_eq!(expected, packet.eval());
        }
    }
}
