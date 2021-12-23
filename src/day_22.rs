use itertools::iproduct;
use nom::{bytes::complete::tag, IResult};
use std::{fmt, ops};

pub fn star_1(data: String) {
    let steps = parse(&data);
    let focus = Cube {
        x: Segment::from_ends(-50, 50),
        y: Segment::from_ends(-50, 50),
        z: Segment::from_ends(-50, 50),
    };
    let steps = steps.iter().filter_map(|step| {
        step.cube.overlapping(focus).map(|cube| RebootStep {
            cube,
            on_off: step.on_off,
        })
    });
    let num_cubes = run_steps(steps);
    println!("{}", num_cubes);
}

pub fn star_2(data: String) {
    let steps = parse(&data);
    let num_cubes = run_steps(steps.into_iter());
    println!("{}", num_cubes);
}

fn run_steps<I>(steps: I) -> u64
where
    I: Iterator<Item = RebootStep>,
{
    let mut shape = Vec::new();
    let mut accum = Vec::new();
    let mut buf = Vec::new();
    for step in steps {
        if step.on_off == OnOff::On {
            accum.clear();
            accum.push(step.cube);
            subtract_multiple(&mut accum, &shape[..], &mut buf);
            shape.append(&mut accum);
        } else {
            subtract_single(&mut shape, step.cube, &mut buf);
        }
    }
    shape.iter().map(|c| c.volume()).sum::<u64>()
}

fn subtract_single(shape: &mut Vec<Cube>, cube: Cube, buf: &mut Vec<Cube>) {
    buf.clear();
    let mut i = 0;
    while i < shape.len() {
        let cur = &mut shape[i];
        if !cur.intersects(cube) {
            i += 1;
            continue;
        }
        let mut non_ovr = cur.non_overlapping(cube);
        if let Some(n) = non_ovr.next() {
            *cur = n;
            buf.extend(non_ovr);
            i += 1;
        } else {
            shape.remove(i);
        }
    }

    shape.append(buf);
}

fn subtract_multiple(shape: &mut Vec<Cube>, cubes: &[Cube], buf: &mut Vec<Cube>) {
    for cube in cubes {
        subtract_single(shape, *cube, buf);
    }
}

fn parse(input: &str) -> Vec<RebootStep> {
    super::utils::parse(reboot_steps, input)
}

fn reboot_steps(input: &str) -> IResult<&str, Vec<RebootStep>> {
    use nom::{character::complete::line_ending, multi::separated_list0};

    separated_list0(line_ending, reboot_step)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RebootStep {
    on_off: OnOff,
    cube: Cube,
}

fn reboot_step(input: &str) -> IResult<&str, RebootStep> {
    use nom::sequence::preceded;

    let (input, on_off) = on_off(input)?;
    let (input, x) = preceded(tag(" x="), segment)(input)?;
    let (input, y) = preceded(tag(",y="), segment)(input)?;
    let (input, z) = preceded(tag(",z="), segment)(input)?;
    let cube = Cube { x, y, z };
    Ok((input, RebootStep { on_off, cube }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OnOff {
    On,
    Off,
}

fn on_off(input: &str) -> IResult<&str, OnOff> {
    use nom::{branch::alt, combinator::value};

    alt((value(OnOff::On, tag("on")), value(OnOff::Off, tag("off"))))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: Segment,
    y: Segment,
    z: Segment,
}

impl Cube {
    fn volume(self) -> u64 {
        (self.x.len + 1) * (self.y.len + 1) * (self.z.len + 1)
    }

    fn intersects(self, rhs: Self) -> bool {
        self.x.intersects(rhs.x) && self.y.intersects(rhs.y) && self.z.intersects(rhs.z)
    }

    fn overlapping(self, rhs: Self) -> Option<Cube> {
        let x = self.x.overlapping(rhs.x)?;
        let y = self.y.overlapping(rhs.y)?;
        let z = self.z.overlapping(rhs.z)?;
        Some(Cube { x, y, z })
    }

    fn non_overlapping(self, rhs: Self) -> impl Iterator<Item = Self> {
        let xs = self.x.parts(rhs.x);
        let ys = self.y.parts(rhs.y).enumerate();
        let zs = self.z.parts(rhs.z).enumerate();
        iproduct!(xs, ys, zs).filter_map(move |(x, (iy, y), (iz, z))| {
            if !x.is_overlapping {
                if iy == 0 && iz == 0 {
                    Some(Cube {
                        x: x.value,
                        y: self.y,
                        z: self.z,
                    })
                } else {
                    None
                }
            } else if !y.is_overlapping {
                if iz == 0 {
                    Some(Cube {
                        x: x.value,
                        y: y.value,
                        z: self.z,
                    })
                } else {
                    None
                }
            } else if !z.is_overlapping {
                Some(Cube {
                    x: x.value,
                    y: y.value,
                    z: z.value,
                })
            } else {
                None
            }
        })
    }

    fn try_adding(self, rhs: Self) -> Option<Cube> {
        if let Some(x) = self.x.try_adding(rhs.x) {
            if self.y == rhs.y && self.z == rhs.z {
                return Some(Cube {
                    x,
                    y: self.y,
                    z: self.z,
                });
            }
        }

        if let Some(y) = self.y.try_adding(rhs.y) {
            if self.x == rhs.x && self.z == rhs.z {
                return Some(Cube {
                    x: self.x,
                    y,
                    z: self.z,
                });
            }
        }

        if let Some(z) = self.z.try_adding(rhs.z) {
            if self.x == rhs.x && self.y == rhs.y {
                return Some(Cube {
                    x: self.x,
                    y: self.y,
                    z,
                });
            }
        }

        None
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[x={},y={},z={}]", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Segment {
    start: i64,
    len: u64,
}

impl Segment {
    fn from_ends(start: i64, end: i64) -> Self {
        Self {
            start: start.min(end),
            len: (start - end).abs() as u64,
        }
    }

    fn end(self) -> i64 {
        self.start + (self.len as i64)
    }

    fn intersects(self, rhs: Self) -> bool {
        rhs.end() >= self.start && rhs.start <= self.end()
    }

    fn parts(self, rhs: Self) -> SegmentPartsIter {
        SegmentPartsIter::new(self, rhs)
    }

    fn overlapping(self, rhs: Self) -> Option<Self> {
        // TODO: Break this down into calculating endpoints.
        if rhs.start <= self.start && rhs.end() >= self.start && rhs.end() < self.end() {
            Some(Self::from_ends(self.start, rhs.end()))
        } else if rhs.start > self.start && rhs.end() < self.end() {
            Some(Self::from_ends(rhs.start, rhs.end()))
        } else if rhs.start > self.start && rhs.start <= self.end() && rhs.end() >= self.end() {
            Some(Self::from_ends(rhs.start, self.end()))
        } else if rhs.start <= self.start && rhs.end() >= self.end() {
            Some(self)
        } else {
            None
        }
    }

    fn non_overlapping(self, rhs: Self) -> SegmentNonOverlappingIter {
        SegmentNonOverlappingIter::new(self, rhs)
    }

    fn try_adding(self, rhs: Self) -> Option<Self> {
        let len = self.len + rhs.len + 1;
        if self.end() == rhs.start - 1 {
            Some(Self {
                start: self.start,
                len,
            })
        } else if rhs.end() == self.start - 1 {
            Some(Self {
                start: rhs.start,
                len,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SegmentPartsIter {
    ovr: Option<Segment>,
    non_ovr: SegmentNonOverlappingIter,
    idx: usize,
}

impl SegmentPartsIter {
    fn new(a: Segment, b: Segment) -> Self {
        let ovr = a.overlapping(b);
        let non_ovr = SegmentNonOverlappingIter::new(a, b);
        Self {
            ovr,
            non_ovr,
            idx: 0,
        }
    }
}

impl Iterator for SegmentPartsIter {
    type Item = Overlap<Segment>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.idx {
                0 => {
                    self.idx += 1;
                    if let Some(ovr) = self.ovr {
                        return Some(Overlap::overlap(ovr));
                    }
                }
                1 => {
                    if let Some(next) = self.non_ovr.next() {
                        return Some(Overlap::non_overlap(next));
                    }
                    self.idx += 1;
                }
                _ => return None,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SegmentNonOverlappingIter {
    left: Option<Segment>,
    right: Option<Segment>,
    full: Option<Segment>,
    idx: usize,
}

impl SegmentNonOverlappingIter {
    fn new(a: Segment, b: Segment) -> Self {
        let left = (b.start > a.start && b.start <= a.end())
            .then(move || Segment::from_ends(a.start, b.start - 1));
        let right = (b.end() >= a.start && b.end() < a.end())
            .then(move || Segment::from_ends(b.end() + 1, a.end()));
        let full = (b.end() < a.start || b.start > a.end()).then(|| a);

        Self {
            left,
            right,
            full,
            idx: 0,
        }
    }
}

impl Iterator for SegmentNonOverlappingIter {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.idx {
                0 => {
                    self.idx += 1;
                    if let Some(left) = self.left {
                        return Some(left);
                    }
                }
                1 => {
                    self.idx += 1;
                    if let Some(right) = self.right {
                        return Some(right);
                    }
                }
                2 => {
                    self.idx += 1;
                    if let Some(full) = self.full {
                        return Some(full);
                    }
                }
                _ => return None,
            }
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end())
    }
}

fn segment(input: &str) -> IResult<&str, Segment> {
    use nom::{character::complete::i64 as i64_, combinator::map, sequence::separated_pair};

    map(separated_pair(i64_, tag(".."), i64_), |(start, end)| {
        Segment::from_ends(start, end)
    })(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Overlap<T> {
    value: T,
    is_overlapping: bool,
}

impl<T> Overlap<T> {
    fn non_overlap(value: T) -> Self {
        Self {
            value,
            is_overlapping: false,
        }
    }

    fn overlap(value: T) -> Self {
        Self {
            value,
            is_overlapping: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_value_segment_works() {
        let seg = Segment::from_ends(0, 0);
        assert_eq!(Segment { start: 0, len: 0 }, seg);
    }

    #[test]
    fn far_left_doesnt_intersect() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, -3);
        assert!(!seg.intersects(ovr));
    }

    #[test]
    fn far_left_non_overlaps_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, -3);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        let expected = vec![seg];
        assert_eq!(expected, actual);
    }

    #[test]
    fn far_left_has_no_overlap() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, -3);
        let actual = seg.overlapping(ovr);
        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn exact_left_intersects() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, -2);
        assert!(seg.intersects(ovr));
    }

    #[test]
    fn exact_left_non_overlaps_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, -2);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        let expected = vec![Segment::from_ends(-1, 2)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn exact_left_has_correct_overlap() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, -2);
        let actual = seg.overlapping(ovr);
        let expected = Some(Segment::from_ends(-2, -2));
        assert_eq!(expected, actual);
    }

    #[test]
    fn left_intersects() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, 1);
        assert!(seg.intersects(ovr));
    }

    #[test]
    fn left_non_overlaps_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, 1);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        let expected = vec![Segment::from_ends(2, 2)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn left_has_correct_overlap() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, 1);
        let actual = seg.overlapping(ovr);
        let expected = Some(Segment::from_ends(-2, 1));
        assert_eq!(expected, actual);
    }

    #[test]
    fn exact_intersects() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-2, 2);
        assert!(seg.intersects(ovr));
    }

    #[test]
    fn exact_has_no_non_overlaps() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-2, 2);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        assert!(actual.is_empty());
    }

    #[test]
    fn exact_has_correct_overlap() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-2, 2);
        let actual = seg.overlapping(ovr);
        let expected = Some(Segment::from_ends(-2, 2));
        assert_eq!(expected, actual);
    }

    #[test]
    fn center_intersects() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-1, 1);
        assert!(seg.intersects(ovr));
    }

    #[test]
    fn center_non_overlaps_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-1, 1);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        let expected = vec![Segment::from_ends(-2, -2), Segment::from_ends(2, 2)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn center_overlap_is_covering_segment() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-1, 1);
        let actual = seg.overlapping(ovr);
        let expected = Some(ovr);
        assert_eq!(expected, actual);
    }

    #[test]
    fn right_intersects() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-1, 5);
        assert!(seg.intersects(ovr));
    }

    #[test]
    fn right_non_overlaps_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-1, 5);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        let expected = vec![Segment::from_ends(-2, -2)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn right_has_correct_overlap() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-1, 5);
        let actual = seg.overlapping(ovr);
        let expected = Some(Segment::from_ends(-1, 2));
        assert_eq!(expected, actual);
    }

    #[test]
    fn exact_right_intersects() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(2, 5);
        assert!(seg.intersects(ovr));
    }

    #[test]
    fn exact_right_non_overlaps_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(2, 5);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        let expected = vec![Segment::from_ends(-2, 1)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn exact_right_has_correct_overlap() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(2, 5);
        let actual = seg.overlapping(ovr);
        let expected = Some(Segment::from_ends(2, 2));
        assert_eq!(expected, actual);
    }

    #[test]
    fn far_right_doesnt_intersect() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(3, 5);
        assert!(!seg.intersects(ovr));
    }

    #[test]
    fn far_right_non_overlaps_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(3, 5);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        let expected = vec![seg];
        assert_eq!(expected, actual);
    }

    #[test]
    fn far_right_has_no_overlap() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(3, 5);
        let actual = seg.overlapping(ovr);
        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn entire_intersects() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, 5);
        assert!(seg.intersects(ovr));
    }

    #[test]
    fn entire_has_no_non_overlaps() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, 5);
        let actual = seg.non_overlapping(ovr).collect::<Vec<_>>();
        assert!(actual.is_empty());
    }

    #[test]
    fn entire_overlap_is_full_segment() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-5, 5);
        let actual = seg.overlapping(ovr);
        let expected = Some(seg);
        assert_eq!(expected, actual);
    }

    #[test]
    fn center_parts_are_correct() {
        let seg = Segment::from_ends(-2, 2);
        let ovr = Segment::from_ends(-1, 1);
        let actual = seg.parts(ovr).collect::<Vec<_>>();
        let expected = vec![
            Overlap::overlap(ovr),
            Overlap::non_overlap(Segment::from_ends(-2, -2)),
            Overlap::non_overlap(Segment::from_ends(2, 2)),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn cube_overlap_is_correct() {
        let cube = Cube {
            x: Segment::from_ends(10, 12),
            y: Segment::from_ends(10, 12),
            z: Segment::from_ends(10, 12),
        };
        let ovr = Cube {
            x: Segment::from_ends(11, 13),
            y: Segment::from_ends(11, 13),
            z: Segment::from_ends(11, 13),
        };
        let actual = cube.overlapping(ovr);
        let expected = Some(Cube {
            x: Segment::from_ends(11, 12),
            y: Segment::from_ends(11, 12),
            z: Segment::from_ends(11, 12),
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn disjoint_cubes_have_no_overlap() {
        let cube = Cube {
            x: Segment::from_ends(10, 12),
            y: Segment::from_ends(10, 12),
            z: Segment::from_ends(10, 12),
        };
        let ovr = Cube {
            x: Segment::from_ends(13, 14),
            y: Segment::from_ends(13, 14),
            z: Segment::from_ends(13, 14),
        };
        let actual = cube.overlapping(ovr);
        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn cube_non_overlap_volume_is_correct() {
        let cube = Cube {
            x: Segment::from_ends(10, 12),
            y: Segment::from_ends(10, 12),
            z: Segment::from_ends(10, 12),
        };
        let ovr = Cube {
            x: Segment::from_ends(11, 13),
            y: Segment::from_ends(11, 13),
            z: Segment::from_ends(11, 13),
        };
        let actual = cube.non_overlapping(ovr).map(|c| c.volume()).sum::<u64>();
        let expected = 19;
        assert_eq!(expected, actual);
    }

    #[test]
    fn cube_volume_is_correct() {
        let a = Cube {
            x: Segment::from_ends(10, 12),
            y: Segment::from_ends(10, 12),
            z: Segment::from_ends(10, 12),
        };
        let actual = a.volume();
        let expected = 27;
        assert_eq!(expected, actual);
    }

    #[test]
    fn cubes_intersect() {
        let cube = Cube {
            x: Segment::from_ends(10, 12),
            y: Segment::from_ends(10, 12),
            z: Segment::from_ends(10, 12),
        };
        let ovr = Cube {
            x: Segment::from_ends(11, 13),
            y: Segment::from_ends(11, 13),
            z: Segment::from_ends(11, 13),
        };
        assert!(cube.intersects(ovr));
    }

    #[test]
    fn adding_cube_volume_is_correct() {
        let a = Cube {
            x: Segment::from_ends(10, 12),
            y: Segment::from_ends(10, 12),
            z: Segment::from_ends(10, 12),
        };
        let b = Cube {
            x: Segment::from_ends(13, 14),
            y: Segment::from_ends(10, 12),
            z: Segment::from_ends(10, 12),
        };
        let actual = a.try_adding(b).unwrap().volume();
        let expected = a.volume() + b.volume();
        assert_eq!(expected, actual);
    }
}
