use common::read_input;

fn solve_part1() -> i32 {
    let input = read_input();
    let mut prev = None;
    let mut count = 0;
    for line in input.lines() {
        let curr = line.parse::<i32>().unwrap();
        if let Some(prev_val) = prev {
            if curr > prev_val {
                count += 1;
            }
        }
        prev = Some(curr);
    }
    count
}

fn solve_part2() -> i32 {
    let mut count = 0;
    count
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
