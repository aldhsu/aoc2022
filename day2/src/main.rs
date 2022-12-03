use std::str::FromStr;

struct Round {
    player1: Move,
    player2: Move,
}

impl Round {
    fn score(&self) -> u32 {
        let win_points: u32 = self.player2.outcome(&self.player1).into();
        let hand_points: u32 = self.player2.into();

        win_points + hand_points
    }
}

struct Round2 {
    player1: Move,
    outcome: Outcome,
}

impl Round2 {
    const MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn score(&self) -> u32 {
        let win_points: u32 = self.outcome.into();
        let hand_points: u32 = self.player1.complement(self.outcome).into();
        win_points + hand_points
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn complement(&self, outcome: Outcome) -> Move {
        match (self, outcome) {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Lost) => Move::Scissors,

            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Lost) => Move::Rock,

            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Lost) => Move::Paper,
            _ => self.clone()
        }

    }
}

impl From<Move> for u32 {
    fn from(val: Move) -> Self {
        match val {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Error {
    ParseMoveError,
    ParseOutcome,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => return Err(Error::ParseMoveError),
        })
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Lost,
    Draw,
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => return Err(Error::ParseOutcome),
        })
    }
}

impl From<Outcome> for u32 {
    fn from(val: Outcome) -> Self {
        match val {
            Outcome::Win => 6,
            Outcome::Lost => 0,
            Outcome::Draw => 3,
        }
    }
}

impl Move {
    fn outcome(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Paper) => Outcome::Lost,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Scissors) => Outcome::Lost,
            (Move::Scissors, Move::Rock) => Outcome::Lost,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

fn parse(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let player1 = parts.next().unwrap().parse().unwrap();
            let player2 = parts.next().unwrap().parse().unwrap();
            Round { player1, player2 }
        })
        .collect()
}

fn parse2(input: &str) -> Vec<Round2> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let player1 = parts.next().unwrap().parse().unwrap();
            let outcome = parts.next().unwrap().parse().unwrap();
            Round2 { player1, outcome }
        })
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let moves = parse(input);
    let score: u32 = moves.iter().map(Round::score).sum();
    println!("part1: {score}");

    let moves = parse2(input);
    let score: u32 = moves.iter().map(Round2::score).sum();
    println!("part2: {score}");
}
