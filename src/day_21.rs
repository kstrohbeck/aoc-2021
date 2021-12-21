use nom::IResult;
use std::collections::HashMap;

pub fn star_1(data: String) {
    let (mut p1, mut p2) = parse(&data);
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut rolled = 0;
    let mut dice = DetDice(1);
    let diff = loop {
        p1 = (p1 + dice.roll_three() - 1) % 10 + 1;
        rolled += 3;
        p1_score += p1;
        if p1_score >= 1000 {
            break p2_score * rolled;
        }
        p2 = (p2 + dice.roll_three() - 1) % 10 + 1;
        rolled += 3;
        p2_score += p2;
        if p2_score >= 1000 {
            break p1_score * rolled;
        }
    };
    println!("{}", diff);
}

pub fn star_2(data: String) {
    let (mut p1, mut p2) = parse(&data);
    let mut states = HashMap::new();
    states.insert((PlayerState::new(p1), PlayerState::new(p2)), 1);
    let mut p1_victories: u64 = 0;
    let mut p2_victories: u64 = 0;
    let mut new_states = HashMap::new();
    while !states.is_empty() {
        // Do P1 move.
        for ((p1, p2), num_univ) in states.drain() {
            for (roll, branches) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let p1 = p1.after_roll(roll);
                let num_univ = num_univ * branches;
                if p1.score >= 21 {
                    p1_victories += num_univ;
                } else {
                    *new_states.entry((p1, p2)).or_insert(0) += num_univ;
                }
            }
        }

        // Do P2 move.
        for ((p1, p2), num_univ) in new_states.drain() {
            for (roll, branches) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let p2 = p2.after_roll(roll);
                let num_univ = num_univ * branches;
                if p2.score >= 21 {
                    p2_victories += num_univ;
                } else {
                    *states.entry((p1, p2)).or_insert(0) += num_univ;
                }
            }
        }
    }
    println!("{}", p1_victories.max(p2_victories));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PlayerState {
    position: u64,
    score: u64,
}

impl PlayerState {
    fn new(position: u64) -> Self {
        Self { position, score: 0 }
    }

    fn after_roll(self, roll: u64) -> Self {
        let position = (self.position + roll - 1) % 10 + 1;
        Self {
            position,
            score: self.score + position,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DetDice(u64);

impl DetDice {
    fn roll(&mut self) -> u64 {
        let ret = self.0;
        self.0 = self.0 % 100 + 1;
        ret
    }

    fn roll_three(&mut self) -> u64 {
        let roll_1 = self.roll();
        let roll_2 = self.roll();
        let roll_3 = self.roll();
        roll_1 + roll_2 + roll_3
    }
}

fn parse(input: &str) -> (u64, u64) {
    super::utils::parse(positions, input)
}

fn positions(input: &str) -> IResult<&str, (u64, u64)> {
    use nom::{character::complete::multispace1, sequence::separated_pair};

    separated_pair(start_pos, multispace1, start_pos)(input)
}

fn start_pos(input: &str) -> IResult<&str, u64> {
    use nom::{
        bytes::complete::tag,
        character::complete::u64 as u64_,
        sequence::{delimited, preceded},
    };

    preceded(
        delimited(tag("Player "), u64_, tag(" starting position: ")),
        u64_,
    )(input)
}
