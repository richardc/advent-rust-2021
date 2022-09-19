use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Point(usize, usize);

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        if let Some((x, y)) = s.split_once(',') {
            Self(x.parse().unwrap(), y.parse().unwrap())
        } else {
            unreachable!();
        }
    }
}

impl Point {
    fn fold_y(&self, line: usize) -> Self {
        if self.1 > line {
            Self(self.0, line - (self.1 - line))
        } else {
            *self
        }
    }

    fn fold_x(&self, line: usize) -> Self {
        if self.0 > line {
            Self(line - (self.0 - line), self.1)
        } else {
            *self
        }
    }
}

#[test]
fn test_point_folding() {
    assert_eq!(Point(0, 0).fold_y(10), Point(0, 0));
    assert_eq!(Point(4, 20).fold_y(10), Point(4, 0));
    assert_eq!(Point(5, 11).fold_y(10), Point(5, 9));

    assert_eq!(Point(0, 0).fold_x(3), Point(0, 0));
    assert_eq!(Point(4, 20).fold_x(3), Point(2, 20));
    assert_eq!(Point(5, 11).fold_x(3), Point(1, 11));
}

#[derive(Debug)]
struct Puzzle {
    points: Vec<Point>,
    commands: Vec<String>,
}

impl From<Vec<&str>> for Puzzle {
    fn from(input: Vec<&str>) -> Self {
        Puzzle {
            points: input
                .iter()
                .take_while(|s| !s.is_empty())
                .map(|&s| Point::from(s))
                .collect(),
            commands: input
                .iter()
                .skip_while(|s| !s.is_empty())
                .skip(1)
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl Puzzle {
    fn apply_command(&mut self, index: usize) {
        let command = &self.commands[index];
        if command.starts_with("fold along y=") {
            if let Some((_, y)) = command.split_once('=') {
                let y = y.parse::<usize>().unwrap();
                let points = self.points.iter().map(|p| p.fold_y(y)).unique().collect();
                self.points = points;
            }
        }

        if command.starts_with("fold along x=") {
            if let Some((_, x)) = command.split_once('=') {
                let x = x.parse::<usize>().unwrap();
                let points = self.points.iter().map(|p| p.fold_x(x)).unique().collect();
                self.points = points;
            }
        }
    }

    fn apply_commands(&mut self) {
        for i in 0..self.commands.len() {
            self.apply_command(i)
        }
    }

    fn print(&self) {
        let xmax = self.points.iter().map(|p| p.0).max().unwrap();
        let ymax = self.points.iter().map(|p| p.1).max().unwrap();
        for y in 0..=ymax {
            println!(
                "{}",
                (0..=xmax)
                    .map(|x| if self.points.contains(&Point(x, y)) {
                        '#'
                    } else {
                        ' '
                    })
                    .collect::<String>()
            )
        }
    }
}

#[test]
fn test_puzzle() {
    let example = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    let mut puzzle = Puzzle::from(example.trim().split('\n').collect::<Vec<&str>>());
    assert_eq!(puzzle.points.len(), 18);
    assert_eq!(puzzle.commands.len(), 2);

    puzzle.apply_command(0);
    assert_eq!(puzzle.points.len(), 17);

    let mut puzzle = Puzzle::from(example.trim().split('\n').collect::<Vec<&str>>());
    puzzle.apply_commands();
    assert_eq!(puzzle.points.len(), 16);
}

use std::io;

fn step1(input: Vec<&str>) -> usize {
    let mut puzzle = Puzzle::from(input);
    puzzle.apply_command(0);
    puzzle.points.len()
}

fn step2(input: Vec<&str>) {
    let mut puzzle = Puzzle::from(input);
    puzzle.apply_commands();
    puzzle.print();
}

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let input = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();

    println!("{}", step1(input.clone()));
    step2(input);
}
