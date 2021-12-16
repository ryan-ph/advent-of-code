use common::read_input;
use std::collections::{HashMap, HashSet};

fn solve_part1() -> i32 {
    let mut vals: HashSet<i32> = HashSet::new();
    for num in read_input().lines().map(|x| x.parse::<i32>().unwrap()) {
        if vals.contains(&num) {
            return (2020 - num) * num;
        } else {
            vals.insert(2020 - num);
        }
    }
    -1
}

fn solve_part2() -> i32 {
    let nums: Vec<i32> = read_input()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut vals: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        vals.insert(2020 - num, i);
    }

    for (i, num1) in nums.iter().enumerate() {
        for (j, num2) in nums.iter().enumerate() {
            let sum: i32 = num1 + num2;
            if let Some(k) = vals.get(&sum) {
                if i == j || i == *k || j == *k {
                    continue;
                }
                return num1 * num2 * (2020 - sum);
            }
        }
    }
    -1
}

fn main() {
    println!("part 1: {}", solve_part1());
    println!("part 2: {}", solve_part2());
}
