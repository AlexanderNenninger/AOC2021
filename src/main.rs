mod day10;

use std::fs;

fn main() {
    let data = fs::read_to_string(day10::INPUT).unwrap();
    let result = day10::solve_1(data);
    println!("{}", result)
}
