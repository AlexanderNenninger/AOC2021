#![allow(dead_code, unused)]
use std::boxed::Box;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair {
    first: char,
    second: char,
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}{}", self.first, self.second)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rules {
    rules: HashMap<Pair, char>,
}

impl FromStr for Rules {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        for line in s.lines() {
            let mut parts = line.split(" -> ");
            let first = parts.next().ok_or(())?;
            let second = parts.next().ok_or(())?;
            let first = first.chars().collect::<Vec<_>>();
            let second = second.chars().collect::<Vec<_>>();
            let first = Pair {
                first: *first.get(0).ok_or(())?,
                second: *first.get(1).ok_or(())?,
            };
            let second = *second.get(0).ok_or(())?;
            rules.insert(first, second);
        }
        Ok(Rules { rules })
    }
}

impl Display for Rules {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut rules = self.rules.iter();
        if let Some((first, second)) = rules.next() {
            write!(f, "{} -> {}", first, second)?;
            for (first, second) in rules {
                write!(f, ", {} -> {}", first, second)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Polymer {
    pairs: HashMap<Pair, usize>,
    elements: HashMap<char, usize>,
}

impl FromStr for Polymer {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = HashMap::new();
        for c in s.chars().zip(s.chars().skip(1)) {
            let pair = Pair {
                first: c.0,
                second: c.1,
            };
            *pairs.entry(pair).or_insert(0) += 1;
        }

        let mut elements = HashMap::new();
        for c in s.chars() {
            *elements.entry(c).or_insert(0) += 1;
        }

        Ok(Polymer { pairs, elements })
    }
}

impl Polymer {
    fn polymerize(&self, rules: &HashMap<Pair, char>) -> Self {
        let mut polymer = self.clone();

        for (pair, count) in self.pairs.iter() {
            if let Some(new_element) = rules.get(pair) {
                polymer.pairs.entry(*pair).and_modify(|c| *c -= count);

                if polymer.pairs.get(pair).unwrap_or(&0) == &0 {
                    polymer.pairs.remove(pair);
                }

                let new_pair = Pair {
                    first: *new_element,
                    second: pair.second,
                };
                *polymer.pairs.entry(new_pair).or_insert(0) += count;
                let new_pair = Pair {
                    first: pair.first,
                    second: *new_element,
                };
                *polymer.pairs.entry(new_pair).or_insert(0) += count;

                *polymer.elements.entry(*new_element).or_insert(0) += count;
            }
        }

        polymer
    }

    fn solution(&self) -> Option<usize> {
        Some(self.elements.values().max()? - self.elements.values().min()?)
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Ok(for (key, val) in self.pairs.iter() {
            if *val != 0 {
                write!(f, "{} -> {}, ", key, val)?;
            }
        })
    }
}

#[cfg(test)]
mod test_rules {
    use super::*;

    const TEST_INPUT: &str = "./input/day14_test.txt";
    #[test]
    fn test_rules_from_str() {
        let data = read_to_string(TEST_INPUT).expect("Failed to read input");

        let parts = data.split("\n\n");
        let rules: Rules = parts
            .skip(1)
            .next()
            .expect("Failed to get rules")
            .parse()
            .expect("Could not parse Rules.");

        assert_eq!(rules.rules.len(), 16);
        assert_eq!(
            rules.rules.get(&Pair {
                first: 'C',
                second: 'H'
            }),
            Some(&'B')
        );
        assert_eq!(
            rules.rules.get(&Pair {
                first: 'C',
                second: 'N'
            }),
            Some(&'C')
        );
    }
}

fn part_1() {
    let data = read_to_string("./input/day14.txt").expect("Failed to read input");

    let mut parts = data.split("\n\n");
    let mut polymer: Polymer = parts
        .next()
        .expect("Failed to get polymer")
        .parse()
        .expect("Could not parse Polymer.");

    let rules: Rules = parts
        .next()
        .expect("Failed to get rules")
        .parse()
        .expect("Could not parse Rules.");

    for _ in 0..10 {
        polymer = polymer.polymerize(&rules.rules);
    }
    println!("{}", polymer);
    println!("{}", polymer.solution().unwrap());
}

fn part_2() {
    let data = read_to_string("./input/day14.txt").expect("Failed to read input");

    let mut parts = data.split("\n\n");
    let mut polymer: Polymer = parts
        .next()
        .expect("Failed to get polymer")
        .parse()
        .expect("Could not parse Polymer.");

    let rules: Rules = parts
        .next()
        .expect("Failed to get rules")
        .parse()
        .expect("Could not parse Rules.");

    for _ in 0..40 {
        polymer = polymer.polymerize(&rules.rules);
    }
    println!("{}", polymer);
    println!("{}", polymer.solution().unwrap());
}

#[cfg(test)]
mod test_polymer {
    use super::*;

    const TEST_INPUT: &str = "./input/day14_test.txt";

    #[test]
    fn test_polymer_from_str() {
        let data = read_to_string(TEST_INPUT).expect("Failed to read input");

        let mut parts = data.split("\n\n");
        let polymer: Polymer = parts
            .next()
            .expect("Failed to get polymer.")
            .parse()
            .expect("Could not parse Polymer.");

        assert_eq!(polymer.pairs.len(), 3);

        assert_eq!(
            polymer.pairs.get(&Pair {
                first: 'N',
                second: 'C'
            }),
            Some(&1)
        );

        assert_eq!(
            polymer.pairs.get(&Pair {
                first: 'N',
                second: 'N'
            }),
            Some(&1)
        );

        assert_eq!(
            polymer.pairs.get(&Pair {
                first: 'C',
                second: 'B'
            }),
            Some(&1)
        );
    }

    #[test]
    fn test_polymerize_small() {
        let data = read_to_string(TEST_INPUT).expect("Failed to read input");

        let mut parts = data.split("\n\n");
        let mut polymer: Polymer = parts
            .next()
            .expect("Failed to get polymer.")
            .parse()
            .expect("Could not parse Polymer.");

        println!("Polymer: {}", polymer);

        let rules: Rules = parts
            .next()
            .expect("Failed to get rules.")
            .parse()
            .expect("Could not parse Rules.");

        println!("Rules: {}", rules);

        // Step 1
        polymer = polymer.polymerize(&rules.rules);
        let true_polymer = "NCNBCHB"
            .parse::<Polymer>()
            .expect("Could not parse Polymer.");

        println!("Polymer:      {}", polymer);
        println!("True Polymer: {}", true_polymer);

        assert_eq!(polymer, true_polymer);

        // Step 2
        polymer = polymer.polymerize(&rules.rules);
        let true_polymer = "NBCCNBBBCBHCB"
            .parse::<Polymer>()
            .expect("Could not parse Polymer.");

        println!("Polymer:      {}", polymer);
        println!("True Polymer: {}", true_polymer);

        assert_eq!(polymer, true_polymer);
    }

    #[test]
    fn test_polymerize_part_1() {
        let data = read_to_string(TEST_INPUT).expect("Failed to read input");

        let mut parts = data.split("\n\n");
        let mut polymer: Polymer = parts
            .next()
            .expect("Failed to get polymer.")
            .parse()
            .expect("Could not parse Polymer.");

        println!("Polymer: {}", polymer);

        let rules: Rules = parts
            .next()
            .expect("Failed to get rules.")
            .parse()
            .expect("Could not parse Rules.");

        println!("Rules: {}", rules);

        for _ in 0..10 {
            polymer = polymer.polymerize(&rules.rules);
        }

        assert_eq!(polymer.solution(), Some(1588));
    }

    #[test]
    fn test_polymerize_part_2() {
        let data = read_to_string(TEST_INPUT).expect("Failed to read input");

        let mut parts = data.split("\n\n");
        let mut polymer: Polymer = parts
            .next()
            .expect("Failed to get polymer.")
            .parse()
            .expect("Could not parse Polymer.");

        println!("Polymer: {}", polymer);

        let rules: Rules = parts
            .next()
            .expect("Failed to get rules.")
            .parse()
            .expect("Could not parse Rules.");

        println!("Rules: {}", rules);

        for _ in 0..40 {
            polymer = polymer.polymerize(&rules.rules);
        }

        assert_eq!(polymer.solution(), Some(2188189693529));
    }

    #[test]
    fn test_part_1() {
        part_1()
    }

    #[test]
    fn test_part_2() {
        part_2()
    }
}
