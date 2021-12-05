use std::fs;


pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}


pub fn read_input() -> String {
    read_file("input.txt")
}
