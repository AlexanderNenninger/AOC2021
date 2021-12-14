use std::collections::HashMap;
use std::str;

pub const INPUT: &str = "input/day14.txt";

type Polymer = Vec<char>;
type Insertions = HashMap<(char, char), char>;

trait Polymerize {
    fn polymerize(self, insertions: &Insertions) -> Self;

    /// Insert a character 'base' at idx.
    fn insert(self, idx: usize, base: char) -> Self;
}

impl Polymerize for Polymer {
    fn polymerize(mut self, insertions: &Insertions) -> Self {
        // No pairs in a 0- or 1-element vector
        if self.len() < 2 {
            return self;
        }

        // Empty Data
        if insertions.len() == 0 {
            return self;
        }

        let mut i = 0;
        loop {
            let fst = match self.get(i) {
                Some(val) => val,
                None => unreachable!(),
            };

            match self.get(i + 1) {
                Some(snd) => match insertions.get(&(*fst, *snd)) {
                    Some(base) => {
                        i += 1;
                        self = self.insert(i, *base);
                    }
                    None => {
                        i += 1;
                        continue;
                    }
                },
                None => return self,
            };
            i += 1;
        }
    }

    fn insert(self, idx: usize, base: char) -> Self {
        self[0..idx]
            .iter()
            .chain(vec![&base])
            .chain(&self[idx..])
            .map(|x| *x)
            .collect()
    }
}

trait ReadStr: Sized {
    type Err;
    fn read_str(s: &str) -> Result<Self, Self::Err>;
}

impl ReadStr for Polymer {
    type Err = ();
    fn read_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        match chars.iter().all(|c: &char| c.is_ascii_uppercase()) {
            true => return Ok(chars),
            false => return Err(()),
        };
    }
}

impl ReadStr for Insertions {
    type Err = ();
    fn read_str(s: &str) -> Result<Self, Self::Err> {
        let mut insertions = Insertions::new();
        for line in s.lines() {
            if line.len() < 7 {
                return Err(());
            }
            let fst = line.chars().nth(0).unwrap();
            let snd = line.chars().nth(1).unwrap();
            let base = line.chars().nth(6).unwrap();
            if !fst.is_ascii_uppercase() & !snd.is_ascii_uppercase() & !base.is_ascii_uppercase() {
                return Err(());
            }

            insertions.insert((fst, snd), base);
        }
        Ok(insertions)
    }
}

#[derive(Debug, PartialEq)]
struct Program {
    polymer: Polymer,
    insertions: Insertions,
}

impl ReadStr for Program {
    type Err = ();
    fn read_str(s: &str) -> Result<Self, Self::Err> {
        let (poly_str, insertions_str) = s.split_once("\n\n").ok_or(())?;

        Ok(Program {
            polymer: Polymer::read_str(poly_str)?,
            insertions: Insertions::read_str(insertions_str)?,
        })
    }
}

impl Program {
    fn run(self) -> Self {
        Self {
            polymer: self.polymer.polymerize(&self.insertions),
            insertions: self.insertions,
        }
    }

    fn count_bases(&self) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for base in self.polymer.iter() {
            match counts.get_mut(base) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    counts.insert(*base, 1);
                }
            };
        }
        counts
    }
}

pub fn solve_1(data: &str) -> usize {
    let mut program = Program::read_str(data).unwrap();
    for _ in 0..10 {
        program = program.run();
    }
    let counts = program.count_bases();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn solve_2(data: &str) -> usize {
    let mut program = Program::read_str(data).unwrap();
    for _ in 0..40 {
        program = program.run();
    }
    let counts = program.count_bases();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub mod tests {
    #[allow(unused)]
    pub const INPUT_TEST: &str = "input/day14_test.txt";

    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use std::fs;

    #[test]
    fn insert() {
        let mut input = vec!['a', 'b', 'c'];
        input = input.insert(1, 'z');

        assert_eq!(input, vec!['a', 'z', 'b', 'c'])
    }

    #[test]
    fn polymerize() {
        let mut insertions: Insertions = HashMap::new();
        insertions.insert(('a', 'a'), 'a');

        let mut polymer: Polymer = "aabcd".chars().collect();
        polymer = polymer.polymerize(&insertions);

        assert_eq!(polymer, "aaabcd".chars().collect::<Vec<char>>());
    }

    #[test]
    fn polymer_read_str() {
        let s = "ABCX";
        let poly = Polymer::read_str(s);
        assert_eq!(poly, Ok(s.chars().collect::<Vec<char>>()));

        let s = "AB CX";
        let poly = Polymer::read_str(s);
        assert_eq!(poly, Err(()));
    }

    #[test]
    fn program_read_str() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let program = Program::read_str(&data).expect("Program could not be parsed");
    }

    #[test]
    fn program_count() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let program = Program::read_str(&data).expect("Program could not be parsed");

        let mut hs = HashMap::new();
        hs.insert('C', 1);
        hs.insert('N', 2);
        hs.insert('B', 1);
        assert_eq!(program.count_bases(), hs);
    }

    #[test]
    fn insertions_read_str() {
        let s = "CC -> N\nCN -> C";
        let insertions = Insertions::read_str(s);

        let mut hm = HashMap::new();
        hm.insert(('C', 'C'), 'N');
        hm.insert(('C', 'N'), 'C');

        assert_eq!(insertions, Ok(hm))
    }

    #[test]
    fn test_1() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let res = solve_1(&data);
        println!("{}", &res);
        assert_eq!(res, 1588)
    }

    #[test]
    fn test_2() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let res = solve_2(&data);
        println!("{}", &res);
        assert_eq!(res, 2188189693529);
    }
}
