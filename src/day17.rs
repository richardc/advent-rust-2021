use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::recognize,
    sequence::{pair, preceded, separated_pair},
    IResult,
};
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Target {
    top_left: Point,
    bottom_right: Point,
}

fn parse_signed_number(input: &str) -> IResult<&str, i32> {
    let (input, value) = alt((recognize(pair(char('-'), digit1)), digit1))(input)?;

    Ok((input, value.parse().unwrap()))
}

fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (start, end)) =
        separated_pair(parse_signed_number, tag(".."), parse_signed_number)(input)?;

    Ok((input, (start, end)))
}

fn parse_target(input: &str) -> IResult<&str, (i32, i32, i32, i32)> {
    let (input, ((x1, x2), (y1, y2))) = pair(
        preceded(tag("target area: x="), parse_range),
        preceded(tag(", y="), parse_range),
    )(input)?;

    Ok((input, (x1, x2, y1, y2)))
}

#[test]
fn test_parser() {
    assert_eq!(
        parse_target("target area: x=2..20, y=-10..-20"),
        Ok(("", (2, 20, -10, -20)))
    );
}

impl From<String> for Target {
    fn from(s: String) -> Self {
        if let Ok((_, (x1, x2, y1, y2))) = parse_target(&s) {
            Target {
                top_left: Point {
                    x: min(x1, x2),
                    y: max(y1, y2),
                },
                bottom_right: Point {
                    x: max(x1, x2),
                    y: min(y1, y2),
                },
            }
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test_target_from() {
    assert_eq!(
        Target::from(String::from("target area: x=20..30, y=-10..-5")),
        Target {
            top_left: Point { x: 20, y: -5 },
            bottom_right: Point { x: 30, y: -10 }
        }
    );
}

impl Target {
    fn hit(&self, p: Point) -> bool {
        p.x >= self.top_left.x
            && p.y <= self.top_left.y
            && p.x <= self.bottom_right.x
            && p.y >= self.bottom_right.y
    }
}

#[test]
fn test_target_hit() {
    let target = Target::from(String::from("target area: x=20..30, y=-10..-5"));

    assert_eq!(target.hit(Point::new(20, -5)), true);
    assert_eq!(target.hit(Point::new(30, -5)), true);
    assert_eq!(target.hit(Point::new(20, -10)), true);
    assert_eq!(target.hit(Point::new(30, -10)), true);
    assert_eq!(target.hit(Point::new(0, 0)), false);
    assert_eq!(target.hit(Point::new(0, -10)), false);
}

impl Target {
    fn missed(&self, p: Point) -> bool {
        p.x > self.bottom_right.x || p.y < self.bottom_right.y
    }
}

#[test]
fn test_target_missed() {
    let target = Target::from(String::from("target area: x=20..30, y=-10..-5"));

    assert_eq!(target.missed(Point::new(30, -10)), false);
    assert_eq!(target.missed(Point::new(0, 0)), false);
    assert_eq!(target.missed(Point::new(30, -11)), true);
    assert_eq!(target.missed(Point::new(31, -10)), true);
}

#[derive(Debug, PartialEq)]
struct Probe {
    p: Point,
    v: Point,
}

impl Probe {
    fn launch(x: i32, y: i32) -> Self {
        Self {
            p: Point { x: 0, y: 0 },
            v: Point { x, y },
        }
    }

    fn step(&mut self) {
        self.p.x += self.v.x;
        self.p.y += self.v.y;

        self.v.x -= self.v.x.signum();
        self.v.y -= 1;
    }
}

#[test]
fn test_probe() {
    let mut probe = Probe::launch(7, 2);
    assert_eq!(
        probe,
        Probe {
            p: Point { x: 0, y: 0 },
            v: Point { x: 7, y: 2 }
        }
    );
    probe.step();
    assert_eq!(
        probe,
        Probe {
            p: Point { x: 7, y: 2 },
            v: Point { x: 6, y: 1 }
        }
    );

    probe.step();
    assert_eq!(
        probe,
        Probe {
            p: Point { x: 13, y: 3 },
            v: Point { x: 5, y: 0 }
        }
    );

    probe.step();
    assert_eq!(
        probe,
        Probe {
            p: Point { x: 18, y: 3 },
            v: Point { x: 4, y: -1 }
        }
    );
}

fn find_height(t: &Target, x: i32, y: i32) -> Option<i32> {
    let mut probe = Probe::launch(x, y);
    let mut max = 0;
    loop {
        if probe.p.y > max {
            max = probe.p.y;
        }

        if t.hit(probe.p.clone()) {
            return Some(max);
        }

        if t.missed(probe.p.clone()) {
            return None;
        }

        probe.step();
    }
}

#[test]
fn test_find_height() {
    let target = Target::from(String::from("target area: x=20..30, y=-10..-5"));

    assert_eq!(find_height(&target, 17, -4), None);

    assert_eq!(find_height(&target, 7, 2), Some(3));
    assert_eq!(find_height(&target, 6, 3), Some(6));
    assert_eq!(find_height(&target, 9, 0), Some(0));

    assert_eq!(find_height(&target, 6, 9), Some(45));
}

#[aoc_generator(day17)]
fn generate(input: &str) -> Target {
    Target::from(String::from(input))
}

// Probably woefully ineffienct to search such a wide space rather than
// calculating some bounds relative to the target
#[aoc(day17, part1)]
fn find_max_height(t: &Target) -> i32 {
    (0..1000)
        .cartesian_product(0..1000)
        .filter_map(|(x, y)| find_height(t, x, y))
        .max()
        .unwrap()
}

#[test]
fn test_find_max_height() {
    let target = Target::from(String::from("target area: x=20..30, y=-10..-5"));
    assert_eq!(find_max_height(&target), 45);
}

#[aoc(day17, part2)]
fn count_firing_vectors(t: &Target) -> usize {
    (0..1000)
        .cartesian_product(-1000..1000)
        .filter_map(|(x, y)| find_height(t, x, y))
        .count()
}

#[test]
fn test_count_firing_vectors() {
    let target = Target::from(String::from("target area: x=20..30, y=-10..-5"));
    assert_eq!(count_firing_vectors(&target), 112);
}
