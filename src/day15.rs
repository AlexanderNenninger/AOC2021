#![allow(dead_code)]
use std::{fmt, str::FromStr, usize};

type Index2D = (usize, usize);

fn all_true(arr: &Vec<Vec<bool>>) -> bool {
    arr.iter()
        .map(|row| row.iter().all(|elem| *elem))
        .all(|elem| elem)
}

fn mat_argmin<T: Ord>(arr: &Vec<Vec<T>>, mask: &Vec<Vec<bool>>) -> Option<Index2D> {
    if arr.len() == 0 {
        return None;
    }
    let mut min_val: Option<&T> = None;
    let mut idx: Option<Index2D> = None;
    for (i, row) in arr.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if (min_val == None || item < min_val.unwrap()) && !mask[i][j] {
                min_val = Some(item);
                idx = Some((i, j))
            };
        }
    }
    idx
}

fn tile_for_part_2(matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut tiled = vec![vec![0; 5 * n]; 5 * m];
    for k in 0..5 {
        for l in 0..5 {
            for i in 0..m {
                for j in 0..n {
                    tiled[i + m * k][j + n * l] = (matrix[i][j] + k + l - 1) % 9 + 1
                }
            }
        }
    }
    tiled
}

struct Cavern {
    risk_level: Vec<Vec<usize>>,
}

impl Cavern {
    /// Dijkstra's algorithm on a grid graph.
    fn distance_update(
        &self,
        u: Index2D,
        v: Index2D,
        distances: &mut Vec<Vec<usize>>,
        predecessors: &mut Vec<Vec<Option<Index2D>>>,
    ) {
        let alternative = distances[u.0][u.1] + self.risk_level[v.0][v.1];
        if alternative < distances[v.0][v.1] {
            distances[v.0][v.1] = alternative;
            predecessors[v.0][v.1] = Some(u);
        }
    }

    fn neighbors(&self, u: Index2D) -> Vec<Index2D> {
        let m = self.risk_level.len() - 1; // max row index
        let n = self.risk_level[0].len() - 1; // max col index

        let mut neighbors = vec![];
        if u.0 > 0 {
            neighbors.push((u.0 - 1, u.1))
        }
        if u.1 > 0 {
            neighbors.push((u.0, u.1 - 1))
        }
        if u.0 < m {
            neighbors.push((u.0 + 1, u.1))
        }
        if u.1 < n {
            neighbors.push((u.0, u.1 + 1))
        }
        neighbors
    }

    fn lowest_risk_path(&self) -> Option<usize> {
        // Size of risk levels
        let n = self.risk_level.len();
        let m = self.risk_level[0].len();
        let s = n * m;

        // Initialize array of Predecessors.
        let mut predecessors: Vec<Vec<Option<Index2D>>> = vec![vec![None; n]; m];

        // Initialize array of distances.
        let mut distance: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; m];

        // Starting position
        distance[0][0] = 0;

        // Initialize array of already visited nodes.
        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

        for _ in 0..s {
            let u = mat_argmin(&distance, &visited)?;
            visited[u.0][u.1] = true;
            for v in self.neighbors(u) {
                if !visited[v.0][v.1] {
                    self.distance_update(u, v, &mut distance, &mut predecessors)
                }
            }
        }

        Some(distance[distance.len() - 1][distance[0].len() - 1])
    }
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
        assert_eq!(cavern.lowest_risk_path(), Some(40));
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let cavern = Cavern::from_str(&input).unwrap();
        println!("Part 1: {}", cavern.lowest_risk_path().unwrap())
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let cavern = Cavern::from_str(&input).unwrap();
        let cavern = Cavern {
            risk_level: tile_for_part_2(&cavern.risk_level),
        };
        assert_eq!(cavern.lowest_risk_path(), Some(315));
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let cavern = Cavern::from_str(&input).unwrap();
        let cavern = Cavern {
            risk_level: tile_for_part_2(&cavern.risk_level),
        };
        println!("Part 2: {}", cavern.lowest_risk_path().unwrap())
    }
}
