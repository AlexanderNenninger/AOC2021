use std::{
    error::Error,
    fmt,
    fmt::{Debug, Display, Formatter},
    fs,
};

use super::ReadStr;

#[derive(Debug, PartialEq)]
struct Problem(Vec<isize>);

impl ReadStr for Problem {
    type Err = Box<dyn std::error::Error>;
    fn read_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data: Result<Vec<_>, _> = s
            .trim()
            .split(',')
            .map(|c| isize::from_str_radix(c, 10))
            .collect();
        return Ok(Problem(data?));
    }
}

impl Problem {
    fn median(&self) -> isize {
        let mut s = self.0.clone();
        s.sort();
        return s[s.len() / 2];
    }

    fn solve_part_1(self) -> isize {
        let m = self.median();
        self.0.into_iter().map(|x| (x - m).abs()).sum()
    }

    fn solve_part_2(mut self) -> isize {
        self.0.sort();
        let xs = self.0;
        let upper_bound = xs[xs.len() - 1];

        let mut cbest = isize::MAX;
        for x in 0..upper_bound {
            let c = cost(&xs, x);
            if c < cbest {
                cbest = c;
            }
        }
        cbest
    }
}

#[derive(Debug)]
struct MathError;

impl Display for MathError {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(&mut f, "[ERROR] MathError")
    }
}

impl std::error::Error for MathError {}

fn fixpoint_iter<F: Fn(f64) -> f64>(mut x: f64, f: F, s: f64) -> Result<f64, MathError> {
    let f = |x| f(x) / s;

    for i in 1..1000 {
        let y = f(x);
        x = y;
        if ((x - y) as f32) < f32::EPSILON {
            return Ok(x);
        }
    }
    return Err(MathError);
}

fn gdc<F: Fn(f64) -> f64>(mut x: f64, df: F, lambda: f64) -> Result<f64, MathError> {
    for i in 1..100000 {
        let y = df(x);
        x = x - lambda * y;
        if (y as f32) < f32::EPSILON {
            return Ok(x);
        }
    }
    return Err(MathError);
}

fn cost(xs: &Vec<isize>, s: isize) -> isize {
    let mut result = 0;
    for x in xs.iter() {
        let e = isize::abs(x - s);
        result += e * (e + 1) / 2
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT_TEST: &str = "input/day07_test.txt";
    const INPUT: &str = "input/day07.txt";

    fn load_test_data() -> String {
        fs::read_to_string(INPUT_TEST).unwrap()
    }

    fn load_data() -> String {
        fs::read_to_string(INPUT).unwrap()
    }

    #[test]
    fn read_str() {
        let data = load_test_data();
        let p = Problem::read_str(&data).unwrap();

        println!("{:?}", &p);
    }

    #[test]
    fn test_part_1() {
        let data = load_test_data();
        let p = Problem::read_str(&data).unwrap();

        println!("{}", p.solve_part_1())
    }

    #[test]
    fn part_1() {
        let data = load_data();
        let p = Problem::read_str(&data).unwrap();

        println!("{}", p.solve_part_1())
    }

    #[test]
    fn test_cost() {
        let data = load_test_data();
        let p = Problem::read_str(&data).unwrap();

        let c = cost(&p.0, 5);

        println!("Cost: {}", c)
    }

    #[test]
    fn test_part_2() {
        let data = load_test_data();
        let p = Problem::read_str(&data).unwrap();

        println!("{}", p.solve_part_2());
    }

    #[test]
    fn part_2() {
        let data = load_data();
        let p = Problem::read_str(&data).unwrap();

        println!("{}", p.solve_part_2());
    }
}
