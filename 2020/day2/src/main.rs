use common::read_input;

fn parse_line(line: &str) -> (usize, usize, char, &str) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let bounds = parts[0]
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (
        bounds[0],
        bounds[1],
        parts[1].chars().next().unwrap(),
        parts.last().unwrap(),
    )
}

fn solve_part1() -> usize {
    let mut ret = 0;
    for line in read_input().lines() {
        let (lower, upper, ch, password) = parse_line(&line);
        let count = password.matches(ch).count();
        if count >= lower && count <= upper {
            ret += 1;
        }
    }
    ret
}

fn solve_part2() -> usize {
    let mut ret = 0;
    ret
}

fn main() {
    println!("part 1: {}", solve_part1());
    println!("part 2: {}", solve_part2());
}
