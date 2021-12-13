#[allow(unused_imports)]

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

struct DelimStack(Vec<Parenthesis>);

impl DelimStack {
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
}

fn check_line(line: &str) -> usize {
    let mut stack = DelimStack::new();
    for c in line.chars() {
        return match stack.check(&c) {
            Ok(_) => continue,
            Err(StackError::Empty) => continue,
            Err(StackError::Parse) => continue,
            Err(StackError::Mismatch(c)) => match c {
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

pub fn solve_1(data: String) -> usize {
    let mut pts = 0;
    for line in data.lines() {
        pts += check_line(line);
    }
    pts
}

mod tests {
    #[allow(unused)]
    pub const INPUT_TEST: &str = "input/day10_test.txt";
    #[allow(unused_imports)]
    use super::solve_1;
    #[test]
    fn name() {
        use std::fs;
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let res = solve_1(data);
        println!("{}", res);
    }
}
