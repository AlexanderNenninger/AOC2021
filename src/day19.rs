#![allow(unused)]
use nalgebra::Point;

#[derive(Debug, PartialEq, Eq)]
struct I32Point<const D: usize>(Point<i32, D>);

/// Lexical Ordering of Points. Simplifies comparison of Scanners
impl<const D: usize> std::cmp::PartialOrd for I32Point<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..D {
            match self.0[i].cmp(&other.0[i]) {
                std::cmp::Ordering::Equal => continue,
                o => return Some(o),
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

/// Since partial_cmp cannot fail, we simply unwrap.
impl<const D: usize> std::cmp::Ord for I32Point<D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Scanner<const D: usize> {
    points: Vec<Point<i32, D>>,
    pos: I32Point<D>,
}
