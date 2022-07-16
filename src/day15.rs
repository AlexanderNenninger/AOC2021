#![allow(dead_code)]
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct Cavern {
    risk_level: Vec<Vec<usize>>,
}

impl FromStr for Cavern {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut risk_level: Vec<Vec<usize>> = Vec::new();
        for line in s.trim().lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).ok_or("Non digit character.")? as usize);
            }
            risk_level.push(row);
        }
        Ok(Cavern { risk_level })
    }
}

impl Cavern {
    // Find the lowest risk pyth by dynamic programming.
    // The risk level of a field is equal to it's own risk level plus the minimum
    // risk level of all adjacent fields.
    fn lowest_risk_path(&self) -> usize {
        let mut risk_level = vec![vec![0; self.risk_level[0].len()]; self.risk_level.len()];

        for i in 0..self.risk_level.len() {
            for j in 0..self.risk_level[0].len() {
                if i > 0 && j > 0 {
                    risk_level[i][j] =
                        self.risk_level[i][j] + risk_level[i - 1][j].min(risk_level[i][j - 1]);
                } else if i == 0 && j > 0 {
                    risk_level[i][j] = self.risk_level[i][j] + risk_level[i][j - 1];
                } else if i > 0 && j == 0 {
                    risk_level[i][j] = self.risk_level[i][j] + risk_level[i - 1][j];
                } else {
                    risk_level[i][j] = self.risk_level[i][j];
                }
            }
        }
        risk_level[self.risk_level.len() - 1][self.risk_level[0].len() - 1] - self.risk_level[0][0]
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.risk_level {
            for c in row {
                write!(f, "{: <3}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    const TEST_INPUT_FILE: &str = "input/day15_test.txt";
    const INPUT_FILE: &str = "input/day15.txt";

    #[test]
    fn test_cavern_from_str() {
        let input = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let cavern = Cavern::from_str(&input).unwrap();

        assert_eq!(cavern.risk_level.len(), 10);
        for row in cavern.risk_level.iter() {
            assert_eq!(row.len(), 10);
        }
    }

    #[test]
    fn test_cavern_lowest_risk_path() {
        let input = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let cavern = Cavern::from_str(&input).unwrap();
        assert_eq!(cavern.lowest_risk_path(), 40);
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let cavern = Cavern::from_str(&input).unwrap();
        println!("Part 1: {}", cavern.lowest_risk_path())
    }
}
