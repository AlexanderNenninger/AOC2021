mod day13;

use std::fs;

fn main() {
    let data = fs::read_to_string(day13::INPUT).unwrap();
    let result_1 = day13::solve_1(&data, Some(1));
    println!("{}", result_1);

    day13::solve_2(&data);
}
