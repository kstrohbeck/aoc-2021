use super::utils::sep_array_5;
use nom::{
    character::complete::{multispace1, u8 as u8_},
    multi::separated_list1,
    IResult,
};

pub fn star_1(data: String) {
    let (nums, bingos) = parse(&data);
    let (i, score) = bingos
        .iter()
        .filter_map(|bingo| bingo.score(&nums[..]))
        .min_by_key(|(i, _)| *i)
        .unwrap();
    println!("{}", score * u64::from(nums[i as usize]));
}

pub fn star_2(data: String) {
    let (nums, bingos) = parse(&data);
    let (i, score) = bingos
        .iter()
        .filter_map(|bingo| bingo.score(&nums[..]))
        .max_by_key(|(i, _)| *i)
        .unwrap();
    println!("{}", score * u64::from(nums[i as usize]));
}

fn parse(input: &str) -> (Vec<u8>, Vec<Bingo>) {
    super::utils::parse(document, input)
}

fn document(input: &str) -> IResult<&str, (Vec<u8>, Vec<Bingo>)> {
    use nom::sequence::separated_pair;

    separated_pair(num_list, multispace1, bingos)(input)
}

fn num_list(input: &str) -> IResult<&str, Vec<u8>> {
    use nom::bytes::complete::tag;

    separated_list1(tag(","), u8_)(input)
}

fn bingos(input: &str) -> IResult<&str, Vec<Bingo>> {
    separated_list1(multispace1, bingo)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bingo([[u8; 5]; 5]);

impl Bingo {
    fn score(&self, numbers: &[u8]) -> Option<(u8, u64)> {
        let mut marked = [[false; 5]; 5];
        let coords = numbers
            .iter()
            .enumerate()
            .filter_map(|(i, number)| self.coords(*number).map(|c| (i, c)));

        for (i, (x, y)) in coords {
            marked[y][x] = true;
            let horiz_win = marked[y].iter().all(|m| *m);
            let vert_win = (0..5).map(|y| marked[y][x]).all(|m| m);
            if horiz_win || vert_win {
                let cells = self.0.iter().flatten();
                let marks = marked.iter().flatten();
                let sum = cells
                    .zip(marks)
                    .filter(|(_, mark)| !*mark)
                    .map(|(cell, _)| u64::from(*cell))
                    .sum();
                return Some((i as u8, sum));
            }
        }
        None
    }

    fn coords(&self, number: u8) -> Option<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| ((x, y), cell)))
            .find(|(_, cell)| **cell == number)
            .map(|(c, _)| c)
    }
}

fn bingo(input: &str) -> IResult<&str, Bingo> {
    use nom::{character::complete::line_ending, combinator::map};

    map(sep_array_5(line_ending, bingo_row), Bingo)(input)
}

fn bingo_row(input: &str) -> IResult<&str, [u8; 5]> {
    use nom::{
        character::complete::{space0, space1},
        sequence::preceded,
    };

    preceded(space0, sep_array_5(space1, u8_))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_correct() {
        let nums = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let bingo = Bingo([
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]);
        assert_eq!(Some((11, 188)), bingo.score(&nums[..]));
    }

    #[test]
    fn bingo_parses() {
        let input = concat!(
            "22 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
        );
        let expected = Ok((
            "\n",
            Bingo([
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ]),
        ));
        assert_eq!(expected, bingo(input));
    }
}
