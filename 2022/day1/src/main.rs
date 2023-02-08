use common::read_input;

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
    0
}

fn main() {
    println!("part 1: {}", solve_part1().unwrap());
    println!("part 2: {}", solve_part2());
}
