use nom::IResult;

pub fn star_1(data: String) {
    print_fish_count(data, 80);
}

pub fn star_2(data: String) {
    print_fish_count(data, 256);
}

fn print_fish_count(data: String, days: u64) {
    let fish = parse(&data);
    let fish = Fish::new(fish);
    let fish = fish.evolve(days);
    println!("{}", fish.count());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fish([u64; 9]);

impl Fish {
    fn new(fish: Vec<u64>) -> Self {
        let mut arr = [0; 9];
        for f in fish {
            arr[f as usize] += 1;
        }
        Self(arr)
    }

    fn evolve(mut self, days: u64) -> Self {
        // Step over days in 9-day cycles.
        for _ in 0..(days / 9) {
            self = self.step_9();
        }

        // Single step the remaining days.
        for _ in 0..(days % 9) {
            self = self.step();
        }

        self
    }

    fn step(self) -> Self {
        let mut new = [0; 9];

        // Decrease timer for all fish.
        new[..8].clone_from_slice(&self.0[1..]);

        // Reset timer for spawning fish.
        new[6] += self.0[0];

        // Spawn new fish.
        new[8] = self.0[0];

        Self(new)
    }

    fn step_9(self) -> Self {
        let Self(x) = self;
        Self([
            x[0] + x[2],
            x[1] + x[3],
            x[2] + x[4],
            x[3] + x[5],
            x[4] + x[6],
            x[5] + x[7] + x[0],
            x[6] + x[8] + x[1],
            x[7] + x[0],
            x[8] + x[1],
        ])
    }

    fn count(self) -> u64 {
        self.0.iter().sum()
    }
}

fn parse(input: &str) -> Vec<u64> {
    fish(input).unwrap().1
}

fn fish(input: &str) -> IResult<&str, Vec<u64>> {
    use nom::{bytes::complete::tag, character::complete::u64 as u64_, multi::separated_list0};

    separated_list0(tag(","), u64_)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_works() {
        let fish = Fish([1, 0, 0, 0, 1, 1, 2, 0, 2]);
        assert_eq!(Fish([0, 0, 0, 1, 1, 2, 1, 2, 1]), fish.step());
    }
}
