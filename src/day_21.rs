use nom::IResult;
use std::collections::HashMap;

pub fn star_1(data: String) {
    let (p1, p2) = parse(&data);
    let mut p1 = PlayerState::new(p1);
    let mut p2 = PlayerState::new(p2);
    let mut dice = DetDice::new();
    let loser = loop {
        if let Some(x) = step(&mut p1, &p2, &mut dice) {
            break x;
        }
        if let Some(x) = step(&mut p2, &p1, &mut dice) {
            break x;
        }
    };
    let score = loser * dice.times_rolled;
    println!("{}", score);
}

pub fn star_2(data: String) {
    let (mut p1, mut p2) = parse(&data);
    let mut p1_quantum_state = QuantumState::new(PlayerState::new(p1), PlayerState::new(p2));
    let mut p2_quantum_state = QuantumState::empty();
    let mut p1_victories: u64 = 0;
    let mut p2_victories: u64 = 0;
    while !p1_quantum_state.0.is_empty() {
        p1_victories += p1_quantum_state.drain_step(&mut p2_quantum_state);
        p2_victories += p2_quantum_state.drain_step(&mut p1_quantum_state);
    }
    println!("{}", p1_victories.max(p2_victories));
}

fn step(a: &mut PlayerState, b: &PlayerState, dice: &mut DetDice) -> Option<u64> {
    let roll = dice.roll_three();
    let new_a = a.after_roll(roll);
    std::mem::replace(a, new_a);
    if a.score >= 1000 {
        Some(b.score)
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QuantumState(HashMap<(PlayerState, PlayerState), u64>);

impl QuantumState {
    fn empty() -> Self {
        Self(HashMap::new())
    }

    fn new(a: PlayerState, b: PlayerState) -> Self {
        let mut state = HashMap::new();
        state.insert((a, b), 1);
        Self(state)
    }

    fn drain_step(&mut self, next: &mut Self) -> u64 {
        let mut victories = 0;
        for ((a, b), num_univ) in self.0.drain() {
            for (roll, branches) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let a = a.after_roll(roll);
                let num_univ = num_univ * branches;
                if a.score >= 21 {
                    victories += num_univ;
                } else {
                    *next.0.entry((b, a)).or_insert(0) += num_univ;
                }
            }
        }
        victories
    }
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
struct DetDice {
    state: u64,
    times_rolled: u64,
}

impl DetDice {
    fn new() -> Self {
        Self::default()
    }

    fn roll(&mut self) -> u64 {
        let ret = self.state;
        self.state = self.state % 100 + 1;
        self.times_rolled += 1;
        ret
    }

    fn roll_three(&mut self) -> u64 {
        let roll_1 = self.roll();
        let roll_2 = self.roll();
        let roll_3 = self.roll();
        roll_1 + roll_2 + roll_3
    }
}

impl Default for DetDice {
    fn default() -> Self {
        Self {
            state: 1,
            times_rolled: 0,
        }
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
