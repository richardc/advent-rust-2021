#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl From<String> for Point {
    fn from(s: String) -> Self {
        if let Some((x, y)) = s.split_once(',') {
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        } else {
            Point::default()
        }
    }
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Self {
        Point { x, y }
    }
}

#[test]
fn test_point() {
    let point = Point::from(String::from("1,2"));
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 2);
}

#[derive(Default, Debug, Clone, Copy)]
struct Edge {
    start: Point,
    end: Point,
}

impl From<String> for Edge {
    fn from(s: String) -> Self {
        if let Some((start, end)) = s.split_once(" -> ") {
            Edge {
                start: Point::from(start.to_string()),
                end: Point::from(end.to_string()),
            }
        } else {
            Edge::default()
        }
    }
}

use std::cmp::{max, min};

impl Edge {
    fn points(self) -> Vec<Point> {
        if self.start.x == self.end.x {
            let y1 = min(self.start.y, self.end.y);
            let y2 = max(self.start.y, self.end.y);
            return (y1..=y2).map(|y| Point::from((self.start.x, y))).collect();
        }
        if self.start.y == self.end.y {
            let x1 = min(self.start.x, self.end.x);
            let x2 = max(self.start.x, self.end.x);
            return (x1..=x2).map(|x| Point::from((x, self.start.y))).collect();
        }
        vec![]
    }

    fn diagonal_points(self) -> Vec<Point> {
        let points = self.points();
        if !points.is_empty() {
            points
        } else {
            let x1 = min(self.start.x, self.end.x);
            let x2 = max(self.start.x, self.end.x);

            let mut x = (x1..=x2).collect::<Vec<_>>();
            if self.start.x > self.end.x {
                x.reverse();
            }

            let y1 = min(self.start.y, self.end.y);
            let y2 = max(self.start.y, self.end.y);
            let mut y = (y1..=y2).collect::<Vec<_>>();
            if self.start.y > self.end.y {
                y.reverse();
            }

            x.into_iter().zip(y.into_iter()).map(Point::from).collect()
        }
    }
}

#[test]
fn test_edge() {
    let edge = Edge::from(String::from("1,2 -> 1,4"));
    assert_eq!(edge.start.x, 1);
    assert_eq!(edge.start.y, 2);
    assert_eq!(edge.end.x, 1);
    assert_eq!(edge.end.y, 4);
    assert_eq!(
        edge.points(),
        vec![(1, 2), (1, 3), (1, 4)]
            .iter()
            .map(|x| Point::from(*x))
            .collect::<Vec<_>>(),
    );

    let edge = Edge::from(String::from("3,4 -> 2,4"));
    assert_eq!(
        edge.points(),
        vec![(2, 4), (3, 4)]
            .iter()
            .map(|x| Point::from(*x))
            .collect::<Vec<_>>(),
    );

    let edge = Edge::from(String::from("1,1 -> 3,3"));
    assert_eq!(
        edge.diagonal_points(),
        vec![(1, 1), (2, 2), (3, 3)]
            .iter()
            .map(|x| Point::from(*x))
            .collect::<Vec<_>>(),
    );

    let edge = Edge::from(String::from("9,7 -> 7,9"));
    assert_eq!(
        edge.diagonal_points(),
        vec![(9, 7), (8, 8), (7, 9)]
            .iter()
            .map(|x| Point::from(*x))
            .collect::<Vec<_>>(),
    );
}
use std::collections::HashMap;

#[allow(dead_code)]
fn draw_points(points: &HashMap<Point, i32>) {
    let x1 = points.keys().map(|p| p.x).min().unwrap();
    let x2 = points.keys().map(|p| p.x).max().unwrap();
    let y1 = points.keys().map(|p| p.y).min().unwrap();
    let y2 = points.keys().map(|p| p.y).max().unwrap();
    for y in y1..=y2 {
        for x in x1..=x2 {
            print!(
                "{} ",
                match points.get(&Point::from((x, y))) {
                    Some(n) => *n,
                    None => 0,
                }
            )
        }
        println!();
    }
    println!();
}

fn highway_to_the_danger_zones(lines: &[String], points: fn(Edge) -> Vec<Point>) -> u32 {
    let points = lines
        .iter()
        .map(|l| Edge::from(l.to_string()))
        .flat_map(points)
        .fold(HashMap::new(), |mut acc, p| {
            acc.entry(p).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

    //draw_points(&points);

    points.values().filter(|p| **p >= 2).count() as u32
}

fn danger_zones(lines: &[String]) -> u32 {
    highway_to_the_danger_zones(lines, |e| e.points())
}

fn diagonal_danger_zones(lines: &[String]) -> u32 {
    highway_to_the_danger_zones(lines, |e| e.diagonal_points())
}

#[test]
fn test_danger_zones() {
    let example = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
    let lines = example
        .to_string()
        .split('\n')
        .skip(1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    assert_eq!(danger_zones(&lines), 5);
    assert_eq!(diagonal_danger_zones(&lines), 12);
}

use std::io;

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    println!("{}", danger_zones(&lines));
    println!("{}", diagonal_danger_zones(&lines));
}
