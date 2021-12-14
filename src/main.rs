mod day14;

use std::fs;

fn main() {
    let data = fs::read_to_string(day14::tests::INPUT_TEST).unwrap();
    let result_1 = day14::solve_1(&data);
    println!("Part One: {}", result_1);
    let result_2 = day14::solve_2(&data);
    println!("Part Two: {}", result_2);
}
