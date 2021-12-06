use common::read_input;

fn solve_part1() -> i32 {
    let mut horizontal = 0;
    let mut vert = 0;

    let input = read_input();
    for line in input.lines() {
        if let [action, unit_str] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let unit = unit_str.parse::<i32>().unwrap();
            if action == "forward" {
                horizontal += unit;
            } else if action == "down" {
                vert += unit;
            } else {
                vert -= unit;
            }
        }
    }
    horizontal * vert
}

fn solve_part2() -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    let input = read_input();
    for line in input.lines() {
        if let [action, unit_str] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let unit = unit_str.parse::<i32>().unwrap();
            if action == "forward" {
                horizontal += unit;
                depth += unit * aim;
            } else if action == "down" {
                aim += unit;
            } else {
                aim -= unit;
            }
        }
    }
    horizontal * depth
}

fn main() {
    println!("part 1: {}", solve_part1());
    println!("part 2: {}", solve_part2());
}
