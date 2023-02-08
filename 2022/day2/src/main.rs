use common::read_input;
use std::cmp::PartialEq;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        match input {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(input: &str) -> Result<GameResult, Self::Err> {
        match input {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(()),
        }
    }
}

trait Score {
    fn score(&self) -> i32;
}

impl Score for Hand {
    fn score(&self) -> i32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Score for GameResult {
    fn score(&self) -> i32 {
        match *self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}

trait Playable {
    fn beats(&self) -> Self;
    fn loses(&self) -> Self;
    fn draws(&self) -> Self;
    fn play(&self, other: &Self) -> GameResult;
    fn opponent_result(&self, result: &GameResult) -> Hand;
}

impl Playable for Hand {
    fn beats(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn draws(&self) -> Self {
        *self
    }

    fn play(&self, other: &Self) -> GameResult {
        if self.beats() == *other {
            GameResult::Win
        } else if other.beats() == *self {
            GameResult::Lose
        } else {
            GameResult::Draw
        }
    }

    fn opponent_result(&self, result: &GameResult) -> Hand {
        match result {
            GameResult::Win => self.loses(),
            GameResult::Draw => *self,
            GameResult::Lose => self.beats(),
        }
    }
}

fn solve_part1() -> i32 {
    read_input()
        .lines()
        .map(|line| {
            let turn: Vec<Hand> = line
                .split(' ')
                .map(|raw| Hand::from_str(raw).expect(&format!("could not parse hand: {}", raw)))
                .collect();
            if turn.len() == 2 {
                let op = &turn[0];
                let me = &turn[1];
                me.score() + me.play(op).score()
            } else {
                panic!("malformed turn: {:?}", turn);
            }
        })
        .sum()
}

fn solve_part2() -> i32 {
    read_input()
        .lines()
        .map(|line| {
            let turn = line.split(' ').collect::<Vec<_>>();
            if turn.len() == 2 {
                let op =
                    Hand::from_str(turn[0]).expect(&format!("could not parse hand: {}", turn[0]));
                let result = GameResult::from_str(turn[1])
                    .expect(&format!("could not parse result: {}", turn[1]));
                let me = op.opponent_result(&result);
                me.score() + result.score()
            } else {
                panic!("malformed turn; {:?}", turn);
            }
        })
        .sum()
}

fn main() {
    println!("part 1: {:?}", solve_part1());
    println!("part 2: {}", solve_part2());
}
