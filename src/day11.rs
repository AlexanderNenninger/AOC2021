#![allow(dead_code)]

use self::Direction::*;
use std::{error::Error, fmt::Display, str::FromStr};

macro_rules! skip_none {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => continue,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft]
            .iter()
            .copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid<const SIZE: usize> {
    data: [[u8; SIZE]; SIZE],
    flashed: [[bool; SIZE]; SIZE],
}

impl<const SIZE: usize> Grid<SIZE> {
    fn new(data: [[u8; SIZE]; SIZE]) -> Self {
        let flashed = [[false; SIZE]; SIZE];
        Self { data, flashed }
    }

    fn increase(&mut self) {
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                *cell += 1;
            }
        }
    }

    fn get_neighbor(&mut self, i: usize, j: usize, direction: Direction) -> Option<(usize, usize)> {
        let i = i as isize;
        let j = j as isize;
        let (i, j) = match direction {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
            Direction::UpLeft => (i - 1, j - 1),
            Direction::UpRight => (i - 1, j + 1),
            Direction::DownLeft => (i + 1, j - 1),
            Direction::DownRight => (i + 1, j + 1),
        };
        if i < 0 || j < 0 || i >= SIZE as isize || j >= SIZE as isize {
            return None;
        }
        Some((i as usize, j as usize))
    }

    fn flash_one(&mut self, i: usize, j: usize) {
        let flashed = &mut self.flashed[i][j];
        if self.data[i][j] > 9 && !*flashed {
            *flashed = true;
            for direction in Direction::iterator() {
                let (neighbor_i, neighbor_j) = skip_none!(self.get_neighbor(i, j, direction));
                self.data[neighbor_i][neighbor_j] += 1;
                self.flash_one(neighbor_i, neighbor_j);
            }
        }
    }

    fn flash(&mut self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.flash_one(i, j);
            }
        }
    }

    fn reset(&mut self) -> usize {
        let mut count = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                let cell = &mut self.data[i][j];
                if *cell > 9 {
                    count += 1;
                    *cell = 0;
                }

                self.flashed[i][j] = false;
            }
        }
        count
    }

    fn step(&mut self) -> usize {
        self.increase();
        self.flash();
        self.reset()
    }

    fn all_equal(&self) -> bool {
        for row in self.data.iter() {
            for cell in row.iter() {
                if *cell != self.data[0][0] {
                    return false;
                }
            }
        }
        true
    }
}

impl<const SIZE: usize> FromStr for Grid<SIZE> {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [[0; SIZE]; SIZE];
        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let entry = data
                    .get_mut(i)
                    .ok_or("Row out of bounds".to_string())?
                    .get_mut(j)
                    .ok_or("Coumn out of bounds".to_string())?;
                *entry = c.to_digit(10).ok_or("Non-digit character.".to_string())? as u8;
            }
        }
        Ok(Self::new(data))
    }
}

impl<const SIZE: usize> Display for Grid<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.data.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_1() -> usize {
    const INPUT_FILE: &str = "input/day11.txt";
    let data = std::fs::read_to_string(INPUT_FILE).expect("Failed to read input file.");
    let mut grid: Grid<10> = Grid::from_str(&data).expect("Failed to parse input file.");
    let mut count = 0;
    for _ in 0..100 {
        count += grid.step();
    }
    count
}

pub fn part_2() -> usize {
    const INPUT_FILE: &str = "input/day11.txt";
    let data = std::fs::read_to_string(INPUT_FILE).expect("Failed to read input file.");
    let mut grid: Grid<10> = Grid::from_str(&data).expect("Failed to parse input file.");
    let mut i = 0;
    loop {
        grid.step();
        i += 1;
        if grid.all_equal() {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write};

    use super::*;

    const TEST_INPUT_FILE: &str = "input/day11_test.txt";
    const TEST_GRID: Grid<5> = Grid {
        data: [
            [1, 1, 1, 1, 1],
            [1, 9, 9, 9, 1],
            [1, 9, 1, 9, 1],
            [1, 9, 9, 9, 1],
            [1, 1, 1, 1, 1],
        ],
        flashed: [[false; 5]; 5],
    };

    #[test]
    fn test_from_str() {
        let data = "11111\n19991\n19191\n19991\n11111";
        let grid: Grid<5> = Grid::from_str(data).unwrap();
        assert_eq!(grid.data, TEST_GRID.data);
    }

    #[test]
    fn test_flash() {
        let mut grid = TEST_GRID.clone();
        grid.increase();
        grid.flash();
        grid.reset();
        let expected = Grid {
            data: [
                [3, 4, 5, 4, 3],
                [4, 0, 0, 0, 4],
                [5, 0, 0, 0, 5],
                [4, 0, 0, 0, 4],
                [3, 4, 5, 4, 3],
            ],
            flashed: [[false; 5]; 5],
        };
        assert_eq!(grid.data, expected.data);
    }

    #[test]
    fn test_large_grid() {
        let data = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let mut grid: Grid<10> = Grid::from_str(&data).unwrap();

        let f_out = fs::File::create("output/day11_test.txt").unwrap();
        let mut f_out = std::io::BufWriter::new(f_out);

        writeln!(f_out, "Before any steps:\n{}", grid).unwrap();

        let mut count = 0;
        for i in 1..=100 {
            count += grid.step();
            writeln!(f_out, "After step {}:\n{}", i, grid).unwrap();
        }
        assert_eq!(count, 1656);
    }
}
