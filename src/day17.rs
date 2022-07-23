#![allow(unused)]

use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
struct TargetArea {
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
}

impl TargetArea {
    fn new(xmin: i32, ymin: i32, xmax: i32, ymax: i32) -> Self {
        Self {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax
    }

    fn reachable(&self, x: i32, y: i32, vx: i32, vy: i32) -> bool {
        // If we are inside the target area, we can reach it (duh).
        if self.contains(x, y) {
            return true;
        }
        self.ymin <= y || vy >= 0 // Above target or moving up
    }
}

impl FromStr for TargetArea {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_X: Regex = Regex::new(r"x=(?P<xmin>-?\d+)\.\.(?P<xmax>-?\d+)").unwrap();
        };

        lazy_static! {
            static ref RE_Y: Regex = Regex::new(r"y=(?P<ymin>-?\d+)\.\.(?P<ymax>-?\d+)").unwrap();
        };

        let x_caps = RE_X.captures(s).ok_or("no x values")?;
        let xmin = x_caps
            .name("xmin")
            .ok_or("no xmin")?
            .as_str()
            .parse::<i32>()?;
        let xmax = x_caps
            .name("xmax")
            .ok_or("no xmax")?
            .as_str()
            .parse::<i32>()?;

        let y_caps = RE_Y.captures(s).ok_or("no y values")?;
        let ymin = y_caps
            .name("ymin")
            .ok_or("no ymin")?
            .as_str()
            .parse::<i32>()?;
        let ymax = y_caps
            .name("ymax")
            .ok_or("no ymax")?
            .as_str()
            .parse::<i32>()?;

        Ok(TargetArea {
            xmin,
            xmax,
            ymin,
            ymax,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct State {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl State {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Self { x, y, vx, vy }
    }

    fn step(self) -> Self {
        Self {
            x: self.x + self.vx,
            y: self.y + self.vy,
            vx: self.vx - self.vx.signum(),
            vy: self.vy - 1,
        }
    }
}

struct Problem {
    target_area: TargetArea,
    state: State,
}

impl Problem {
    /// Initialize Problem.
    fn new(target_area: TargetArea, starting_state: State) -> Self {
        Self {
            target_area,
            state: starting_state,
        }
    }

    fn step(&mut self) {
        self.state = self.state.step();
    }

    fn target_area_reachable(&self) -> bool {
        self.target_area
            .reachable(self.state.x, self.state.y, self.state.vx, self.state.vy)
    }

    fn target_area_contains(&self) -> bool {
        self.target_area.contains(self.state.x, self.state.y)
    }

    fn solve(&self) -> Result<Vec<State>, Vec<State>> {
        let mut state = self.state.clone();
        let mut history = vec![];
        history.push(state);

        loop {
            state = state.step();
            history.push(state);

            if self.target_area.contains(state.x, state.y) {
                return Ok(history);
            }

            if !self
                .target_area
                .reachable(state.x, state.y, state.vx, state.vy)
            {
                return Err(history);
            }
        }
    }

    // Calculate minimal and maximal horizontal velocity such that the velocity in reaches 0 in the target area.
    fn estimate_x_velocity(&self) -> (i32, i32) {
        let xmin_dist = (self.target_area.xmin - self.state.x) as f32;
        let xmax_dist = (self.target_area.xmax - self.state.x) as f32;

        let vx_min = (-1. + f32::sqrt(1. - 4.0 * (1.0 - 2.0 * xmin_dist))) / 2.0;
        let vx_max = (-1. + f32::sqrt(1. - 4.0 * (1.0 - 2.0 * xmax_dist))) / 2.0;

        (vx_min.ceil() as i32, vx_max.floor() as i32)
    }

    /// Calculate the maximal height, the probe can reach.
    /// Brute force solution via grid search.
    fn part_1(&mut self, vymax: i32) -> i32 {
        let (vxmin, vxmax) = self.estimate_x_velocity();

        let mut max_height = i32::MIN;
        for vx in vxmin..=vxmax {
            self.state.vx = vx;
            for vy in 0..=vymax {
                self.state.vy = vy;
                let peak = match self.solve() {
                    Ok(history) => history.iter().max_by_key(|&s| s.y).unwrap().y,
                    Err(history) => continue,
                };
                max_height = max_height.max(peak);
            }
        }
        max_height
    }

    /// Find all velocities that can reach the target area.
    /// Brute force solution via grid search.
    /// Returns the number of velocities that can reach the target area.
    fn part_2(&mut self, vxmin: i32, vxmax: i32, vymin: i32, vymax: i32) -> usize {
        let mut count = 0;
        for vx in vxmin..=vxmax {
            self.state.vx = vx;
            for vy in vymin..=vymax {
                self.state.vy = vy;
                if self.solve().is_ok() {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    const TEST_INPUT_FILE: &'static str = "input/day17_test.txt";
    const INPUT_FILE: &'static str = "input/day17.txt";

    #[test]
    fn test_parse_target_area() {
        let input = "x=-3..-1, y=1..3";
        let target_area = TargetArea::from_str(input).unwrap();
        assert_eq!(target_area.xmin, -3);
        assert_eq!(target_area.xmax, -1);
        assert_eq!(target_area.ymin, 1);
        assert_eq!(target_area.ymax, 3);
    }

    #[test]
    fn test_estimate_x_velocity() {
        let data = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let problem = Problem::new(target, State::new(0, 0, 0, 0));
        let (vx_min, vx_max) = problem.estimate_x_velocity();

        assert_eq!(vx_min, 6);
        assert_eq!(vx_max, 7);
    }

    #[test]
    fn test_reachable() {
        let data = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let problem = Problem::new(target, State::new(0, 0, 6, 3));
        assert!(problem.target_area_reachable());

        let problem = Problem::new(target, State::new(0, 0, 7, 2));
        assert!(problem.target_area_reachable());

        let problem = Problem::new(target, State::new(40, 0, 7, 3));
        assert!(!problem.target_area_reachable());
    }

    #[test]
    fn test_in_target_area() {
        let data = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let problem = Problem::new(target, State::new(25, -7, 6, 3));
        assert!(problem.target_area_contains());

        let problem = Problem::new(target, State::new(20, -10, 7, 2));
        assert!(problem.target_area_contains());

        let problem = Problem::new(target, State::new(40, 0, 6, 0));
        assert!(!problem.target_area_contains());
    }

    #[test]
    fn test_solve() {
        let data = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let problem = Problem::new(target, State::new(0, 0, 6, 9));
        let history = problem.solve().unwrap();
        assert_eq!(history.iter().max_by_key(|&s| s.y).unwrap().y, 45);
    }

    #[test]
    fn test_part_1() {
        let data = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let mut problem = Problem::new(target, State::new(0, 0, 0, 0));
        let max_height = problem.part_1(10);
        assert_eq!(max_height, 45);
    }

    #[test]
    fn part_1() {
        let data = fs::read_to_string(INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let mut problem = Problem::new(target, State::new(0, 0, 0, 0));
        let max_height = problem.part_1(1000);
        println!("Max Height: {}", max_height);
    }

    #[test]
    fn test_part_2() {
        let data = fs::read_to_string(TEST_INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let mut problem = Problem::new(target, State::new(0, 0, 0, 0));
        let count = problem.part_2(0, 100, -100, 100);
        assert_eq!(count, 112);
    }

    #[test]
    fn part_2() {
        let data = fs::read_to_string(INPUT_FILE).unwrap();
        let target = TargetArea::from_str(&data).unwrap();

        let mut problem = Problem::new(target, State::new(0, 0, 0, 0));
        let count = problem.part_2(0, 1000, -1000, 1000);
        println!("Count: {}", count);
    }
}
