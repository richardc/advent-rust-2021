use std::collections::HashSet;

use euclid::{box3d, default::Box3D};
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

fn parse_box(input: &str) -> Box3D<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"x=(?P<x1>-?\d+)..(?P<x2>-?\d+),y=(?P<y1>-?\d+)..(?P<y2>-?\d+),z=(?P<z1>-?\d+)..(?P<z2>-?\d+)")
        .unwrap();
    }
    let captures = RE.captures(input).unwrap();

    box3d(
        captures["x1"].parse().unwrap(),
        captures["y1"].parse().unwrap(),
        captures["z1"].parse().unwrap(),
        captures["x2"].parse::<i64>().unwrap() + 1,
        captures["y2"].parse::<i64>().unwrap() + 1,
        captures["z2"].parse::<i64>().unwrap() + 1,
    )
}

#[derive(PartialEq)]
enum Operation {
    On,
    Off,
}
struct Instruction {
    operation: Operation,
    region: Box3D<i64>,
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
    cells: HashSet<[i64; 3]>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            cells: HashSet::new(),
        }
    }

    fn lit_cubes(&self) -> usize {
        self.cells.len()
    }
}

impl Reactor {
    fn apply(&mut self, instruction: &Instruction) {
        for (x, y, z) in iproduct!(
            instruction.region.x_range(),
            instruction.region.y_range(),
            instruction.region.z_range()
        ) {
            match instruction.operation {
                Operation::On => self.cells.insert([x, y, z]),
                Operation::Off => self.cells.remove(&[x, y, z]),
            };
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

#[aoc(day22, part1, slicing)]
fn lit_initialized_cubes_slicing(input: &[Instruction]) -> usize {
    let mut reactor = SlicingReactor::new();

    // Only consider operations on our core
    let bounds = parse_box("x=-50..50,y=-50..50,z=-50..50");

    input
        .iter()
        .filter(|i| bounds.contains_box(&i.region))
        .for_each(|i| reactor.apply(i));

    reactor.lit_cubes()
}

#[test]
fn test_lit_initialized_cubes_slicing() {
    assert_eq!(
        lit_initialized_cubes_slicing(&generate(include_str!("day22_example.txt"))),
        590784
    );
}

#[derive(Default)]
struct SlicingReactor {
    lit_regions: Vec<Box3D<i64>>,
}

fn slice_cube(cube: &Box3D<i64>, cut: &Box3D<i64>) -> Vec<Box3D<i64>> {
    let mut result = vec![];
    // Generate cubelets by slicing the cube by its intersecting cut.
    let xs = [cube.max.x, cube.min.x, cut.max.x, cut.min.x]
        .into_iter()
        .sorted()
        .tuple_windows::<(_, _)>();

    let ys = [cube.max.y, cube.min.y, cut.max.y, cut.min.y]
        .into_iter()
        .sorted()
        .tuple_windows::<(_, _)>();

    let zs = [cube.max.z, cube.min.z, cut.max.z, cut.min.z]
        .into_iter()
        .sorted()
        .tuple_windows::<(_, _)>();

    for ((x1, x2), (y1, y2), (z1, z2)) in iproduct!(xs, ys, zs) {
        let cubelet = box3d(x1, y1, z1, x2, y2, z2);
        // Lose the cutting tool
        if *cut != cubelet {
            result.push(cubelet);
        }
    }

    result
}

#[test]
fn test_slice_cube() {
    assert_eq!(
        slice_cube(&box3d(0, 0, 0, 10, 10, 10), &box3d(0, 0, 0, 1, 1, 1)).len(),
        26,
    );
}

impl SlicingReactor {
    fn new() -> Self {
        Self::default()
    }

    fn apply(&mut self, instruction: &Instruction) {
        let mut regions = vec![];

        for lit in &self.lit_regions {
            if let Some(intersection) = lit.intersection(&instruction.region) {
                slice_cube(lit, &intersection).iter().for_each(|b| {
                    regions.push(*b);
                });
            } else {
                regions.push(*lit);
            }
        }

        if instruction.operation == Operation::On {
            regions.push(instruction.region)
        }
        self.lit_regions = regions;
    }

    fn lit_cubes(&self) -> usize {
        self.lit_regions.iter().map(|b| b.volume() as usize).sum()
    }
}

#[aoc(day22, part2)]
fn lit_all_cubes(input: &[Instruction]) -> usize {
    let mut reactor = SlicingReactor::new();

    input.iter().for_each(|i| reactor.apply(i));

    reactor.lit_cubes()
}

#[test]
fn test_lit_all_cubes() {
    assert_eq!(
        lit_all_cubes(&generate(include_str!("day22_example_2.txt"))),
        2758514936282235
    );
}
