use super::utils::Vec2d;
use nom::{character::complete::line_ending, multi::separated_list0, IResult};
use std::ops;

pub fn star_1(data: String) {
    use std::collections::HashSet;

    let sensors = parse(&data);
    let off_rots = off_rots(&sensors[..]);

    let mut all_points = HashSet::new();
    for idx in 0..sensors.len() {
        let (off, rot) = off_rots[idx].unwrap();
        for point in &sensors[idx] {
            all_points.insert(off + point.rotated(rot));
        }
    }

    println!("{}", all_points.len());
}

pub fn star_2(data: String) {
    let sensors = parse(&data);
    let off_rots = off_rots(&sensors[..]);

    let mut biggest_dist = 0;
    for idx_a in 0..sensors.len() {
        for idx_b in 0..sensors.len() {
            let a_off = off_rots[idx_a].unwrap().0;
            let b_off = off_rots[idx_b].unwrap().0;
            let dist = a_off.manhattan_distance(b_off);
            biggest_dist = biggest_dist.max(dist);
        }
    }

    println!("{}", biggest_dist);
}

fn check_overlap(a: &[Point], b: &[Point]) -> Option<(Rotation, Point)> {
    use std::collections::HashMap;

    for rot in ALL_ROTATIONS {
        let mut offset_counts = HashMap::new();
        for p_a in a {
            for p_b in b {
                *offset_counts.entry(*p_a - p_b.rotated(rot)).or_insert(0) += 1;
            }
        }
        for (offset, count) in offset_counts {
            if count >= 12 {
                return Some((rot, offset));
            }
        }
    }
    None
}

fn off_rots(sensors: &[Vec<Point>]) -> Vec<Option<(Point, Rotation)>> {
    let mut off_rots = vec![None; sensors.len()];
    off_rots[0] = Some((Point { x: 0, y: 0, z: 0 }, NULL_ROTATION));

    let mut in_progress = true;
    while in_progress {
        in_progress = false;
        for idx_a in 0..sensors.len() {
            let (a_off, a_rot) = match off_rots[idx_a] {
                Some(x) => x,
                None => continue,
            };

            let a = &sensors[idx_a];

            for idx_b in 0..sensors.len() {
                if idx_a == idx_b || off_rots[idx_b].is_some() {
                    continue;
                }

                let b = &sensors[idx_b];

                if let Some((rot, off)) = check_overlap(a, b) {
                    off_rots[idx_b] = Some((a_off + off.rotated(a_rot), rot + a_rot));
                    in_progress = true;
                }
            }
        }
        if !in_progress || off_rots.iter().all(|x| x.is_some()) {
            break;
        }
    }

    off_rots
}

fn parse(input: &str) -> Vec<Vec<Point>> {
    super::utils::parse(sensors, input)
}

fn sensors(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    use nom::character::complete::multispace0;

    separated_list0(multispace0, sensor)(input)
}

fn sensor(input: &str) -> IResult<&str, Vec<Point>> {
    use nom::sequence::preceded;

    preceded(sensor_header, separated_list0(line_ending, point))(input)
}

fn sensor_header(input: &str) -> IResult<&str, u64> {
    use nom::{bytes::complete::tag, character::complete::u64 as u64_};
    let (input, _) = tag("--- scanner ")(input)?;
    let (input, n) = u64_(input)?;
    let (input, _) = tag(" ---")(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, n))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn rotated(self, rot: Rotation) -> Self {
        Self {
            x: rot.x.sign.apply(self.axis(rot.x.axis)),
            y: rot.y.sign.apply(self.axis(rot.y.axis)),
            z: rot.z.sign.apply(self.axis(rot.z.axis)),
        }
    }

    fn axis(self, axis: Axis) -> i64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    fn manhattan_distance(self, rhs: Self) -> u64 {
        (self.x - rhs.x).abs() as u64
            + (self.y - rhs.y).abs() as u64
            + (self.z - rhs.z).abs() as u64
    }
}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    use nom::character::complete::{char as char_, i64 as i64_};

    let (input, x) = i64_(input)?;
    let (input, _) = char_(',')(input)?;
    let (input, y) = i64_(input)?;
    let (input, _) = char_(',')(input)?;
    let (input, z) = i64_(input)?;
    Ok((input, Point { x, y, z }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rotation {
    x: SignedAxis,
    y: SignedAxis,
    z: SignedAxis,
}

impl Rotation {
    fn axis(self, axis: Axis) -> SignedAxis {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

impl ops::Add for Rotation {
    type Output = Rotation;

    fn add(self, rhs: Rotation) -> Self {
        let mut x = self.axis(rhs.x.axis);
        x.sign = x.sign + rhs.x.sign;
        let mut y = self.axis(rhs.y.axis);
        y.sign = y.sign + rhs.y.sign;
        let mut z = self.axis(rhs.z.axis);
        z.sign = z.sign + rhs.z.sign;
        Self { x, y, z }
    }
}

macro_rules! r {
    (x) => {
        SignedAxis {
            sign: Sign::Positive,
            axis: Axis::X,
        }
    };
    (-x) => {
        SignedAxis {
            sign: Sign::Negative,
            axis: Axis::X,
        }
    };
    (y) => {
        SignedAxis {
            sign: Sign::Positive,
            axis: Axis::Y,
        }
    };
    (-y) => {
        SignedAxis {
            sign: Sign::Negative,
            axis: Axis::Y,
        }
    };
    (z) => {
        SignedAxis {
            sign: Sign::Positive,
            axis: Axis::Z,
        }
    };
    (-z) => {
        SignedAxis {
            sign: Sign::Negative,
            axis: Axis::Z,
        }
    };
}

const NULL_ROTATION: Rotation = Rotation {
    x: r!(x),
    y: r!(y),
    z: r!(z),
};

const ALL_ROTATIONS: [Rotation; 24] = [
    NULL_ROTATION,
    Rotation {
        x: r!(x),
        y: r!(-z),
        z: r!(y),
    },
    Rotation {
        x: r!(x),
        y: r!(-y),
        z: r!(-z),
    },
    Rotation {
        x: r!(x),
        y: r!(z),
        z: r!(-y),
    },
    Rotation {
        x: r!(-x),
        y: r!(z),
        z: r!(y),
    },
    Rotation {
        x: r!(-x),
        y: r!(y),
        z: r!(-z),
    },
    Rotation {
        x: r!(-x),
        y: r!(-z),
        z: r!(-y),
    },
    Rotation {
        x: r!(-x),
        y: r!(-y),
        z: r!(z),
    },
    Rotation {
        x: r!(y),
        y: r!(z),
        z: r!(x),
    },
    Rotation {
        x: r!(y),
        y: r!(-x),
        z: r!(z),
    },
    Rotation {
        x: r!(y),
        y: r!(-z),
        z: r!(-x),
    },
    Rotation {
        x: r!(y),
        y: r!(x),
        z: r!(-z),
    },
    Rotation {
        x: r!(-y),
        y: r!(x),
        z: r!(z),
    },
    Rotation {
        x: r!(-y),
        y: r!(z),
        z: r!(-x),
    },
    Rotation {
        x: r!(-y),
        y: r!(-x),
        z: r!(-z),
    },
    Rotation {
        x: r!(-y),
        y: r!(-z),
        z: r!(x),
    },
    Rotation {
        x: r!(z),
        y: r!(x),
        z: r!(y),
    },
    Rotation {
        x: r!(z),
        y: r!(-y),
        z: r!(x),
    },
    Rotation {
        x: r!(z),
        y: r!(-x),
        z: r!(-y),
    },
    Rotation {
        x: r!(z),
        y: r!(y),
        z: r!(-x),
    },
    Rotation {
        x: r!(-z),
        y: r!(y),
        z: r!(x),
    },
    Rotation {
        x: r!(-z),
        y: r!(x),
        z: r!(-y),
    },
    Rotation {
        x: r!(-z),
        y: r!(-y),
        z: r!(-x),
    },
    Rotation {
        x: r!(-z),
        y: r!(-x),
        z: r!(y),
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SignedAxis {
    sign: Sign,
    axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Sign {
    Negative,
    Positive,
}

impl Sign {
    fn apply(self, num: i64) -> i64 {
        match self {
            Self::Negative => -1 * num,
            Self::Positive => num,
        }
    }
}

impl ops::Add for Sign {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self == rhs {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Axis {
    X,
    Y,
    Z,
}
