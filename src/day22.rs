use std::collections::HashSet;

use euclid::{box3d, default::Box3D};
use itertools::iproduct;
use lazy_static::lazy_static;
use regex::Regex;

fn parse_box(input: &str) -> Box3D<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"x=(?P<x1>-?\d+)..(?P<x2>-?\d+),y=(?P<y1>-?\d+)..(?P<y2>-?\d+),z=(?P<z1>-?\d+)..(?P<z2>-?\d+)")
        .unwrap();
    }
    let captures = RE.captures(input).unwrap();

    box3d(
        captures["x1"].parse().unwrap(),
        captures["y1"].parse().unwrap(),
        captures["z1"].parse().unwrap(),
        captures["x2"].parse::<i32>().unwrap() + 1,
        captures["y2"].parse::<i32>().unwrap() + 1,
        captures["z2"].parse::<i32>().unwrap() + 1,
    )
}

enum Operation {
    On,
    Off,
}
struct Instruction {
    operation: Operation,
    region: Box3D<i32>,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let (op, region) = input.split_once(' ').unwrap();
        Instruction {
            region: parse_box(region),
            operation: match op {
                "on" => Operation::On,
                "off" => Operation::Off,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Clone)]
struct Reactor {
    bounds: Box3D<i32>,
    cells: HashSet<[i32; 3]>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            bounds: parse_box("x=-50..50,y=-50..50,z=-50..50"),
            cells: HashSet::new(),
        }
    }

    fn lit_cubes(&self) -> usize {
        self.cells.len()
    }
}

impl Reactor {
    fn apply(&mut self, instruction: &Instruction) {
        if let Some(bounds) = self.bounds.intersection(&instruction.region) {
            for (x, y, z) in iproduct!(bounds.x_range(), bounds.y_range(), bounds.z_range()) {
                match instruction.operation {
                    Operation::On => self.cells.insert([x, y, z]),
                    Operation::Off => self.cells.remove(&[x, y, z]),
                };
            }
        }
    }
}

#[test]
fn test_instruction_apply() {
    let mut reactor = Reactor::new();
    reactor.apply(&Instruction::from("on x=10..12,y=10..12,z=10..12"));
    assert_eq!(reactor.lit_cubes(), 27);

    reactor.apply(&Instruction::from("on x=11..13,y=11..13,z=11..13"));
    assert_eq!(reactor.lit_cubes(), 46);

    reactor.apply(&Instruction::from("off x=9..11,y=9..11,z=9..11"));
    assert_eq!(reactor.lit_cubes(), 38);

    reactor.apply(&Instruction::from("on x=10..10,y=10..10,z=10..10"));
    assert_eq!(reactor.lit_cubes(), 39);
}

#[aoc_generator(day22)]
fn generate(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

#[aoc(day22, part1)]
fn lit_initialized_cubes(input: &[Instruction]) -> usize {
    let mut reactor = Reactor::new();

    // Only consider operations on our core
    let bounds = parse_box("x=-50..50,y=-50..50,z=-50..50");

    input
        .iter()
        .filter(|i| bounds.contains_box(&i.region))
        .for_each(|i| reactor.apply(i));

    reactor.lit_cubes()
}

#[test]
fn test_lit_initialized_cubes() {
    assert_eq!(
        lit_initialized_cubes(&generate(include_str!("day22_example.txt"))),
        590784
    );
}
