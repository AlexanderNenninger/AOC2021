use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::isize;
use std::num::ParseIntError;
use std::str;

pub const INPUT: &str = "input/day13.txt";

#[derive(Debug)]
pub struct DotError {
    details: String,
}

impl DotError {
    fn new(msg: &str) -> DotError {
        DotError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self.details)
    }
}

impl Error for DotError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for DotError {
    fn from(err: ParseIntError) -> Self {
        DotError::new(&format!("ERROR: {}", err))
    }
}

impl From<&str> for DotError {
    fn from(err: &str) -> Self {
        DotError::new(err)
    }
}

//------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dot {
    x: isize,
    y: isize,
}

impl Dot {
    pub fn fold_x(mut self, x: isize) -> Self {
        match self.x < x {
            true => return self,
            false => {
                self.x = 2 * x - self.x;
                return self;
            }
        }
    }

    pub fn fold_y(mut self, y: isize) -> Self {
        match self.y < y {
            true => return self,
            false => {
                self.y = 2 * y - self.y;
                return self;
            }
        }
    }

    pub fn fold_from_str(mut self, s: &str) -> Result<Self, DotError> {
        let s = &s[11..];
        let (dir, val_str) = s.split_once("=").ok_or("Could not split on '='.")?;

        let val = val_str.parse::<isize>()?;

        match dir {
            "x" => Ok(self.fold_x(val)),
            "y" => Ok(self.fold_y(val)),
            _ => Err(DotError::new("Could not fold")),
        }
    }
}

impl str::FromStr for Dot {
    type Err = DotError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(",").ok_or("No comma found.")?;

        let x = x_str.parse::<isize>()?;
        let y = y_str.parse::<isize>()?;

        Ok(Self { x: x, y: y })
    }
}

pub fn show(dots: &HashSet<Dot>) {
    let w = dots.iter().map(|dot| dot.x).max().unwrap_or_default() + 1;
    let h = dots.iter().map(|dot| dot.y).max().unwrap_or_default() + 1;
    for y in 0..h {
        for x in 0..w {
            let d = Dot { x: x, y: y };
            if dots.contains(&d) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
}

//--------------------------------------------------------------

pub fn solve_1(data: &String, max_folds: Option<usize>) -> usize {
    #[derive(PartialEq, Eq)]
    enum Parsing {
        Dots,
        Folds,
    }

    let mut reading_state = Parsing::Dots;
    let mut dots = HashSet::new();

    let mut i = 0;
    for line in data.lines() {
        if reading_state == Parsing::Dots {
            let d = line.parse::<Dot>();
            match d {
                Ok(d) => {
                    dots.insert(d);
                }
                Err(d) => {
                    reading_state = Parsing::Folds;
                    continue;
                }
            }
        } else {
            if i >= max_folds.unwrap_or(usize::MAX) {
                break;
            }

            dots = dots
                .into_iter()
                .map(|d: Dot| d.fold_from_str(line).unwrap())
                .collect();
            i += 1;
        }
    }
    show(&dots);
    dots.len()
}

pub fn solve_2(data: &String) {
    solve_1(data, None);
}

mod tests {
    use super::*;
    #[allow(unused)]
    use std::fs;
    #[allow(unused)]
    const INPUT_TEST: &str = "input/day13_test.txt";

    #[test]
    fn test_1() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let result = solve_1(&data, Some(1));

        assert_eq!(result, 17)
    }

    #[test]
    fn test_2() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        solve_2(&data);
    }

    #[test]
    fn test_dot_from_str() {
        let s = "20,13";
        let d = s.parse::<Dot>().unwrap();
        assert_eq!(d, Dot { x: 20, y: 13 })
    }

    #[test]
    fn test_fold() {
        let mut d = Dot { x: 20, y: 12 };
        d = d.fold_x(10);
        d = d.fold_y(6);

        assert_eq!(d, Dot { x: 0, y: 0 });

        let mut d = Dot { x: 2, y: 3 };
        d = d.fold_x(10);
        d = d.fold_y(6);

        assert_eq!(d, Dot { x: 2, y: 3 });

        let mut d = Dot { x: 12, y: 8 };
        d = d.fold_x(10);
        d = d.fold_y(6);

        assert_eq!(d, Dot { x: 8, y: 4 });
    }
    #[test]
    fn test_dot_fold_from_str() {
        let s = "20,12";
        let mut d = s.parse::<Dot>().unwrap();
        assert_eq!(d, Dot { x: 20, y: 12 });

        let s1 = "fold along y=7";
        let s2 = "fold along x=5";

        d = d.fold_from_str(s1).unwrap();
        d = d.fold_from_str(s2).unwrap();

        assert_eq!(d, Dot { x: -10, y: 2 })
    }
}
