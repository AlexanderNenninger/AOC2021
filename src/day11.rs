#![allow(dead_code)]
use std::{fmt::Display, str::FromStr};

struct Energies([[u32; 10]; 10]);

impl Energies {
    /// Assign add other to each element of self.
    fn add_u32(&mut self, other: u32) {
        for row in self.0.iter_mut() {
            for elem in row.iter_mut() {
                *elem += other;
            }
        }
    }

    /// Return a mask indicating which elements of self are greater than 9.
    fn flash(&self) -> [[bool; 10]; 10] {
        let mut mask = [[false; 10]; 10];
        for (i, row) in self.0.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if *elem > 9 {
                    mask[i][j] = true;
                }
            }
        }
        mask
    }

    /// Increase all elements, whose neighbor has flashed by 1.
    fn sense_flash(&mut self, mask: &[[bool; 10]; 10]) {
        for (i, row) in self.0.iter_mut().enumerate() {
            for (j, elem) in row.iter_mut().enumerate() {
                if mask[i][j] {
                    if i > 0 && mask[i - 1][j] {
                        *elem += 1;
                    }

                    if i < 9 && mask[i + 1][j] {
                        *elem += 1;
                    }

                    if j > 0 && mask[i][j - 1] {
                        *elem += 1;
                    }

                    if j < 9 && mask[i][j + 1] {
                        *elem += 1;
                    }

                    if i > 0 && j > 0 && mask[i - 1][j - 1] {
                        *elem += 1;
                    }

                    if i > 0 && j < 9 && mask[i - 1][j + 1] {
                        *elem += 1;
                    }

                    if i < 9 && j > 0 && mask[i + 1][j - 1] {
                        *elem += 1;
                    }

                    if i < 9 && j < 9 && mask[i + 1][j + 1] {
                        *elem += 1;
                    }
                }
            }
        }
    }

    fn step(&mut self) {
        self.add_u32(1);
        let mask = self.flash();
        self.sense_flash(&mask);
    }
}

impl FromStr for Energies {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octos = Energies([[0; 10]; 10]);
        let s = s.trim();

        for (y, line) in s.lines().enumerate() {
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                octos.0[y][x] = c.to_digit(10).ok_or(())?;
            }
        }
        Ok(octos)
    }
}

impl Display for Energies {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.iter() {
            for elem in row.iter() {
                write!(f, "{: >2}", elem)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const INPUT_TEST: &str = "input/day11_test.txt";

    #[test]
    fn test_from_str() {
        let input = "
            11111
            19991
            19191
            19991
            11111
        ";
        let energies = Energies::from_str(input).unwrap();
        assert_eq!(energies.0[0][0], 1);
        assert_eq!(energies.0[0][1], 1);
        assert_eq!(energies.0[0][2], 1);
        assert_eq!(energies.0[0][3], 1);
        assert_eq!(energies.0[0][4], 1);

        assert_eq!(energies.0[1][0], 1);
        assert_eq!(energies.0[1][1], 9);
        assert_eq!(energies.0[1][2], 9);
        assert_eq!(energies.0[1][3], 9);
        assert_eq!(energies.0[1][4], 1);
    }

    #[test]
    fn test_add_u32() {
        let mut energies = Energies([[0; 10]; 10]);
        energies.add_u32(1);
        assert_eq!(energies.0[0][0], 1);
        assert_eq!(energies.0[0][1], 1);
        assert_eq!(energies.0[0][2], 1);
        assert_eq!(energies.0[0][3], 1);
        assert_eq!(energies.0[0][4], 1);
    }

    #[test]
    fn test_flash() {
        let mut energies = Energies([[0; 10]; 10]);
        energies.0[0][0] = 1;
        energies.0[0][1] = 1;
        energies.0[0][2] = 1;
        energies.0[0][3] = 1;
        energies.0[0][4] = 1;

        energies.0[1][0] = 1;
        energies.0[1][1] = 10;
        energies.0[1][2] = 10;
        energies.0[1][3] = 10;
        energies.0[1][4] = 1;

        let mask = energies.flash();
        assert_eq!(mask[0][0], false);
        assert_eq!(mask[0][1], false);
        assert_eq!(mask[0][2], false);
        assert_eq!(mask[0][3], false);
        assert_eq!(mask[0][4], false);

        assert_eq!(mask[1][0], false);
        assert_eq!(mask[1][1], true);
        assert_eq!(mask[1][2], true);
        assert_eq!(mask[1][3], true);
        assert_eq!(mask[1][4], false);
    }

    #[test]
    fn test_step() {
        let input = read_to_string(INPUT_TEST).unwrap();
        let mut energies = Energies::from_str(&input).unwrap();

        println!("{}", energies);

        energies.add_u32(2);

        println!("{}", energies);

        let mask = energies.flash();
        energies.sense_flash(&mask);

        println!("{}", energies);
    }
}
