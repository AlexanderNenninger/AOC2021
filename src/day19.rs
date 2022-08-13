#![allow(unused)]
use itertools::Itertools;
use num_traits::Pow;
use std::{
    collections::HashSet,
    iter::FromIterator,
    mem::size_of,
    ops::{Add, Rem, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point<const DIM: usize> {
    coords: [isize; DIM],
}

impl<const DIM: usize> Point<DIM> {
    fn new(coords: [isize; DIM]) -> Self {
        Self { coords }
    }
}

impl<const DIM: usize> Add for Point<DIM> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut coords = [0; DIM];
        for i in 0..DIM {
            coords[i] = self.coords[i] + other.coords[i];
        }
        Self { coords }
    }
}

impl<const DIM: usize> Sub for Point<DIM> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut coords = [0; DIM];
        for i in 0..DIM {
            coords[i] = self.coords[i] - other.coords[i];
        }
        Self { coords }
    }
}

impl<const DIM: usize> Pow<u32> for Point<DIM> {
    type Output = Self;
    fn pow(self, exp: u32) -> Self {
        let mut coords = self.coords.clone();
        for i in 0..DIM {
            coords[i] = coords[i].pow(exp);
        }
        Self::new(coords)
    }
}

// Used for generating consistently labled point clouds.
impl<const DIM: usize> PartialOrd for Point<DIM> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..DIM {
            if self.coords[i] != other.coords[i] {
                return self.coords[i].partial_cmp(&other.coords[i]);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl<const DIM: usize> Ord for Point<DIM> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PointCloud<const DIM: usize> {
    points: Vec<Point<DIM>>,
}

impl<const DIM: usize> PointCloud<DIM> {
    fn new(points: Vec<Point<DIM>>) -> Self {
        Self {
            points: points.into_iter().sorted().dedup().collect(),
        }
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn pairwise_distances(&self) -> Vec<f64> {
        self.iter()
            .cartesian_product(self)
            .map(|(p1, p2)| {
                (p1.pow(2) - p2.pow(2))
                    .coords
                    .iter()
                    .map(|&x| x as f64)
                    .sum::<f64>()
                    .sqrt()
            })
            .collect()
    }

    fn iter(&self) -> impl Iterator<Item = &Point<DIM>> {
        self.points.iter()
    }

    /// Matches if self.pairwise_differences() equals other.pairwise_differences().
    /// This type of equality is rotation and translationn invariant.
    fn matches(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.pairwise_distances() == other.pairwise_distances()
    }

    fn matching_points(&self, other: &Self) -> Option<PointCloud<DIM>> {
        let mut max_match = None;
        for k in 2..self.len() {
            let mut found_match = false;
            for s1 in self.subsets(k) {
                for s2 in other.subsets(k) {
                    if s1.matches(&s2) {
                        max_match = Some(s1.clone());
                        found_match = true;
                    }
                }
            }
            if !found_match {
                break;
            }
        }
        max_match
    }

    fn subsets<'a>(&'a self, k: usize) -> impl Iterator<Item = PointCloud<DIM>> + 'a {
        self.points
            .iter()
            .combinations(k)
            .map(|points| -> PointCloud<DIM> {
                let mut result = Self::new(points.into_iter().cloned().collect());
                result
            })
    }
}

impl<const DIM: usize> IntoIterator for PointCloud<DIM> {
    type Item = Point<DIM>;
    type IntoIter = std::vec::IntoIter<Point<DIM>>;
    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl<'a, const DIM: usize> IntoIterator for &'a PointCloud<DIM> {
    type Item = &'a Point<DIM>;
    type IntoIter = std::slice::Iter<'a, Point<DIM>>;
    fn into_iter(self) -> Self::IntoIter {
        self.points.iter()
    }
}

impl<const DIM: usize> FromIterator<Point<DIM>> for PointCloud<DIM> {
    fn from_iter<I: IntoIterator<Item = Point<DIM>>>(iter: I) -> Self {
        let mut result = Self::new(iter.into_iter().collect());
        result
    }
}

struct Scanner {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_cloud() {
        let mut cloud = PointCloud::new(vec![
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([0, 1]),
            Point::new([1, 1]),
            Point::new([2, 0]),
            Point::new([0, 2]),
            Point::new([2, 1]),
            Point::new([1, 2]),
            Point::new([2, 2]),
        ]);
        assert_eq!(cloud.len(), 9);

        let mut dists = cloud.pairwise_distances();
        assert_eq!(dists.len(), 9);

        for p in cloud.iter() {
            assert!(p.coords[0].abs() + p.coords[1].abs() <= 4);
        }
    }

    #[test]
    fn test_matches() {
        todo!()
    }
}
