use common::read_input;

fn solve_part1() -> i32 {
    let input = read_input();
    let mut prev = None;
    let mut count = 0;
    for curr in input.lines().map(|x| x.parse::<i32>().unwrap()) {
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
    let input = read_input();
    let depths: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut count = 0;
    let mut prev = None;
    for curr_window in depths.windows(3) {
        let curr = curr_window.iter().sum::<i32>();
        if let Some(prev_sum) = prev {
            if curr > prev_sum {
                count += 1;
            }
        }
        prev = Some(curr);
    }
    count
}

fn main() {
    println!("part 1: {}", solve_part1());
    println!("part 2: {}", solve_part2());
}
