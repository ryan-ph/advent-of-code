use common::read_input;
use std::collections::HashSet;

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
    let mut ret = 0;
    ret
}

fn main() {
    println!("part 1: {}", solve_part1());
    println!("part 2: {}", solve_part2());
}
