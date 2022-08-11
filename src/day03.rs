#![allow(unused)]
use std::error::Error;
use std::str::FromStr;

fn to_uint(bits: Vec<u8>) -> usize {
    bits.iter().fold(0, |acc, b| acc * 2 + *b as usize)
}

fn most_common_value(bits: &Vec<&Vec<u8>>, on_tie: u8) -> Vec<u8> {
    let mut counts = vec![0; bits[0].len()];
    for row in bits.iter() {
        for (i, &b) in row.iter().enumerate() {
            counts[i] += b as usize;
        }
    }
    if on_tie != 0 {
        return counts
            .iter()
            .map(|&c| (2 * c >= bits.len()) as u8) // c / n >= 1/2
            .collect();
    } else {
        return counts
            .iter()
            .map(|&c| (2 * c > bits.len()) as u8) // c / n > 1/2
            .collect();
    }
}

fn least_common_value(bits: &Vec<&Vec<u8>>, on_tie: u8) -> Vec<u8> {
    let most_common_values = most_common_value(bits, on_tie);
    most_common_values.into_iter().map(|b| 1 - b).collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Problem {
    data: Vec<Vec<u8>>,
}

impl Problem {
    fn new(data: Vec<Vec<u8>>) -> Self {
        Self { data }
    }

    /// Count the most common bit in each column.
    fn most_common_value(&self, on_tie: u8) -> Vec<u8> {
        let mut counts = vec![0; self.data[0].len()];
        for row in self.data.iter() {
            for (i, &b) in row.iter().enumerate() {
                counts[i] += b as usize;
            }
        }
        if on_tie != 0 {
            return counts
                .iter()
                .map(|&c| (2 * c >= self.data.len()) as u8) // c / n >= 1/2
                .collect();
        } else {
            return counts
                .iter()
                .map(|&c| (2 * c > self.data.len()) as u8) // c / n > 1/2
                .collect();
        }
    }

    /// Count the least common bit in each column.
    fn least_common_bit(&self) -> Vec<u8> {
        let most_common = self.most_common_value(0);
        most_common.iter().map(|&b| 1 - b).collect()
    }

    fn gamma_rate(&self) -> usize {
        to_uint(self.most_common_value(0))
    }

    fn epsilon_rate(&self) -> usize {
        to_uint(self.least_common_bit())
    }

    fn solve_part_1(&self) -> usize {
        self.gamma_rate() * self.epsilon_rate()
    }

    fn oxygen_generator_rating(&self) -> usize {
        let mut filtered: Vec<&Vec<u8>> = self.data.iter().collect();
        for j in 0..self.data[0].len() {
            let criterion = most_common_value(&filtered, 1)[j];
            filtered = filtered
                .iter()
                .filter(|&row| row[j] == criterion)
                .map(|&row| row)
                .collect();
            if filtered.len() == 1 {
                return to_uint(filtered[0].clone());
            }
        }
        unreachable!("Programmer error: no solution found.")
    }

    fn co2_scrubber_rating(&self) -> usize {
        let mut filtered: Vec<&Vec<u8>> = self.data.iter().collect();
        for j in 0..self.data[0].len() {
            let criterion = least_common_value(&filtered, 1)[j];
            filtered = filtered
                .iter()
                .filter(|&row| row[j] == criterion)
                .map(|&row| row)
                .collect();
            if filtered.len() == 1 {
                return to_uint(filtered[0].clone());
            }
        }
        unreachable!("Programmer error: no solution found.")
    }

    fn solve_part_2(&self) -> usize {
        self.oxygen_generator_rating() * self.co2_scrubber_rating()
    }
}

impl FromStr for Problem {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '0' => Ok(0),
                        '1' => Ok(1),
                        _ => Err(format!("Invalid character: {}", c)),
                    })
                    .collect::<Result<Vec<u8>, String>>()
            })
            .collect::<Result<Vec<Vec<u8>>, String>>()?;
        Ok(Problem { data })
    }
}

pub fn part_1() -> usize {
    let input = include_str!("../input/day03.txt");
    let problem = Problem::from_str(input).unwrap();
    problem.solve_part_1()
}

pub fn part_2() -> usize {
    let input = include_str!("../input/day03.txt");
    let problem = Problem::from_str(input).unwrap();
    problem.solve_part_2()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    const TEST_INPUT_FILE: &str = "input/day03_test.txt";

    #[test]
    fn test_from_str() {
        let data = "01\n10";
        let problem = Problem::from_str(data).unwrap();
        assert_eq!(problem.data, vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn test_most_common_bit() {
        let data = read_to_string(TEST_INPUT_FILE).unwrap();
        let problem = Problem::from_str(&data).unwrap();
        assert_eq!(problem.most_common_value(0), vec![1, 0, 1, 1, 0]);
    }

    #[test]
    fn test_to_uint() {
        assert_eq!(to_uint(vec![0, 0, 0]), 0);
        assert_eq!(to_uint(vec![0, 0, 1]), 1);
        assert_eq!(to_uint(vec![0, 1, 0]), 2);
        assert_eq!(to_uint(vec![0, 1, 1]), 3);
        assert_eq!(to_uint(vec![1, 0, 0]), 4);
        assert_eq!(to_uint(vec![1, 0, 1]), 5);
        assert_eq!(to_uint(vec![1, 1, 0]), 6);
        assert_eq!(to_uint(vec![1, 1, 1]), 7);
        assert_eq!(to_uint(vec![1, 0, 1, 1, 0]), 22);
    }

    #[test]
    fn test_gamma_rate() {
        let data = read_to_string(TEST_INPUT_FILE).unwrap();
        let problem = Problem::from_str(&data).unwrap();
        assert_eq!(problem.gamma_rate(), 22);
    }

    #[test]
    fn test_epsilon_rate() {
        let data = read_to_string(TEST_INPUT_FILE).unwrap();
        let problem = Problem::from_str(&data).unwrap();
        assert_eq!(problem.epsilon_rate(), 9);
    }

    #[test]
    fn test_solve_part_1() {
        let data = read_to_string(TEST_INPUT_FILE).unwrap();
        let problem = Problem::from_str(&data).unwrap();
        assert_eq!(problem.solve_part_1(), 22 * 9);
    }

    #[test]
    fn test_oxygen_generator_rating() {
        let data = read_to_string(TEST_INPUT_FILE).unwrap();
        let problem = Problem::from_str(&data).unwrap();
        assert_eq!(problem.oxygen_generator_rating(), 23);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let data = read_to_string(TEST_INPUT_FILE).unwrap();
        let problem = Problem::from_str(&data).unwrap();
        assert_eq!(problem.co2_scrubber_rating(), 10);
    }

    #[test]
    fn test_solve_part_2() {
        let data = read_to_string(TEST_INPUT_FILE).unwrap();
        let problem = Problem::from_str(&data).unwrap();
        assert_eq!(problem.solve_part_2(), 23 * 10);
    }
}
