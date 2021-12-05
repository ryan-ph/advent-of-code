use std::fs;


pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}


pub fn read_part1() -> String {
    read_file("part1.txt")
}

pub fn read_part2() -> String {
    read_file("part2.txt")
}
