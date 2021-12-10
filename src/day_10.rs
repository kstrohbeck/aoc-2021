use nom::IResult;

pub fn star_1(data: String) {
    let lines = parse(&data);
    let mut total_score = 0;
    for line in lines {
        let mut stack = Vec::new();
        for c in line {
            if c.ty == DelimTy::Open {
                stack.push(c);
            } else {
                let t = stack.pop().unwrap();
                if c.shape != t.shape {
                    total_score += c.shape.corrupted_score();
                    break;
                }
            }
        }
    }
    println!("{}", total_score);
}

pub fn star_2(data: String) {
    let lines = parse(&data);
    let mut scores = Vec::new();
    'outer: for line in lines {
        let mut stack = Vec::new();
        for c in line {
            if c.ty == DelimTy::Open {
                stack.push(c);
            } else {
                let t = stack.pop().unwrap();
                if c.shape != t.shape {
                    continue 'outer;
                }
            }
        }
        let score = stack
            .into_iter()
            .rev()
            .map(|c| c.shape.incomplete_score())
            .fold(0, |acc, s| acc * 5 + s);
        scores.push(score);
    }
    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);
}

fn parse(input: &str) -> Vec<Vec<Delim>> {
    super::utils::parse(lines, input)
}

fn lines(input: &str) -> IResult<&str, Vec<Vec<Delim>>> {
    super::utils::lines(line)(input)
}

fn line(input: &str) -> IResult<&str, Vec<Delim>> {
    use nom::multi::many1;

    many1(delim)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Delim {
    shape: DelimShape,
    ty: DelimTy,
}

impl TryFrom<char> for Delim {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use DelimShape::{Angle, Curly, Paren, Square};
        use DelimTy::{Close, Open};
        match c {
            '(' => Ok(Delim {
                shape: Paren,
                ty: Open,
            }),
            ')' => Ok(Delim {
                shape: Paren,
                ty: Close,
            }),
            '[' => Ok(Delim {
                shape: Square,
                ty: Open,
            }),
            ']' => Ok(Delim {
                shape: Square,
                ty: Close,
            }),
            '{' => Ok(Delim {
                shape: Curly,
                ty: Open,
            }),
            '}' => Ok(Delim {
                shape: Curly,
                ty: Close,
            }),
            '<' => Ok(Delim {
                shape: Angle,
                ty: Open,
            }),
            '>' => Ok(Delim {
                shape: Angle,
                ty: Close,
            }),
            _ => Err(()),
        }
    }
}

fn delim(input: &str) -> IResult<&str, Delim> {
    use nom::{character::complete::anychar, combinator::map_res};

    map_res(anychar, Delim::try_from)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DelimShape {
    Paren,
    Square,
    Curly,
    Angle,
}

impl DelimShape {
    fn corrupted_score(self) -> u64 {
        match self {
            Self::Paren => 3,
            Self::Square => 57,
            Self::Curly => 1197,
            Self::Angle => 25137,
        }
    }

    fn incomplete_score(self) -> u64 {
        match self {
            Self::Paren => 1,
            Self::Square => 2,
            Self::Curly => 3,
            Self::Angle => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DelimTy {
    Open,
    Close,
}
