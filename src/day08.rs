#![allow(dead_code, unused)]

use std::fmt;
use std::str;

const VALID_NUMBERS: [u8; 10] = [
    //76543210
    0b01110111u8, // 0
    0b00100010u8, // 1
    0b01011101u8, // 2
    0b01101101u8, // 3
    0b00101110u8, // 4
    0b01101011u8, // 5
    0b01111011u8, // 6
    0b00100101u8, // 7
    0b01111111u8, // 8
    0b01101111u8,
];

#[derive(Debug)]
struct Digit(u8);

impl Digit {
    fn num_elements(&self) -> Result<u8, u8> {
        match self.0 {
            0 => Ok(6),
            1 => Ok(2),
            2 => Ok(5),
            3 => Ok(5),
            4 => Ok(4),
            5 => Ok(5),
            6 => Ok(6),
            7 => Ok(3),
            8 => Ok(7),
            9 => Ok(6),
            _ => Err(self.0),
        }
    }

    fn from_cfg(cfg: u8) -> Result<Self, ()> {
        for (i, vn) in VALID_NUMBERS.iter().enumerate() {
            if cfg == *vn {
                return Ok(Digit(i as u8));
            }
        }
        return Err(());
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 7 {
            return Err(());
        }

        for c in s.chars() {
            if (c as u8) < ('a' as u8) || (c as u8) > ('g' as u8) {
                return Err(());
            }
        }

        match s.trim().len() {
            2 => Ok(Digit(1)),
            4 => Ok(Digit(4)),
            3 => Ok(Digit(7)),
            7 => Ok(Digit(8)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Display {
    input: Vec<Option<Digit>>,
    output: Vec<Option<Digit>>,
}

impl Display {
    fn count_parsed_output(&self) -> usize {
        let mut acc: usize = 0;
        for maybe_d in self.output.iter() {
            if let Some(_) = maybe_d {
                acc += 1;
            }
        }
        acc
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.input.iter() {
            if let Some(d) = i {
                write!(f, "{}", d.0)?;
            } else {
                write!(f, "?")?;
            }
        }
        write!(f, " | ")?;
        for i in self.output.iter() {
            if let Some(d) = i {
                write!(f, "{}", d.0)?;
            } else {
                write!(f, "?")?;
            }
        }
        Ok(())
    }
}

impl str::FromStr for Display {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input_str, output_str) = s.split_once(" | ").ok_or(())?;

        let mut input = Vec::with_capacity(16);
        for dstr in input_str.split_whitespace() {
            let d: Option<Digit> = dstr.parse().ok();
            input.push(d)
        }

        let mut output = Vec::with_capacity(16);
        for dstr in output_str.split_whitespace() {
            let d: Option<Digit> = dstr.parse().ok();
            output.push(d)
        }

        Ok(Display { input, output })
    }
}

struct Problem {
    displays: Vec<Display>,
}

impl Problem {
    fn count_parsed_output(&self) -> usize {
        let mut acc = 0;
        for disp in self.displays.iter() {
            acc += disp.count_parsed_output()
        }
        acc
    }
}

impl str::FromStr for Problem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut displays = Vec::with_capacity(200);
        for line in s.lines() {
            displays.push(line.parse::<Display>()?)
        }
        Ok(Problem { displays })
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for disp in self.displays.iter() {
            writeln!(f, "{}", disp)?
        }
        Ok(())
    }
}

fn is_valid(cfg: u8) -> bool {
    for vn in VALID_NUMBERS {
        if cfg == vn {
            return true;
        }
    }
    false
}

fn get(bits: u8, i: u8) -> u8 {
    (bits >> i) % 2
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[test]
    fn test_get() {
        let bits = VALID_NUMBERS;
        for bit in bits {
            println!("{:1b}", get(bit, 3))
        }
    }

    const TEST_INPUT: &str = "input/day08_test.txt";
    const INPUT: &str = "input/day08.txt";

    #[test]
    fn load_data() {
        let data = fs::read_to_string(TEST_INPUT).unwrap();
    }

    #[test]
    fn display_digits() {
        for i in 0..10 {
            let d = Digit(i);
            println!("{}", d);
        }
    }

    #[test]
    fn parse_digit() {
        let data = "gcfb";
        let d: Digit = data.parse().unwrap();

        println!("{}", d)
    }

    #[test]
    fn parse_display() {
        let data =
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let disp: Display = data.parse().unwrap();

        println!("{} ... count parsed: {}", disp, disp.count_parsed_output());
    }

    #[test]
    fn parse_problem() {
        let data = fs::read_to_string(TEST_INPUT).unwrap();
        let p: Problem = data.parse().unwrap();
        println!("{}, Count Parsed: {}", p, p.count_parsed_output())
    }

    #[test]
    fn solve_1() {
        let data = fs::read_to_string(INPUT).unwrap();
        let p: Problem = data.parse().unwrap();
        println!("{}, Count Parsed: {}", p, p.count_parsed_output())
    }
}
