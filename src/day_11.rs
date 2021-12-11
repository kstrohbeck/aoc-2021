use super::utils::sep_array_10;
use itertools::iproduct;
use nom::IResult;

pub fn star_1(data: String) {
    let mut octos = parse(&data);
    let mut total_flashes = 0;

    for _ in 0..100 {
        let (new_octos, num_flashes) = step(octos);
        octos = new_octos;
        total_flashes += num_flashes;
    }

    println!("{}", total_flashes);
}

pub fn star_2(data: String) {
    let mut octos = parse(&data);

    for i in 0.. {
        let (new_octos, num_flashes) = step(octos);
        octos = new_octos;

        if num_flashes == 100 {
            println!("{}", i);
            break;
        }
    }
}

fn step(mut octos: [[u8; 10]; 10]) -> ([[u8; 10]; 10], usize) {
    let mut num_flashes = 0;
    let mut is_flashing = false;
    for (row, col) in iproduct!(0..10, 0..10) {
        octos[row][col] += 1;
        if octos[row][col] > 9 {
            num_flashes += 1;
            is_flashing = true;
        }
    }

    while is_flashing {
        is_flashing = false;
        let mut new_octos = octos;

        for (row, col) in iproduct!(0..10, 0..10) {
            if octos[row][col] > 9 {
                new_octos[row][col] = 0;
            }

            if new_octos[row][col] == 0 {
                continue;
            }

            let flash_adjust = neighbors(row, col, 10, 10)
                .filter(|(row, col)| octos[*row][*col] > 9)
                .count();
            new_octos[row][col] += flash_adjust as u8;
            if new_octos[row][col] > 9 {
                num_flashes += 1;
                is_flashing = true;
            }
        }

        octos = new_octos;
    }

    (octos, num_flashes)
}

fn neighbors(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let rows_start = if row == 0 { row } else { row - 1 };
    let rows_end = if row >= height - 1 { row } else { row + 1 };
    let cols_start = if col == 0 { col } else { col - 1 };
    let cols_end = if col >= width - 1 { col } else { col + 1 };
    let rows = rows_start..=rows_end;
    let cols = cols_start..=cols_end;
    iproduct!(cols, rows)
        .map(|(c, r)| (r, c))
        .filter(move |(r, c)| *r != row || *c != col)
}

pub fn parse(input: &str) -> [[u8; 10]; 10] {
    super::utils::parse(octopodes, input)
}

pub fn octopodes(input: &str) -> IResult<&str, [[u8; 10]; 10]> {
    use nom::character::complete::line_ending;

    sep_array_10(line_ending, row)(input)
}

pub fn row(input: &str) -> IResult<&str, [u8; 10]> {
    use nom::combinator::success;

    sep_array_10(success(()), digit)(input)
}

pub fn digit(input: &str) -> IResult<&str, u8> {
    use nom::{character::complete::anychar, combinator::map_opt};

    map_opt(anychar, |c| c.to_digit(10).map(|d| d as u8))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod neighbors {
        use super::neighbors;

        #[test]
        fn ul_correct() {
            let ul = neighbors(0, 0, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(1, 0), (0, 1), (1, 1)], ul);
        }

        #[test]
        fn u_correct() {
            let u = neighbors(2, 0, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(1, 0), (3, 0), (1, 1), (2, 1), (3, 1)], u);
        }

        #[test]
        fn ur_correct() {
            let ur = neighbors(5, 0, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(4, 0), (4, 1), (5, 1)], ur);
        }

        #[test]
        fn l_correct() {
            let l = neighbors(0, 2, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(0, 1), (1, 1), (1, 2), (0, 3), (1, 3)], l);
        }

        #[test]
        fn c_correct() {
            let c = neighbors(2, 2, 5, 4).collect::<Vec<_>>();
            assert_eq!(
                vec![
                    (1, 1),
                    (2, 1),
                    (3, 1),
                    (1, 2),
                    (3, 2),
                    (1, 3),
                    (2, 3),
                    (3, 3)
                ],
                c
            );
        }

        #[test]
        fn r_correct() {
            let r = neighbors(5, 2, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(4, 1), (5, 1), (4, 2), (4, 3), (5, 3)], r);
        }

        #[test]
        fn dl_correct() {
            let dl = neighbors(0, 4, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(0, 3), (1, 3), (1, 4)], dl);
        }

        #[test]
        fn d_correct() {
            let d = neighbors(2, 4, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(1, 3), (2, 3), (3, 3), (1, 4), (3, 4)], d);
        }

        #[test]
        fn dr_correct() {
            let dr = neighbors(5, 4, 5, 4).collect::<Vec<_>>();
            assert_eq!(vec![(4, 3), (5, 3), (4, 4)], dr);
        }
    }
}
