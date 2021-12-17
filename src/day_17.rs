use nom::{bytes::complete::tag, IResult};

pub fn star_1(data: String) {
    let bounds = parse(&data);
    let mut highest = 0;
    let on_hit = |_, y| {
        let height = (y * (y + 1)) / 2;
        highest = highest.max(height);
    };
    run_sims(bounds, on_hit);
    println!("{}", highest);
}

pub fn star_2(data: String) {
    let bounds = parse(&data);
    let mut num_hits = 0;
    let on_hit = |_, _| num_hits += 1;
    run_sims(bounds, on_hit);
    println!("{}", num_hits);
}

// Really should be an iterator.
fn run_sims<F: FnMut(i64, i64)>(bounds: Bounds, mut on_hit: F) {
    let leftmost = (2f64 * bounds.left() as f64).sqrt().floor() as i64;
    let rightmost = bounds.right();

    for start_x_vel in leftmost..=rightmost {
        let bottom_most = bounds.bottom();
        // This is completely incorrect.
        let top_most = bounds.right() * 2;

        for start_y_vel in bottom_most..=top_most {
            let mut x_vel: i64 = start_x_vel;
            let mut y_vel: i64 = start_y_vel;
            let mut x_pos: i64 = 0;
            let mut y_pos: i64 = 0;
            loop {
                x_pos += x_vel;
                y_pos += y_vel;
                if x_vel > 0 {
                    x_vel -= 1;
                } else if x_vel < 0 {
                    x_vel += 1;
                }
                y_vel -= 1;
                if bounds.contains(x_pos, y_pos) {
                    on_hit(start_x_vel, start_y_vel);
                    break;
                }
                if y_pos < bounds.bottom() {
                    break;
                }
            }
        }
    }
}

fn parse(data: &str) -> Bounds {
    super::utils::parse(bounds, data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bounds {
    x: i64,
    y: i64,
    width: u64,
    height: u64,
}

impl Bounds {
    fn new(xs: (i64, i64), ys: (i64, i64)) -> Self {
        let x = xs.0.min(xs.1);
        let y = ys.0.min(ys.1);
        let width = (xs.0 - xs.1).abs() as u64;
        let height = (ys.0 - ys.1).abs() as u64;
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn left(self) -> i64 {
        self.x
    }

    fn right(self) -> i64 {
        self.x + (self.width as i64)
    }

    fn bottom(self) -> i64 {
        self.y
    }

    fn top(self) -> i64 {
        self.y + (self.height as i64)
    }

    fn contains(self, x: i64, y: i64) -> bool {
        let in_left = x >= self.left();
        let in_right = x <= self.right();
        let in_bottom = y >= self.bottom();
        let in_top = y <= self.top();
        in_left && in_right && in_bottom && in_top
    }
}

fn bounds(input: &str) -> IResult<&str, Bounds> {
    let (input, _) = tag("target area: ")(input)?;
    let (input, xs) = coord_pair('x')(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, ys) = coord_pair('y')(input)?;
    Ok((input, Bounds::new(xs, ys)))
}

fn coord_pair(c: char) -> impl FnMut(&str) -> IResult<&str, (i64, i64)> {
    use nom::{
        character::complete::{char as char_, i64 as i64_},
        sequence::{pair, preceded, separated_pair},
    };

    move |input| {
        preceded(
            pair(char_(c), char_('=')),
            separated_pair(i64_, tag(".."), i64_),
        )(input)
    }
}
