use ndarray::prelude::*;
use std::{cmp::Ordering, collections::BinaryHeap, io};

#[derive(Debug)]
struct Puzzle {
    map: Array2<u8>,
}

impl FromIterator<String> for Puzzle {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let lines = iter.into_iter().collect::<Vec<_>>();
        let x = lines[0].len();
        let y = lines.len();
        Puzzle {
            map: Array::from_shape_vec(
                (y, x),
                lines
                    .iter()
                    .flat_map(|s| s.chars().map(|c| c as u8 - b'0'))
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(usize, usize);

#[derive(PartialEq, Eq)]
struct State {
    cost: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Puzzle {
    fn adjacent(&self, Point(x, y): Point) -> Vec<Point> {
        let mut adj = vec![];
        if x > 1 {
            adj.push(Point(x - 1, y));
        }
        if x < self.map.dim().0 - 1 {
            adj.push(Point(x + 1, y));
        }

        if y > 1 {
            adj.push(Point(x, y - 1));
        }
        if y < self.map.dim().1 - 1 {
            adj.push(Point(x, y + 1));
        }

        adj
    }

    fn shortest_path(&self) -> usize {
        let start = Point(0, 0);
        let end = Point(self.map.dim().0 - 1, self.map.dim().1 - 1);

        // Dijkstra's algorithm - from the manpage for std::collections::binaryheap
        let mut dist = Array::from_elem(self.map.dim(), usize::MAX);
        dist[[start.0, start.1]] = 0;

        let mut queue = BinaryHeap::new();
        queue.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = queue.pop() {
            if position == end {
                return cost;
            }

            if cost > dist[[position.0, position.1]] {
                continue;
            }

            for location in self.adjacent(position) {
                let next = State {
                    cost: cost + self.map[[location.0, location.1]] as usize,
                    position: location,
                };

                if next.cost < dist[[location.0, location.1]] {
                    dist[[location.0, location.1]] = next.cost;
                    queue.push(next);
                }
            }
        }
        unreachable!()
    }
}

#[test]
fn test_puzzle() {
    let example = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    let lines = example.trim().split('\n').map(|s| s.to_string());
    let puzzle = Puzzle::from_iter(lines);

    assert_eq!(puzzle.map[[0, 0]], 1);
    assert_eq!(puzzle.map[[5, 5]], 2);
    assert_eq!(puzzle.shortest_path(), 40);
}

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap());
    let puzzle = Puzzle::from_iter(lines);

    println!("{}", puzzle.shortest_path());
}
