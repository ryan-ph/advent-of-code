use common::read_input;

fn parse_coor(coor: &str) -> (i32, i32) {
    let vec: Vec<i32> = coor.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    if let [x, y] = vec[..] {
        (x, y)
    } else {
        panic!("invalid coordinate format: {}", coor);
    }
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    if let [coor1_raw, _, coor2_raw] = line.split_whitespace().collect::<Vec<&str>>()[..] {
        (parse_coor(coor1_raw), parse_coor(coor2_raw))
    } else {
        panic!("invalid line: {}", line);
    }
}

fn solve_part1() -> i32 {
    let mut ret = 0;
    ret
}

fn solve_part2() -> i32 {
    let mut ret = 0;
    ret
}

fn main() {
    println!("part 1: {}", solve_part1());
    println!("part 2: {}", solve_part2());
}
