use common::read_input;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
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

trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

trait Play {
    fn play(&self, other: &Self) -> GameResult;
}

impl Play for Hand {
    fn play(&self, other: &Self) -> GameResult {
        if self.beats() == *other {
            GameResult::Win
        } else if other.beats() == *self {
            GameResult::Lose
        } else {
            GameResult::Draw
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
                let hand_score = match turn[1] {
                    Hand::Rock => 1,
                    Hand::Paper => 2,
                    Hand::Scissors => 3,
                };
                let result_score = match turn[1].play(&turn[0]) {
                    GameResult::Win => 6,
                    GameResult::Lose => 0,
                    GameResult::Draw => 3,
                };
                hand_score + result_score
            } else {
                panic!("malformed turn: {:?}", turn);
            }
        })
        .sum()
}

fn solve_part2() -> i32 {
    let mut ret = 0;
    ret
}

fn main() {
    println!("part 1: {:?}", solve_part1());
    println!("part 2: {}", solve_part2());
}
