#[allow(dead_code)] // used in tests
const EXAMPLE: &str = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

use std::{cmp::min, collections::HashSet};

use ndarray::prelude::*;

#[derive(Default, Clone)]
struct State {
    data: Array<u8, Ix2>,
}

impl State {
    #[allow(dead_code)] // used in tests
    fn new(x: usize, y: usize) -> Self {
        State {
            data: Array::zeros((x, y)),
        }
    }
}

impl From<Vec<&str>> for State {
    fn from(lines: Vec<&str>) -> Self {
        let x = lines[0].len();
        let y = lines.len();
        State {
            data: Array::from_shape_vec(
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

#[test]
fn test_state_from() {
    let vec = EXAMPLE.trim().split('\n').to_owned().collect::<Vec<_>>();
    let state = State::from(vec);
    assert_eq!(state.data[[0, 0]], 5);
    assert_eq!(state.data[[0, 1]], 4);
    assert_eq!(state.data.dim(), (10, 10));
}

impl State {
    fn step(&mut self) -> usize {
        // You get an energy, and you get an energy!
        self.data += 1;

        let (xmax, ymax) = self.data.dim();
        let mut seen = HashSet::<(usize, usize)>::new();
        loop {
            let flashed = HashSet::from_iter(
                self.data
                    .indexed_iter()
                    .filter(|((_, _), &v)| v > 9)
                    .map(|(point, _)| point),
            );
            if flashed != seen {
                // new flashes to handle
                for &(x, y) in flashed.difference(&seen) {
                    // add energy to each neighbour
                    let mut neighbours = self.data.slice_mut(s![
                        x.saturating_sub(1)..min(xmax, x + 2),
                        y.saturating_sub(1)..min(ymax, y + 2)
                    ]);
                    neighbours += 1;
                }
                seen = flashed;
            } else {
                // reset all flashed cells to 0
                flashed.iter().for_each(|(x, y)| self.data[[*x, *y]] = 0);
                return flashed.len();
            }
        }
    }
}

#[test]
fn test_state_step() {
    let mut state = State::new(3, 3);
    state.data[[0, 0]] = 8;
    assert_eq!(state.step(), 0);
    assert_eq!(state.data, aview2(&[[9, 1, 1], [1, 1, 1], [1, 1, 1]]));

    assert_eq!(state.step(), 1);
    assert_eq!(state.data, aview2(&[[0, 3, 2], [3, 3, 2], [2, 2, 2]]));

    assert_eq!(state.step(), 0);
    assert_eq!(state.data, aview2(&[[1, 4, 3], [4, 4, 3], [3, 3, 3]]));
}

fn count_flashes(start: &State, steps: usize) -> usize {
    let mut total = 0;
    let mut state = start.clone();
    for _ in 0..steps {
        total += state.step();
    }
    total
}

#[test]
fn test_count_flashes() {
    let vec = EXAMPLE.trim().split('\n').to_owned().collect::<Vec<_>>();
    let state = State::from(vec);
    assert_eq!(count_flashes(&state, 10), 204);
    assert_eq!(count_flashes(&state, 100), 1656);
}

fn when_everyone_flashes(start: &State) -> usize {
    let mut state = start.clone();
    let everyone = state.data.len();
    for step in 1.. {
        if state.step() == everyone {
            return step;
        }
    }
    unreachable!()
}

#[test]
fn test_when_everyone_flashes() {
    let vec = EXAMPLE.trim().split('\n').to_owned().collect::<Vec<_>>();
    let state = State::from(vec);

    assert_eq!(when_everyone_flashes(&state), 195);
}

use std::io;

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let input = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();

    let state = State::from(input);
    println!("{}", count_flashes(&state, 100));
    println!("{}", when_everyone_flashes(&state));
}
