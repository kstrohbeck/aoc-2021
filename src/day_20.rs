use nom::IResult;
use std::collections::HashSet;

pub fn star_1(data: String) {
    let Info {
        pixel_map,
        mut image,
    } = parse(&data);
    let image = image.step_n(&pixel_map[..], 2);
    println!("{}", image.num_lit());
}

pub fn star_2(data: String) {
    let Info {
        pixel_map,
        mut image,
    } = parse(&data);
    let image = image.step_n(&pixel_map[..], 50);
    println!("{}", image.num_lit());
}

fn parse(input: &str) -> Info {
    super::utils::parse(info, input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Info {
    pixel_map: Vec<Pixel>,
    image: Image,
}

fn info(input: &str) -> IResult<&str, Info> {
    use nom::{character::complete::multispace1, combinator::map, sequence::separated_pair};

    map(
        separated_pair(pixel_map, multispace1, image),
        |(pixel_map, image)| Info { pixel_map, image },
    )(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Image {
    pixels: HashSet<(i64, i64)>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    ambient: Pixel,
}

impl Image {
    fn step_n(&self, pixel_map: &[Pixel], num: usize) -> Self {
        let mut image = self.clone();
        for _ in 0..num {
            image = image.step(pixel_map);
        }
        image
    }

    fn step(&self, pixel_map: &[Pixel]) -> Self {
        let mut builder = ImageBuilder::new();
        builder.ambient = if self.ambient == Pixel::Light {
            pixel_map[511]
        } else {
            pixel_map[0]
        };
        for y in (self.y_min - 1)..=(self.y_max + 1) {
            for x in (self.x_min - 1)..=(self.x_max + 1) {
                if pixel_map[self.coord_to_lookup((x, y))] == Pixel::Light {
                    builder.set_pixel((x, y));
                }
            }
        }
        builder.build()
    }

    fn coord_to_lookup(&self, coord: (i64, i64)) -> usize {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, (x, y))| match self.pixel((coord.0 + x, coord.1 + y)) {
            Pixel::Dark => 0,
            Pixel::Light => 1 << (8 - i),
        })
        .sum()
    }

    fn pixel(&self, coord: (i64, i64)) -> Pixel {
        if self.pixels.contains(&coord) {
            Pixel::Light
        } else if self.is_in_bounds(coord) {
            Pixel::Dark
        } else {
            self.ambient
        }
    }

    fn is_in_bounds(&self, (x, y): (i64, i64)) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }

    fn num_lit(&self) -> usize {
        self.pixels.iter().count()
    }

    fn print(&self) {
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                print!("{}", self.pixel((x, y)).as_char());
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImageBuilder {
    pixels: HashSet<(i64, i64)>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    ambient: Pixel,
}

impl ImageBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn set_pixel(&mut self, (x, y): (i64, i64)) {
        self.pixels.insert((x, y));
        self.x_min = self.x_min.min(x);
        self.x_max = self.x_max.max(x);
        self.y_min = self.y_min.min(y);
        self.y_max = self.y_max.max(y);
    }

    fn build(self) -> Image {
        Image {
            pixels: self.pixels,
            x_min: self.x_min,
            x_max: self.x_max,
            y_min: self.y_min,
            y_max: self.y_max,
            ambient: self.ambient,
        }
    }
}

impl Default for ImageBuilder {
    fn default() -> Self {
        Self {
            pixels: HashSet::new(),
            x_min: i64::MAX,
            x_max: i64::MIN,
            y_min: i64::MAX,
            y_max: i64::MIN,
            ambient: Pixel::Dark,
        }
    }
}

fn image(mut input: &str) -> IResult<&str, Image> {
    use nom::{character::complete::line_ending, error::Error};

    let mut builder = ImageBuilder::new();

    'outer: for y in 0.. {
        for x in 0.. {
            if let Ok((inp, _)) = line_ending::<_, Error<_>>(input) {
                input = inp;
                break;
            }

            let (inp, p) = match pixel(input) {
                Ok(x) => x,
                Err(_) => break 'outer,
            };
            input = inp;
            if p == Pixel::Light {
                builder.set_pixel((x, y));
            }
        }
    }

    Ok((input, builder.build()))
}

fn pixel_map(input: &str) -> IResult<&str, Vec<Pixel>> {
    use nom::multi::many1;

    many1(pixel)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pixel {
    Dark,
    Light,
}

impl Pixel {
    fn as_char(self) -> char {
        match self {
            Self::Dark => '.',
            Self::Light => '#',
        }
    }
}

impl TryFrom<char> for Pixel {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Dark),
            '#' => Ok(Self::Light),
            _ => Err(()),
        }
    }
}

fn pixel(input: &str) -> IResult<&str, Pixel> {
    use nom::{character::complete::anychar, combinator::map_res};

    map_res(anychar, Pixel::try_from)(input)
}
