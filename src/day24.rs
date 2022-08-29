use std::{
    error::{self, Error},
    fmt, fs,
    str::FromStr,
};
use Instruction::*;
use Value::*;

fn num_to_digits(mut num: i64) -> [i64; 14] {
    let mut rem;
    let mut digits = [0; 14];
    for i in 0..14 {
        (num, rem) = (num / 10, num % 10);
        digits[i] = rem;
    }
    digits
}

#[derive(Debug)]
struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}
impl error::Error for ParseError {}

#[derive(Debug)]
struct EvalError(&'static str);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EvalError: {}", self.0)
    }
}
impl error::Error for EvalError {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Value {
    Index(usize),
    Literal(i64),
}

impl Value {
    fn from_option_str(s: Option<&str>) -> Result<Self, ParseError> {
        if let Some(s) = s {
            return s.parse();
        }
        Err(ParseError("Empty input."))
    }

    fn equals_i64(self, other: i64) -> bool {
        match self {
            Index(_) => false,
            Literal(v) => v == other,
        }
    }
}

impl FromStr for Value {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<i64>() {
            return Ok(Literal(num));
        } else {
            return Ok(match s.trim() {
                "w" => Index(0),
                "x" => Index(1),
                "y" => Index(2),
                "z" => Index(3),
                _ => return Err(ParseError("Unknown register.")),
            });
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
    NoOp,
}

impl Instruction {
    fn eval(
        &self,
        memory: &mut [i64; 4],
        input: &[i64],
        input_ptr: &mut usize,
    ) -> Result<(), EvalError> {
        match self {
            &Inp(v) => Self::input(v, memory, input, input_ptr),
            &Add(idx, v) => Self::add(idx, v, memory),
            &Mul(idx, v) => Self::mul(idx, v, memory),
            &Div(idx, v) => Self::div(idx, v, memory),
            &Mod(idx, v) => Self::modulo(idx, v, memory),
            &Eql(idx, v) => Self::eql(idx, v, memory),
            &NoOp => Ok(()),
        }
    }

    fn input(
        idx: usize,
        memory: &mut [i64; 4],
        input: &[i64],
        input_ptr: &mut usize,
    ) -> Result<(), EvalError> {
        memory[idx] = input[*input_ptr];
        Ok(())
    }

    fn add(idx: usize, v: Value, memory: &mut [i64; 4]) -> Result<(), EvalError> {
        match v {
            Index(idy) => memory[idx] += memory[idy],
            Literal(v) => memory[idx] += v,
        };
        return Ok(());
    }

    fn mul(idx: usize, v: Value, memory: &mut [i64; 4]) -> Result<(), EvalError> {
        match v {
            Index(idy) => memory[idx] *= memory[idy],
            Literal(v) => memory[idx] *= v,
        };
        return Ok(());
    }

    fn div(idx: usize, v: Value, memory: &mut [i64; 4]) -> Result<(), EvalError> {
        match v {
            Index(idy) => memory[idx] /= memory[idy],
            Literal(v) => memory[idx] /= v,
        };
        return Ok(());
    }

    fn modulo(idx: usize, v: Value, memory: &mut [i64; 4]) -> Result<(), EvalError> {
        match v {
            Index(idy) => memory[idx] %= memory[idy],
            Literal(v) => memory[idx] %= v,
        };
        return Ok(());
    }

    fn eql(idx: usize, v: Value, memory: &mut [i64; 4]) -> Result<(), EvalError> {
        match v {
            Index(idy) => memory[idx] = (memory[idx] == memory[idy]) as i64,
            Literal(v) => memory[idx] = (memory[idx] == v) as i64,
        };
        return Ok(());
    }

    fn is_identity(&self) -> bool {
        match self {
            &Inp(v) => v == 0,
            &Add(idx, v) => v.equals_i64(0),
            &Mul(idx, v) => v.equals_i64(1),
            &Div(idx, v) => v.equals_i64(1),
            &Mod(idx, v) => false,
            &Eql(idx, v) => false,
            &NoOp => true,
        }
    }
}

fn idx_from_option_str(s: Option<&str>) -> Result<usize, Box<dyn Error>> {
    if let Some(s) = s {
        return Ok(match s {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => return Err(Box::new(ParseError("Could not parse usize."))),
        });
    }

    Err(Box::new(ParseError("Could not parse usize.")))
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR: ParseError = ParseError("Invalid Instruction.");

        let mut parts = s.split(" ");
        let op = parts.next().ok_or(ERROR)?;
        Ok(match op {
            "inp" => Inp(idx_from_option_str(parts.next())?),
            "add" => Add(
                idx_from_option_str(parts.next())?,
                Value::from_option_str(parts.next())?,
            ),
            "mul" => Mul(
                idx_from_option_str(parts.next())?,
                Value::from_option_str(parts.next())?,
            ),
            "div" => Div(
                idx_from_option_str(parts.next())?,
                Value::from_option_str(parts.next())?,
            ),
            "mod" => Mod(
                idx_from_option_str(parts.next())?,
                Value::from_option_str(parts.next())?,
            ),
            "eql" => Eql(
                idx_from_option_str(parts.next())?,
                Value::from_option_str(parts.next())?,
            ),
            _ => return Err(Box::new(ERROR)),
        })
    }
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions: instructions,
        }
    }

    fn optimize(self) -> Self {
        let instructions = self
            .instructions
            .into_iter()
            .filter(|instruction| instruction.is_identity())
            .collect();
        Program::new(instructions)
    }

    fn eval(&self, input: &[i64]) -> Result<[i64; 4], EvalError> {
        let mut memory = [0i64; 4];
        let mut input_ptr = 0usize;

        for instruction in self.instructions.iter() {
            instruction.eval(&mut memory, input, &mut input_ptr)?
        }
        Ok(memory)
    }

    fn check_model_number(&self, num: i64) -> Result<bool, EvalError> {
        let input = num_to_digits(num);
        if !input.len() == 14 {
            return Err(EvalError("Model number has exactly 14 digits."));
        }
        if input.iter().any(|&i| i == 0) {
            return Ok(false);
        }
        let result = self.eval(&input)?;
        Ok(result[3] == 0)
    }
}

impl FromStr for Program {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];
        for line in s.trim().lines() {
            instructions.push(line.trim().parse::<Instruction>()?)
        }
        Ok(Program::new(instructions))
    }
}

pub fn part_1() -> usize {
    const INPUT_FILE: &str = "input/day24.txt";
    let data = fs::read_to_string(INPUT_FILE).unwrap();
    let program: Program = data.parse::<Program>().unwrap().optimize();
    for i in 10i64.pow(14)..10i64.pow(15) {
        if program.check_model_number(i).unwrap() {
            return i as usize;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT_FILE: &str = "input/day24.txt";

    #[test]
    fn test_load() {
        const INPUT: &str = "inp x\nmul x -1";
        let program = Program::from_str(INPUT).unwrap();
        println!("{:?}", &program);
    }

    #[test]
    fn test_eval() {
        const INPUT: &str = "inp x\nmul x -1";
        let program = Program::from_str(INPUT).unwrap();

        let input = vec![1i64];
        let result = program.eval(&input).unwrap();

        assert_eq!(result[1], -1);
    }

    #[test]
    fn test_eval_2() {
        const INPUT: &str = "
            inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2
        ";
        let program = Program::from_str(INPUT.trim()).unwrap();

        let input = vec![14i64];
        let result = program.eval(&input).unwrap();

        assert_eq!(result, [1, 1, 1, 0]);
    }

    #[test]
    fn test_optimize() {
        let data = fs::read_to_string(INPUT_FILE).unwrap();
        let program = Program::from_str(&data).unwrap();

        println!("Unoptimized: {}", program.instructions.len());

        let program = program.optimize();
        println!("Optimized: {}", program.instructions.len());

        println!("{:?}", program);
    }

    #[test]
    fn test_model_number() {
        let data = fs::read_to_string(INPUT_FILE).unwrap();
        let program = Program::from_str(&data).unwrap();
        let model_number = 13579246899999;
        let res = program.check_model_number(model_number).unwrap();
        assert_eq!(res, false);
    }
}
