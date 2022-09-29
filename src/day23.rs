use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
struct State {
    cells: [u8; 1],
}

#[derive(PartialEq, Eq, Clone, Default)]
struct Game {
    state: State,
    cost: u32,
    solved: bool,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    fn legal_moves(&self) -> Vec<Game> {
        vec![Game {
            state: State { cells: [1] },
            cost: 42,
            solved: true,
            ..self.clone()
        }]
    }
}

#[aoc_generator(day23)]
fn generate(_input: &str) -> Game {
    Game::default()
}

// Dijkstra but using a HashMap rather than an array to track Game States
#[aoc(day23, part1)]
fn cheapest_path(input: &Game) -> u32 {
    let start = input.clone();
    let mut costs: HashMap<State, u32> = HashMap::new();
    let mut queue = BinaryHeap::new();

    costs.insert(start.state, 0);
    queue.push(start);

    while let Some(game) = queue.pop() {
        if game.solved {
            return game.cost;
        }

        if game.cost > *costs.entry(game.state).or_insert(u32::MAX) {
            // We've found a cheaper way to reach this state, skip
            continue;
        }

        for next in game.legal_moves() {
            // is this a cheaper way to a known state?
            if next.cost < *costs.entry(next.state).or_insert(u32::MAX) {
                costs.insert(next.state, next.cost);
                queue.push(next);
            }
        }
    }
    u32::MAX
}

#[test]
fn test_cheapest_path() {
    assert_eq!(cheapest_path(&Game::default()), 42);
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Cell {
    column: char,
    index: u8,
}

impl From<&str> for Cell {
    fn from(input: &str) -> Self {
        let mut chars = input.chars();
        let column = chars.next().unwrap();
        let index = chars.as_str().parse().unwrap();
        Self { column, index }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.column, self.index)
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.column, self.index)
    }
}

impl PartialEq<&str> for Cell {
    fn eq(&self, other: &&str) -> bool {
        format!("{}", self) == *other
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Move {
    start: Cell,
    end: Cell,
    blocked: Vec<Cell>,
    moves: u8,
}

impl Move {
    fn new(start: &str, end: &str, blocked: Vec<&str>, moves: u8) -> Self {
        Self {
            start: Cell::from(start),
            end: Cell::from(end),
            blocked: blocked.iter().map(|&b| Cell::from(b)).collect(),
            moves,
        }
    }
}

// #############
// #...........#   hallway0..hallway11
// ###B#C#B#D###   {col}0
//   #A#D#C#A#     {col}1   we can derive this as
//   #########
//    a

fn base_moves() -> Vec<Move> {
    vec![
        // Column A
        Move::new("a0", "h0", vec!["h1"], 3),
        Move::new("a0", "h1", vec![], 2),
        Move::new("a0", "h3", vec![], 2),
        Move::new("a0", "h5", vec!["h3"], 4),
        Move::new("a0", "h7", vec!["h3", "h5"], 6),
        Move::new("a0", "h9", vec!["h3", "h5", "h7"], 8),
        Move::new("a0", "h10", vec!["h3", "h5", "h7", "h9"], 9),
        // Column B
        Move::new("b0", "h0", vec!["h1", "h3"], 5),
        Move::new("b0", "h1", vec!["h3"], 4),
        Move::new("b0", "h3", vec![], 2),
        Move::new("b0", "h5", vec![], 2),
        Move::new("b0", "h7", vec!["h5"], 4),
        Move::new("b0", "h9", vec!["h5", "h7"], 6),
        Move::new("b0", "h10", vec!["h5", "h7", "h9"], 7),
        // Column C
        Move::new("c0", "h0", vec!["h1", "h3", "h5"], 7),
        Move::new("c0", "h1", vec!["h3", "h5"], 6),
        Move::new("c0", "h3", vec!["h5"], 4),
        Move::new("c0", "h5", vec![], 2),
        Move::new("c0", "h7", vec![], 2),
        Move::new("c0", "h9", vec!["h7"], 4),
        Move::new("c0", "h10", vec!["h7", "h9"], 5),
        // Column D
        Move::new("d0", "h0", vec!["h1", "h3", "h5", "h7"], 9),
        Move::new("d0", "h1", vec!["h3", "h5", "h7"], 8),
        Move::new("d0", "h3", vec!["h5", "h7"], 6),
        Move::new("d0", "h5", vec!["h7"], 4),
        Move::new("d0", "h7", vec![], 2),
        Move::new("d0", "h9", vec![], 2),
        Move::new("d0", "h10", vec!["h9"], 3),
    ]
}

fn all_moves(depth: u8) -> Vec<Move> {
    // generates a1 -> * from a0 -> *, adding one move and the blocked cells
    base_moves()
        .iter()
        .flat_map(|base| {
            (0..depth).map(|index| {
                let start = Cell {
                    index,
                    ..base.start
                };
                // a1 is inhibited by a0 plus rest of the path, a2 by a1 and a0, etc
                let mut blocked = base.blocked.clone();
                blocked.extend((0..index).map(|x| Cell {
                    index: x,
                    ..base.start
                }));
                let extra = Move {
                    start,
                    blocked,
                    moves: base.moves + index,
                    ..*base
                };
                extra
            })
        })
        .collect()
}

#[test]
fn test_all_moves() {
    let moves: HashMap<String, Move> = HashMap::from_iter(
        all_moves(4)
            .into_iter()
            .map(|m| (format!("{} -> {}", m.start, m.end), m)),
    );
    // From the hand-built states
    assert_eq!(moves["a0 -> h0"].moves, 3);
    assert_eq!(moves["a0 -> h0"].blocked, vec!["h1"]);
    // From the derived states, it costs one more and must step up
    assert_eq!(moves["a1 -> h0"].moves, 4);
    assert_eq!(moves["a1 -> h0"].blocked, vec!["h1", "a0"]);

    // From the derived states, it costs one more and must step up
    assert_eq!(moves["a3 -> h0"].moves, 6);
    assert_eq!(moves["a3 -> h0"].blocked, vec!["h1", "a0", "a1", "a2"]);
}
