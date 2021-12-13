#[allow(unused_imports)]
use std::fmt;

pub const INPUT: &str = "input/day10.txt";

enum StackError<Parenthesis> {
    Mismatch(Parenthesis),
    Empty,
    Parse,
}

enum Parenthesis {
    Round,
    Square,
    Curly,
    Tri,
}

impl fmt::Display for Parenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Round => write!(f, "{}", ")"),
            Self::Square => write!(f, "{}", "]"),
            Self::Curly => write!(f, "{}", "}"),
            Self::Tri => write!(f, "{}", ">"),
        }
    }
}

struct ParStack(Vec<Parenthesis>);

impl ParStack {
    fn new() -> Self {
        Self(vec![])
    }

    fn check(&mut self, c: &char) -> Result<Parenthesis, StackError<char>> {
        match c {
            '(' => {
                self.0.push(Parenthesis::Round);
                Ok(Parenthesis::Round)
            }
            ')' => match self.0.pop() {
                Some(Parenthesis::Round) => Ok(Parenthesis::Round),
                Some(_) => Err(StackError::Mismatch(*c)),
                None => Err(StackError::Empty),
            },

            '[' => {
                self.0.push(Parenthesis::Square);
                Ok(Parenthesis::Square)
            }
            ']' => match self.0.pop() {
                Some(Parenthesis::Square) => Ok(Parenthesis::Square),
                Some(_) => Err(StackError::Mismatch(*c)),
                None => Err(StackError::Empty),
            },

            '{' => {
                self.0.push(Parenthesis::Curly);
                Ok(Parenthesis::Curly)
            }
            '}' => match self.0.pop() {
                Some(Parenthesis::Curly) => Ok(Parenthesis::Curly),
                Some(_) => Err(StackError::Mismatch(*c)),
                None => Err(StackError::Empty),
            },

            '<' => {
                self.0.push(Parenthesis::Tri);
                Ok(Parenthesis::Tri)
            }
            '>' => match self.0.pop() {
                Some(Parenthesis::Tri) => Ok(Parenthesis::Tri),
                Some(_) => Err(StackError::Mismatch(*c)),
                None => Err(StackError::Empty),
            },
            _ => return Err(StackError::Parse),
        }
    }

    fn complete(mut self) -> usize {
        let mut pts = 0;
        while self.0.len() > 0 {
            pts *= 5;

            let par = self.0.pop().unwrap();
            pts += match par {
                Parenthesis::Round => 1,
                Parenthesis::Square => 2,
                Parenthesis::Curly => 3,
                Parenthesis::Tri => 4,
            };
        }
        pts
    }
}

fn check_line(line: &str) -> usize {
    let mut stack = ParStack::new();
    for c in line.chars() {
        return match stack.check(&c) {
            Ok(_) => continue,
            Err(StackError::Empty) => continue, // We ran out of characters
            Err(StackError::Parse) => continue, // Unknown character
            Err(StackError::Mismatch(c)) => match c {
                // Corrupted line!!
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
        };
    }
    return 0;
}

fn complete_line(line: &str) -> Option<usize> {
    let mut stack = ParStack::new();
    for c in line.chars() {
        return match stack.check(&c) {
            Ok(_) => continue,                    // Next char
            Err(StackError::Empty) => None,       // We ran out of characters
            Err(StackError::Parse) => continue,   // Unknown character
            Err(StackError::Mismatch(_)) => None, // Corrupt line
        };
    }
    return Some(stack.complete());
}

pub fn solve_1(data: &String) -> usize {
    let mut pts = 0;
    for line in data.lines() {
        pts += check_line(line);
    }
    pts
}

pub fn solve_2(data: &String) -> usize {
    let mut pts = vec![];
    for line in data.lines() {
        match complete_line(line) {
            Some(res) => pts.push(res),
            None => continue,
        };
    }
    pts.sort();
    pts[pts.len() / 2]
}

mod tests {
    #[allow(unused)]
    pub const INPUT_TEST: &str = "input/day10_test.txt";

    #[allow(unused_imports)]
    use super::{solve_1, solve_2};

    #[test]
    fn test_1() {
        use std::fs;
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let res = solve_1(&data);
        println!("{}", &res);
        assert_eq!(res, 26397)
    }

    #[test]
    fn test_2() {
        use std::fs;
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let res = solve_2(&data);
        println!("{}", res);
        assert_eq!(288957, res)
    }
}
