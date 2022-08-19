#![allow(unused)]

use std::{cell::Ref, cmp::Ordering, error::Error, str::FromStr};

use itertools::Itertools;
use na::SVector;
extern crate nalgebra as na;

type Point<const D: usize> = na::Point<f64, D>;

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn cmp_points_lexical<const D: usize>(p: &Point<D>, q: &Point<D>) -> Ordering {
    for i in 0..D {
        // Equal if close
        if (p[i] - q[i]).abs() < 1e-6 {
            continue;
        }
        return p[i].partial_cmp(&q[i]).unwrap();
    }
    Ordering::Equal
}

fn reflection_matrices<const D: usize>() -> impl Iterator<Item = na::SMatrix<f64, D, D>> {
    (0..2u32.pow(D as u32)).map(|i| {
        let mut m = na::SMatrix::<f64, D, D>::identity();
        for j in 0..D {
            if i & (1 << j) != 0 {
                m[(j, j)] = -1.0;
            }
        }
        m
    })
}

fn permutation_matrices<const D: usize>() -> impl Iterator<Item = na::SMatrix<f64, D, D>> {
    let base = [0.; D];
    let mut working_set = Vec::with_capacity(D);
    for i in 0..D {
        working_set.push(base);
        working_set[i][i] = 1.;
    }
    working_set.into_iter().permutations(D).map(|p| {
        let mut m = na::SMatrix::<f64, D, D>::identity();
        for i in 0..D {
            for j in 0..D {
                m[(i, j)] = p[i][j];
            }
        }
        m
    })
}

fn points_almost_equal<const D: usize>(p: &Point<D>, q: &Point<D>) -> bool {
    for i in 0..D {
        if (p[i] - q[i]).abs() > 1e-6 {
            return false;
        }
    }
    true
}

fn tup_to_opt<T>(opt1: Option<T>, opt2: Option<T>) -> Option<(T, T)> {
    match (opt1, opt2) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
struct OPointCloud<const D: usize> {
    points: Vec<Point<D>>,
}

impl<const D: usize> OPointCloud<D> {
    fn new(points: Vec<Point<D>>) -> Self {
        Self { points }
    }

    fn from_ref_point_cloud<'a>(points: &'a RefPointCloud<D>) -> Self {
        Self {
            points: points.points.iter().map(|&p| p.clone()).collect(),
        }
    }

    fn almost_equal(&self, other: &Self) -> bool {
        self.points
            .iter()
            .all(|p| other.points.iter().any(|q| points_almost_equal(p, q)))
    }

    fn subsets(&self, size: usize) -> impl Iterator<Item = Vec<&Point<D>>> + '_ {
        self.points.iter().combinations(size)
    }

    fn pairs(&self) -> impl Iterator<Item = (&Point<D>, &Point<D>)> + '_ {
        self.subsets(2).map(|pair| (pair[0], pair[1]))
    }

    fn pairwise_distances(&'_ self) -> impl Iterator<Item = f64> + '_ {
        self.pairs().map(|pair| (pair.0 - pair.1).abs().sum())
    }

    /// Two point clouds are isometric iff they have the same number of points and the same
    /// pairwise distances. We need to use unordered collections to ignore permutations.
    fn matches(&self, other: &Self) -> bool {
        if !self.points.len() == other.points.len() {
            return false;
        }
        let d1 = self
            .pairwise_distances()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap());
        let d2 = other
            .pairwise_distances()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap());
        d1.zip_eq(d2).all(|(x, y)| x == y)
    }

    fn maximal_matching_subsets<'a>(
        &'a self,
        other: &'a Self,
    ) -> Option<(RefPointCloud<'_, D>, RefPointCloud<'_, D>)> {
        let ref this = &RefPointCloud::from_point_cloud(self);
        let other = &RefPointCloud::from_point_cloud(other);
        let mut max_size = 0;
        let mut max_subset_1 = None;
        let mut max_subset_2 = None;
        for k in 2..=this.points.len() {
            let mut found_match = false;
            for s1 in this.subsets(k) {
                for s2 in other.subsets(k) {
                    if s1.matches(&s2) {
                        found_match = true;
                        if s1.points.len() > max_size {
                            max_size = s1.points.len();
                            max_subset_1 = Some(s1.clone());
                            max_subset_2 = Some(s2.clone());
                        }
                    }
                }
            }
            if !found_match {
                return tup_to_opt(max_subset_1, max_subset_2);
            }
        }
        tup_to_opt(max_subset_1, max_subset_2)
    }

    fn barycenter(&self) -> na::SVector<f64, D> {
        let mut sum = na::SVector::from([0.; D]);
        for p in &self.points {
            sum += p.coords;
        }
        sum / self.points.len() as f64
    }

    fn translate(&self, offset: na::SVector<f64, D>) -> Self {
        Self {
            points: self.points.iter().map(|p| p + offset).collect(),
        }
    }

    fn rotate(&self, mat: na::SMatrix<f64, D, D>) -> Self {
        Self {
            points: self.points.iter().map(|p| mat * p).collect(),
        }
    }

    fn centered(&self) -> Self {
        let barycenter = self.barycenter();
        self.translate(-barycenter)
    }

    fn joined(&self, other: &Self) -> Self {
        Self {
            points: self
                .points
                .iter()
                .chain(other.points.iter())
                .cloned()
                .sorted_by(cmp_points_lexical::<D>)
                .dedup_by(points_almost_equal::<D>)
                .collect(),
        }
    }

    fn fuse(&self, other: &Self) -> Option<Self> {
        let (s1, s2) = self.maximal_matching_subsets(other)?;
        let b1 = s1.barycenter();
        let b2 = s2.barycenter();

        let c1 = s1.centered();
        let c2 = s2.centered();

        let mut pmat = None;
        let mut rmat = None;
        for permutation in permutation_matrices() {
            for reflection in reflection_matrices() {
                let rotated = c1.rotate(permutation * reflection);
                if rotated.almost_equal(&c2) {
                    pmat = Some(permutation);
                    rmat = Some(reflection);
                    break;
                }
            }
        }
        let pmat = pmat?;
        let rmat = rmat?;

        let pc1 = self.translate(-b1).rotate(pmat * rmat);
        let pc2 = other.translate(-b2);

        Some(pc1.joined(&pc2))
    }
}

impl<const D: usize> FromStr for OPointCloud<D> {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        for line in s.lines() {
            let mut iter = line.split(',');
            let mut data = [0.; D];
            for i in 0..D {
                data[i] = iter
                    .next()
                    .ok_or("Parse Error: Not enough coordinates provided.".to_string())?
                    .parse::<f64>()?;
            }
            points.push(Point::from(data));
        }
        Ok(Self::new(points))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct RefPointCloud<'a, const D: usize> {
    points: Vec<&'a Point<D>>,
}

impl<'a, const D: usize> RefPointCloud<'a, D> {
    fn new(points: Vec<&'a Point<D>>) -> Self {
        Self { points }
    }

    fn from_point_cloud(cloud: &'a OPointCloud<D>) -> Self {
        Self::new(cloud.points.iter().collect())
    }

    fn subsets(&self, size: usize) -> impl Iterator<Item = RefPointCloud<'a, D>> + '_ {
        self.points
            .iter()
            .combinations(size)
            .map(|subset| Self::new(subset.into_iter().map(|&p| p).collect()))
    }

    fn pairs(&'a self) -> impl Iterator<Item = (&'a Point<D>, &'a Point<D>)> + '_ {
        self.subsets(2).map(|pair| (pair.points[0], pair.points[1]))
    }

    fn pairwise_distances(&'a self) -> impl Iterator<Item = f64> + '_ {
        self.pairs().map(|pair| (pair.0 - pair.1).abs().sum())
    }

    /// Two point clouds are isometric iff they have the same number of points and the same
    /// pairwise distances. We need to use unordered collections to ignore permutations.
    fn matches(&self, other: &Self) -> bool {
        if !self.points.len() == other.points.len() {
            return false;
        }
        let d1 = self
            .pairwise_distances()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap());
        let d2 = other
            .pairwise_distances()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap());
        d1.zip_eq(d2).all(|(x, y)| x == y)
    }

    fn maximal_matching_subsets(
        &'a self,
        other: &'a Self,
    ) -> Option<(RefPointCloud<'_, D>, RefPointCloud<'_, D>)> {
        let mut max_size = 0;
        let mut max_subset_1 = None;
        let mut max_subset_2 = None;
        for k in 2..=self.points.len() {
            let mut found_match = false;
            for s1 in self.subsets(k) {
                for s2 in other.subsets(k) {
                    if s1.matches(&s2) {
                        found_match = true;
                        if s1.points.len() > max_size {
                            max_size = s1.points.len();
                            max_subset_1 = Some(s1.clone());
                            max_subset_2 = Some(s2.clone());
                        }
                    }
                }
            }
            if !found_match {
                return tup_to_opt(max_subset_1, max_subset_2);
            }
        }
        tup_to_opt(max_subset_1, max_subset_2)
    }

    fn barycenter(&self) -> na::SVector<f64, D> {
        let mut sum = na::SVector::<f64, D>::from([0.; D]);
        for &p in self.points.iter() {
            sum += na::SVector::<f64, D>::from(p.coords);
        }
        sum / self.points.len() as f64
    }

    fn translate(&self, offset: na::SVector<f64, D>) -> OPointCloud<D> {
        OPointCloud {
            points: self.points.iter().map(|&p| p + offset).collect(),
        }
    }

    fn rotate(&self, mat: na::SMatrix<f64, D, D>) -> OPointCloud<D> {
        OPointCloud {
            points: self.points.iter().map(|&p| mat * p).collect(),
        }
    }

    fn centered(&self) -> OPointCloud<D> {
        let barycenter = self.barycenter();
        self.translate(-barycenter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_points() {
        let p1 = Point::from([1., 2., 3.]);
        let p2 = Point::from([1., 2., 3.]);
        let p3 = Point::from([1., 2., 4.]);
        let p4 = Point::from([-2., 2., 3.]);

        assert_eq!(cmp_points_lexical(&p1, &p2), Ordering::Equal);
        assert_eq!(cmp_points_lexical(&p1, &p3), Ordering::Less);

        let points = vec![p1, p2, p3, p4];
        let sorted: Vec<Point<3>> = points
            .into_iter()
            .sorted_by(|a, b| cmp_points_lexical(a, b))
            .collect();

        assert_eq!(sorted, vec![p4, p1, p2, p3]);
    }

    #[test]
    fn test_mirror_matrices() {
        for mat in reflection_matrices::<2>() {
            println!("{}", mat);
        }
    }

    #[test]
    fn test_permutation_matrices() {
        for mat in permutation_matrices::<2>() {
            println!("{}", mat);
        }
    }

    #[test]
    fn test_point_cloud_2() {
        let input = "1,1\n1,6\n8,3\n3,4\n5,5\n8,9";
        let cloud = OPointCloud::<2>::from_str(input).unwrap();
        assert_eq!(cloud.points.len(), 6);
        assert_eq!(cloud.points[0], Point::from_slice(&[1., 1.]));
    }

    #[test]
    fn test_point_cloud_3() {
        let input = "1,1,1\n1,6,1\n8,3,1\n3,4,1\n5,5,1\n8,9,1";
        let cloud = OPointCloud::<3>::from_str(input).unwrap();
        assert_eq!(cloud.points.len(), 6);
        assert_eq!(cloud.points[0], Point::from_slice(&[1., 1., 1.]));
        assert_eq!(cloud.points[1], Point::from_slice(&[1., 6., 1.]));
        assert_eq!(cloud.points[2], Point::from_slice(&[8., 3., 1.]));
    }

    #[test]
    fn test_point_cloud_3_distances() {
        let input = "1,1,1\n1,6,1\n8,3,1\n3,4,1\n5,5,1\n8,9,1";
        let cloud = OPointCloud::<3>::from_str(input).unwrap();
        let distances: Vec<f64> = cloud.pairwise_distances().collect();
        assert_eq!(distances.len(), 15);
    }

    #[test]
    fn test_point_cloud_3_matches() {
        let input = "1,1,1\n1,6,1\n8,3,1\n3,4,1\n5,5,1\n8,9,1";
        let cloud = OPointCloud::<3>::from_str(input).unwrap();

        let input = "1,6,1\n1,1,1\n8,3,1\n3,4,1\n5,5,1\n8,9,1";
        let other = OPointCloud::<3>::from_str(input).unwrap();
        assert!(cloud.matches(&other));
    }

    #[test]
    fn test_point_cloud_3_maximal_matching_subset() {
        let input = "1,1,1\n1,6,1\n8,3,1\n3,4,1\n5,5,1\n8,9,1";
        let cloud = OPointCloud::<3>::from_str(input).unwrap();

        let input = "2,7,2\n2,2,2\n9,4,2\n1,2,3\n4,5,6\n7,8,9";
        let other = OPointCloud::<3>::from_str(input).unwrap();
        let (s1, s2) = cloud.maximal_matching_subsets(&other).unwrap();

        assert_eq!(s1.points.len(), 3);
        assert_eq!(s2.points.len(), 3);
    }

    #[test]
    fn test_centered() {
        let input = "1,1,1\n1,6,1\n8,3,1\n3,4,1\n5,5,1\n8,9,1";
        let cloud = OPointCloud::<3>::from_str(input).unwrap();

        let input = "2,2,2\n2,7,2\n9,4,2\n1,2,3\n4,5,6\n7,8,9";
        let other = OPointCloud::<3>::from_str(input).unwrap();
        let (s1, s2) = cloud.maximal_matching_subsets(&other).unwrap();

        let s1 = s1.centered();
        let s2 = s2.centered();

        assert!(s1
            .points
            .iter()
            .zip_eq(s2.points.iter())
            .all(|(p1, p2)| (p1 - p2).norm() < 1e-6));
    }

    #[test]
    fn test_fuse_translated() {
        let input = "1,1,1\n1,6,1\n8,3,1\n3,4,1\n5,5,1\n8,9,1";
        let cloud_1 = OPointCloud::<3>::from_str(input).unwrap();

        let input = "2,2,2\n2,7,2\n9,4,2\n1,2,3\n4,5,6\n7,8,9";
        let cloud_2 = OPointCloud::<3>::from_str(input).unwrap();

        let fused = cloud_1.fuse(&cloud_2).unwrap();

        assert_eq!(fused.points.len(), 9);
    }

    #[test]
    fn test_fuse_transformed() {
        let input = "1,1\n1,6\n8,3\n3,4\n5,5\n8,9";
        let cloud_1 = OPointCloud::<2>::from_str(input).unwrap();

        let input = "-1,-1\n-6,-1\n-3,-8\n1,2\n3,4\n5,6";
        let cloud_2 = OPointCloud::<2>::from_str(input).unwrap();

        let fused = cloud_1.fuse(&cloud_2).unwrap();

        assert_eq!(fused.points.len(), 9);
    }
}
