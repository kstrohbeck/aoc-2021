use nom::IResult;

pub fn star_1(data: String) {
    let commands = parse(&data);
    let mut horiz = 0;
    let mut depth = 0;
    for command in commands {
        match command.direction {
            Direction::Forward => horiz += command.distance,
            Direction::Down => depth += command.distance,
            Direction::Up => depth -= command.distance,
        }
    }
    println!("{}", horiz * depth);
}

pub fn star_2(data: String) {
    let commands = parse(&data);
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        let dist = command.distance as i64;
        match command.direction {
            Direction::Forward => {
                horiz += dist;
                depth += aim * dist;
            }
            Direction::Down => aim += dist,
            Direction::Up => aim -= dist,
        }
    }
    println!("{}", horiz * depth);
}

fn parse(data: &str) -> Vec<Command> {
    super::utils::parse(commands, data)
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    super::utils::lines(command)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Command {
    direction: Direction,
    distance: u64,
}

fn command(input: &str) -> IResult<&str, Command> {
    use nom::{
        character::complete::{space1, u64 as u64_},
        combinator::map,
        sequence::separated_pair,
    };

    map(separated_pair(direction, space1, u64_), |(dir, dis)| {
        Command {
            direction: dir,
            distance: dis,
        }
    })(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Forward,
    Down,
    Up,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    use nom::{branch::alt, bytes::complete::tag, combinator::value};

    alt((
        value(Direction::Forward, tag("forward")),
        value(Direction::Down, tag("down")),
        value(Direction::Up, tag("up")),
    ))(input)
}
