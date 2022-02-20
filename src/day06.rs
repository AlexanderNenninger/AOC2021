use super::ReadStr;
use regex::Regex;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct State([usize; 9]);

impl State {
    fn step(self) -> Self {
        let mut new_state = State([0; 9]);

        new_state.0[0] = self.0[1];
        new_state.0[1] = self.0[2];
        new_state.0[2] = self.0[3];
        new_state.0[3] = self.0[4];
        new_state.0[4] = self.0[5];
        new_state.0[5] = self.0[6];
        new_state.0[6] = self.0[7] + self.0[0];
        new_state.0[7] = self.0[8];
        new_state.0[8] = self.0[0];

        new_state
    }

    fn sum(&self) -> usize {
        self.0.iter().sum()
    }
}

impl ReadStr for State {
    type Err = ();
    fn read_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\d+").unwrap();
        let mut state = State([0; 9]);
        for cap in re.captures_iter(s) {
            let idx = match usize::from_str_radix(&cap[0], 10) {
                Ok(idx) => idx,
                Err(_) => return Err(()),
            };
            state.0[idx] += 1;
        }
        return Ok(state);
    }
}

#[cfg(test)]
mod test {

    const INPUT_TEST: &str = "input/day06_test.txt";
    const INPUT: &str = "input/day06.txt";

    fn load_test_data() -> String {
        fs::read_to_string(INPUT_TEST).unwrap()
    }

    fn load_data() -> String {
        fs::read_to_string(INPUT).unwrap()
    }

    use super::*;
    #[test]
    fn test_read() {
        let data = load_test_data();
        let s = State::read_str(&data).unwrap();

        print!("{:?}\n", s);
    }
    #[test]
    fn test_part_1() {
        let data = load_test_data();
        let mut s = State::read_str(&data).unwrap();

        for _ in 0..80 {
            s = s.step();
        }

        print!("{}, {:?}\n", s.sum(), s);
    }

    #[test]
    fn part_1() {
        let data = load_data();
        let mut s = State::read_str(&data).unwrap();

        for _ in 0..80 {
            s = s.step();
        }

        print!("{}, {:?}\n", s.sum(), s);
    }

    #[test]
    fn part_2() {
        let data = load_data();
        let mut s = State::read_str(&data).unwrap();

        for _ in 0..256 {
            s = s.step();
        }

        print!("{}, {:?}\n", s.sum(), s);
    }
}
