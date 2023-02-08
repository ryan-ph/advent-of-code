use common::read_input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn solve_part1() -> Option<i32> {
    let input = read_input();
    let mut max: Option<i32> = None;
    let mut curr = 0;

    for line in input.lines() {
        if line.is_empty() {
            if let Some(curr_max) = max {
                if curr > curr_max {
                    max = Some(curr)
                }
            } else {
                max = Some(curr);
            }
            curr = 0
        } else {
            curr += line.parse::<i32>().ok()?;
        }
    }

    max
}

fn solve_part2() -> i32 {
    let mut heap = BinaryHeap::new();
    let mut curr = 0;
    for line in read_input().lines() {
        if line.is_empty() {
            heap.push(Reverse(curr));
            if heap.len() > 3 {
                heap.pop();
            }
            curr = 0;
        } else {
            curr += line.parse::<i32>().unwrap();
        }
    }
    heap.iter().map(|Reverse(x)| x).sum()
}

fn main() {
    println!("part 1: {:?}", solve_part1());
    println!("part 2: {:?}", solve_part2());
}
