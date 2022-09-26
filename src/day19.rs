use itertools::Itertools;

#[derive(Debug, PartialEq)]
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

#[derive(Default)]
struct Scanner {
    probes: Vec<Point>,
}

impl<'a> FromIterator<&'a str> for Scanner {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut lines = iter.into_iter();
        lines.next(); // drop separator
        Scanner {
            probes: lines.map(|s| Point::from(s)).collect(),
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

#[aoc(day19, part1)]
fn count_beacons(_scanners: &[Scanner]) -> usize {
    0
}

#[cfg(test)]
const EXAMPLE: &str = include_str!("day19_example.txt");

#[test]
fn test_count_beacons() {
    assert_eq!(count_beacons(&generate(EXAMPLE.trim())), 79);
}
