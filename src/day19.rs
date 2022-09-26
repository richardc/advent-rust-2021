use itertools::Itertools;
use std::{collections::HashSet, ops};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let v = s.split(',').map(|x| x.parse().unwrap()).collect_vec();
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(
        Point::new(0, 0, 0).manhattan_distance(&Point::new(1, 0, 0)),
        1,
    );

    assert_eq!(
        Point::new(0, 0, 0).manhattan_distance(&Point::new(-100, 100, 200)),
        400,
    );
}

#[derive(Default)]
struct Scanner {
    probes: Vec<Point>,
}

impl<'a> FromIterator<&'a str> for Scanner {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut lines = iter.into_iter();
        lines.next(); // drop separator
        Scanner {
            probes: lines.map(Point::from).collect(),
        }
    }
}

#[aoc_generator(day19)]
fn generate(input: &str) -> Vec<Scanner> {
    input
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, group)| Scanner::from_iter(group))
        .collect()
}

#[test]
fn test_generate() {
    let scanners = generate(EXAMPLE);
    assert_eq!(scanners.len(), 5);
    assert_eq!(scanners[0].probes.len(), 25);
    assert_eq!(
        scanners[0].probes[0],
        Point {
            x: 404,
            y: -588,
            z: -901
        }
    );
}

impl Scanner {
    fn get_orientations(&self) -> Vec<Vec<Point>> {
        // Turn to face a direction, 6 of
        let facings: &[fn(&Point) -> Point] = &[
            |&Point { x, y, z }| Point::new(x, y, z),
            |&Point { x, y, z }| Point::new(y, -x, z),
            |&Point { x, y, z }| Point::new(-x, -y, z),
            |&Point { x, y, z }| Point::new(-y, x, z),
            |&Point { x, y, z }| Point::new(y, z, x),
            |&Point { x, y, z }| Point::new(y, -z, -x),
        ];

        // Rotate to find 'up', 4 of
        let rotations: &[fn(&Point) -> Point] = &[
            |&Point { x, y, z }| Point::new(x, y, z),
            |&Point { x, y, z }| Point::new(x, -z, y),
            |&Point { x, y, z }| Point::new(x, -y, -z),
            |&Point { x, y, z }| Point::new(x, z, -y),
        ];

        rotations
            .iter()
            .cartesian_product(facings)
            .map(|(face, rotate)| self.probes.iter().map(|p| rotate(&face(p))).collect_vec())
            .collect_vec()
    }
}

#[test]
fn test_rotations() {
    let scanners = generate(include_str!("day19_small_example.txt"));
    let permutations = scanners[0].get_orientations();
    assert_eq!(permutations.len(), 24);
    assert_eq!(permutations[0], scanners[0].probes);
    assert_eq!(permutations.iter().any(|p| *p == scanners[1].probes), true);

    assert_eq!(
        scanners
            .iter()
            .all(|scanner| permutations.iter().any(|p| *p == scanner.probes)),
        true
    );
}

fn beacons_match(known: &mut HashSet<Point>, sensor: &Scanner) -> Option<Point> {
    let orientations = sensor.get_orientations();
    for rotated in &orientations {
        let distances = known
            .iter()
            .cartesian_product(rotated)
            .map(|(known, check)| *known - *check);

        for distance in distances {
            let translated = rotated.iter().map(|p| *p + distance);
            if translated.clone().filter(|p| known.contains(p)).count() >= 12 {
                known.extend(translated);
                return Some(distance);
            }
        }
    }
    None
}

#[test]
fn test_beacons_match() {
    let scanners = generate(include_str!("day19_example.txt"));
    let sensors = Vec::from_iter(&scanners[1..]);
    let mut known: HashSet<Point> = HashSet::from_iter(scanners[0].probes.clone());

    assert_eq!(
        beacons_match(&mut known, sensors[0]),
        Some(Point::new(68, -1246, -43))
    );
}

fn assemble_points(scanners: &[Scanner]) -> (usize, Vec<Point>) {
    let mut sensors = Vec::from_iter(&scanners[1..]);
    let mut positions = vec![];
    let mut known: HashSet<Point> = HashSet::from_iter(scanners[0].probes.clone());
    'outer: while !sensors.is_empty() {
        for i in 0..sensors.len() {
            if let Some(position) = beacons_match(&mut known, sensors[i]) {
                positions.push(position);
                sensors.remove(i);
                continue 'outer;
            }
        }
        // We should match at least one sensor per pass
        unreachable!()
    }

    (known.len(), positions)
}

#[aoc(day19, part1)]
fn count_beacons(scanners: &[Scanner]) -> usize {
    assemble_points(scanners).0
}

#[cfg(test)]
const EXAMPLE: &str = include_str!("day19_example.txt");

#[test]
fn test_count_beacons() {
    assert_eq!(count_beacons(&generate(EXAMPLE.trim())), 79);
}

#[aoc(day19, part2)]
fn how_wide_was_it(scanners: &[Scanner]) -> i32 {
    let points = assemble_points(scanners).1;
    points
        .iter()
        .cartesian_product(points.iter())
        .map(|(p1, p2)| p1.manhattan_distance(p2))
        .max()
        .unwrap()
}

#[test]
fn test_how_wide_was_it() {
    assert_eq!(how_wide_was_it(&generate(EXAMPLE.trim())), 3621);
}
