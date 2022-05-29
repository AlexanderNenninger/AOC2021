#![allow(dead_code)]
use std::{
    fmt::{self, Display},
    iter::repeat,
    str::FromStr,
};

/// Runlength encoded character
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RlChar {
    c: char,
    n: usize,
}

impl Display for RlChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = repeat(self.c).take(self.n).collect::<String>();
        write!(f, "{}", out)
    }
}

/// Runlength encoded String
#[derive(Debug, PartialEq, Eq)]
struct RlString(Vec<RlChar>);

impl RlString {
    fn push(&mut self, c: char, n: usize) {
        if self.0.is_empty() {
            self.0.push(RlChar { c, n });
        } else {
            let last = self.0.last_mut().unwrap();
            if last.c == c {
                last.n += n;
            } else {
                self.0.push(RlChar { c, n });
            }
        }
    }

    /// Inserts a new character between repetitons of at.
    fn insert_at_char(&mut self, c: char, at: char) {
        if c == at {
            for rlc in self.0.iter_mut() {
                if rlc.c == c {
                    // assumpion: n > 0
                    rlc.n += rlc.n - 1;
                }
            }
        } else {
            let mut i = 0;
            while i < self.0.len() {
                if self.0[i].c == at && self.0[i].n > 1 {
                    let n = self.0[i].n;
                    let seq = &[RlChar { c: at, n: 1 }, RlChar { c, n: 1 }];
                    let replacement = repeat(seq).take(n - 1).flatten();
                    self.0.splice(i..i, replacement.copied());

                    self.0[i + 2 * (n - 1)].n = 1;
                    i += 2 * (n - 1) + 1;
                } else {
                    i += 1;
                }
            }
        }
    }

    /// Inserts a new character between between.0 and between.1.
    fn insert_between(&mut self, c: char, between: (char, char)) {
        if between.0 == between.1 {
            self.insert_at_char(c, between.0);
        } else {
            if self.0.len() < 2 {
                return;
            }
            let mut i = 1;
            while i < self.0.len() {
                if self.0[i - 1].c == between.0 && self.0[i].c == between.1 {
                    if self.0[i - 1].c == c {
                        self.0[i - 1].n += 1;
                        i += 1;
                    } else if self.0[i].c == c {
                        self.0[i].n += 1;
                        i += 1;
                    } else {
                        self.0.insert(i, RlChar { c, n: 1 });
                        i += 2;
                    }
                } else {
                    i += 1;
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.0.iter().map(|c| c.n).sum()
    }
}

impl FromStr for RlString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 0 {
            return Ok(RlString(Vec::new()));
        } else {
            let mut chars = Vec::with_capacity(s.len());
            let mut prev = RlChar {
                c: s.chars().next().unwrap(),
                n: 0,
            };
            for c in s.chars() {
                if c == prev.c {
                    prev.n += 1;
                } else {
                    chars.push(prev);
                    prev = RlChar { c, n: 1 };
                }
            }
            chars.push(prev);
            Ok(RlString(chars))
        }
    }
}

impl Display for RlString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.0.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_string() {
        let s = RlString::from_str("").unwrap();
        assert_eq!(s.0.len(), 0);
    }

    #[test]
    fn test_with_repeated_char() {
        let s = RlString::from_str("aaaaa").unwrap();
        assert_eq!(s.0.len(), 1);
        assert_eq!(s.0[0].c, 'a');
        assert_eq!(s.0[0].n, 5);
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_with_mixed_chars() {
        let s = RlString::from_str("abbaaa").unwrap();
        assert_eq!(s.0.len(), 3);
        assert_eq!(s.0[0].c, 'a');
        assert_eq!(s.0[0].n, 1);
        assert_eq!(s.0[1].c, 'b');
        assert_eq!(s.0[1].n, 2);
        assert_eq!(s.0[2].c, 'a');
        assert_eq!(s.0[2].n, 3);
        assert_eq!(s.len(), 6);
    }

    #[test]
    fn test_insert_at_char_same() {
        let mut s = RlString::from_str("abbaaa").unwrap();
        s.insert_at_char('b', 'b');
        assert_eq!(s.0.len(), 3);
        assert_eq!(s.to_string(), "abbbaaa");
    }

    #[test]
    fn test_insert_at_char_different() {
        let mut s = RlString::from_str("abbaaa").unwrap();
        s.insert_at_char('c', 'b');
        println!("s: {}", s);
        assert_eq!(s.0.len(), 5);
        assert_eq!(s.to_string(), "abcbaaa", "{}", s);
    }

    #[test]
    fn test_insert_at_char_different_end() {
        let mut s = RlString::from_str("abbb").unwrap();
        s.insert_at_char('c', 'b');
        println!("{}", s);
        assert_eq!(s.0.len(), 6);
        assert_eq!(s.to_string(), "abcbcb", "{}", s);
    }

    #[test]
    fn test_insert_at_char_beginning() {
        let mut s = RlString::from_str("aa").unwrap();
        s.insert_at_char('c', 'a');
        println!("{}", s);
        assert_eq!(s.0.len(), 3);
        assert_eq!(s.to_string(), "aca", "{}", s);
    }

    #[test]
    fn test_insert_inbetween_same() {
        let mut s = RlString::from_str("abbaaa").unwrap();
        s.insert_between('b', ('b', 'b'));
        assert_eq!(s.0.len(), 3);
        assert_eq!(s.to_string(), "abbbaaa");
    }

    #[test]
    fn test_insert_inbetween_different() {
        let mut s = RlString::from_str("abbaaa").unwrap();
        s.insert_between('c', ('a', 'b'));
        assert_eq!(s.0.len(), 4);
        assert_eq!(s.to_string(), "acbbaaa");
    }

    #[test]
    fn test_insert_inbetween_different_end() {
        let mut s = RlString::from_str("abba").unwrap();
        s.insert_between('c', ('b', 'a'));
        assert_eq!(s.0.len(), 4);
        assert_eq!(s.to_string(), "abbca");
    }

    #[test]
    fn test_insert_inbetween_beginning() {
        let mut s = RlString::from_str("aa").unwrap();
        s.insert_between('c', ('a', 'a'));
        assert_eq!(s.0.len(), 3);
        assert_eq!(s.to_string(), "aca");
    }

    #[test]
    fn test_insert_same() {
        let mut s = RlString::from_str("abbaaa").unwrap();
        s.insert_between('b', ('a', 'b'));
        assert_eq!(s.0.len(), 3);
        assert_eq!(s.to_string(), "abbbaaa");
    }
}
