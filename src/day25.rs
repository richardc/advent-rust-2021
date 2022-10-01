use itertools::Itertools;
use ndarray::prelude::*;

type Cucumbers = Array2<u8>;

#[test]
fn test_generate() {
    let state = generate("...>>>>>...");
    assert_eq!(state.dim(), (11, 1));
    assert_eq!(state[[1, 0]], b'.');
}

fn step(state: &Cucumbers) -> Cucumbers {
    let state = step_right(state);
    step_down(&state)
}

fn step_down(state: &Cucumbers) -> Cucumbers {
    let (_, max_y) = state.dim();
    let mut moves = vec![];
    for ((x, y), c) in state.indexed_iter() {
        if *c == b'v' {
            if state[[x, (y + 1) % max_y]] == b'.' {
                moves.push(((x, y), (x, (y + 1) % max_y)));
            }
        }
    }

    let mut next = state.clone();
    for ((x1, y1), (x2, y2)) in moves {
        next[[x2, y2]] = next[[x1, y1]];
        next[[x1, y1]] = b'.';
    }

    next
}

fn step_right(state: &Cucumbers) -> Cucumbers {
    let (max_x, _) = state.dim();
    let mut moves = vec![];
    for ((x, y), c) in state.indexed_iter() {
        if *c == b'>' {
            if state[[(x + 1) % max_x, y]] == b'.' {
                moves.push(((x, y), ((x + 1) % max_x, y)));
            }
        }
    }

    let mut next = state.clone();
    for ((x1, y1), (x2, y2)) in moves {
        next[[x2, y2]] = next[[x1, y1]];
        next[[x1, y1]] = b'.';
    }

    next
}

#[test]
fn test_step_east_moves() {
    let state = generate("...>>>>>...");

    let state = step_right(&state);
    assert_eq!(state, generate("...>>>>.>.."), "1 step");

    let state = step_right(&state);
    assert_eq!(state, generate("...>>>.>.>."));
}

#[aoc_generator(day25)]
fn generate(input: &str) -> Cucumbers {
    let rows = input.lines().collect_vec().len();
    let cols = input.lines().next().unwrap().len();
    let data = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c as u8)
        .collect();

    Array::from_shape_vec((rows, cols), data)
        .unwrap()
        .reversed_axes()
}

#[aoc(day25, part1)]
fn safe_iteration(start: &Cucumbers) -> usize {
    let mut iteration = 0;
    let mut prev = (*start).clone();
    loop {
        let next = step(&prev);
        iteration += 1;

        if next == prev {
            return iteration;
        }

        prev = next;
    }
}

#[test]
fn test_safe_iteration() {
    assert_eq!(
        safe_iteration(&generate(include_str!("day25_example.txt"))),
        58
    );
}
