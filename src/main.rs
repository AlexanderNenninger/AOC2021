mod day10;

use std::fs;

fn main() {
    let data = fs::read_to_string(day10::INPUT).unwrap();
    let result_1 = day10::solve_1(&data);
    println!("{}", result_1);

    let result_2 = day10::solve_2(&data);
    println!("{}", result_2);
}
