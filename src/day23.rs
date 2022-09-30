use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

#[cfg(test)]
use pretty_assertions::assert_eq;

type Cost = u32;

#[derive(PartialEq, Eq, Clone, Default)]
struct Game {
    rows: u8,
    state: State,
    moves: Vec<Move>,
}

impl Game {
    fn new(pods: Vec<char>) -> Self {
        let columns = ['a', 'b', 'c', 'd'];

        let rows = pods.len() as u8 / 4;
        let moves = all_moves(rows);
        let mut state = State::default();

        for (i, pod) in pods.iter().enumerate() {
            let cell = Cell {
                column: columns[i % 4],
                index: i as u8 / 4,
            };

            state.cells.insert(cell, Pod::new(*pod));
        }
        Game { rows, state, moves }
    }
}

fn pod_chars(input: &str) -> Vec<char> {
    input
        .chars()
        .into_iter()
        .filter(|c| matches!(c, 'A' | 'B' | 'C' | 'D'))
        .collect()
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        Game::new(pod_chars(input))
    }
}

impl Game {
    fn solved(&self, state: &State) -> bool {
        let columns = ['a', 'b', 'c', 'd'];
        (0..2).cartesian_product(columns).all(|(index, column)| {
            if let Some(pod) = state.cells.get(&Cell { column, index }) {
                pod.kind == column.to_ascii_uppercase()
            } else {
                false
            }
        })
    }
}

#[test]
fn test_game_solved_state() {
    let game = Game::from("ABDC ABCD");
    assert_eq!(game.state, "a0=A,a1=A,b0=B,b1=B,c0=D,c1=C,d0=C,d1=D");
    assert!(!game.solved(&game.state));

    let game = Game::from("ABCD ABCD");
    assert_eq!(game.state, "a0=A,a1=A,b0=B,b1=B,c0=C,c1=C,d0=D,d1=D");
    assert!(game.solved(&game.state));
}

impl Game {
    fn moves_for_cell(&self, state: &State, cell: &Cell) -> Vec<(State, Cost)> {
        let pod = state.cells.get(cell).unwrap();
        if cell.column != 'h' {
            // In a home room
            if pod.kind.to_ascii_lowercase() == cell.column {
                // In our home room
                if (cell.index..self.rows).all(|index| {
                    if let Some(occupant) = state.cells.get(&Cell { index, ..*cell }) {
                        pod.kind == occupant.kind
                    } else {
                        false
                    }
                }) {
                    // Every cell below us is home, so are we, no move needed
                    return vec![];
                }
            }

            // We need to try moving into the hallway
            return self
                .moves
                .iter()
                .filter(|m| m.start == *cell)
                .filter(|m| !m.blocked.iter().any(|cell| state.cells.contains_key(cell)))
                .map(|m| {
                    let mut state = state.clone();
                    state.cells.remove(cell);
                    state.cells.insert(m.end, pod.clone());
                    (state, m.moves * pod.cost())
                })
                .collect();
        } else {
            // We're in the hallway.  We can go home if our homeroom is empty or has no strangers
            let column = pod.kind.to_ascii_lowercase();
            if !(0..self.rows).all(|index| {
                if let Some(occupant) = state.cells.get(&Cell { column, index }) {
                    pod.kind == occupant.kind
                } else {
                    true
                }
            }) {
                return vec![];
            }

            let lowest_spot = (0..self.rows as u8)
                .rev()
                .filter(|&index| !state.cells.contains_key(&Cell { column, index }))
                .next()
                .unwrap();

            let home_cell = Cell {
                column,
                index: lowest_spot,
            };

            return self
                .moves
                .iter()
                // inverse path
                .filter(|m| m.start == home_cell && m.end == *cell)
                // not blocked
                .filter(|m| !m.blocked.iter().any(|cell| state.cells.contains_key(cell)))
                .map(|m| {
                    let mut state = state.clone();
                    state.cells.remove(cell);
                    state.cells.insert(m.start, pod.clone());
                    (state, m.moves * pod.cost())
                })
                .collect();
        }
    }
}

#[test]
fn test_moves_for_cell() {
    let game = Game::from("BACD ABCD");
    assert_eq!(
        game.state,
        State::from("a0=B,a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D")
    );
    let moves = game.moves_for_cell(&game.state, &Cell::from("a0"));
    assert_eq!(moves.len(), 7);
    assert_eq!(
        moves,
        vec![
            (State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h0=B"), 30),
            (State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h1=B"), 20),
            (State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h3=B"), 20),
            (State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h5=B"), 40),
            (State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h7=B"), 60),
            (State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h9=B"), 80),
            (State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h10=B"), 90),
        ],
        "Starting moves for a0"
    );

    let moves = game.moves_for_cell(
        &State::from("a1=A,b0=A,b1=B,c0=C,c1=C,d0=D,d1=D,h0=B"),
        &Cell::from("h0"),
    );
    assert_eq!(moves, vec![], "No legal move for h0");

    let moves = game.moves_for_cell(
        &State::from("a0=A,a1=A,b1=B,c0=C,c1=C,d0=D,d1=D,h0=B"),
        &Cell::from("h0"),
    );
    assert_eq!(
        moves,
        vec![(State::from("a0=A,a1=A,b0=B,b1=B,c0=C,c1=C,d0=D,d1=D"), 50)],
        "Winning move, h0 -> a0"
    );
}

impl Game {
    fn legal_moves(&self, state: &State) -> Vec<(State, Cost)> {
        state
            .cells
            .keys()
            .flat_map(|cell| self.moves_for_cell(state, cell))
            .collect()
    }
}

// Walk over the game states
#[derive(PartialEq, Eq, Clone, Default)]
struct Walk {
    state: State,
    cost: Cost,
}

impl Ord for Walk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Walk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// Dijkstra but using a HashMap rather than an array to track Game States
fn cheapest_path(game: &Game) -> Cost {
    let mut costs: HashMap<State, Cost> = HashMap::new();
    let mut queue = BinaryHeap::new();

    costs.insert(game.state.to_owned(), 0);
    queue.push(Walk {
        state: game.state.to_owned(),
        cost: 0,
    });

    while let Some(Walk { state, cost }) = queue.pop() {
        if game.solved(&state) {
            return cost;
        }

        if cost > *costs.entry(state.to_owned()).or_insert(Cost::MAX) {
            // We've found a cheaper way to reach this state, so we're not interested in the paths out
            continue;
        }

        for (state, move_cost) in game.legal_moves(&state) {
            let next = Walk {
                state,
                cost: cost + move_cost,
            };

            // Is this currently the cheapest path to a known state?
            if next.cost < *costs.entry(next.state.to_owned()).or_insert(Cost::MAX) {
                // Yes, queue it up
                costs.insert(next.state.to_owned(), next.cost);
                queue.push(next);
            }
        }
    }
    Cost::MAX
}

#[ignore] // This is slow
#[test]
fn test_cheapest_path() {
    assert_eq!(
        cheapest_path(&generate(include_str!("day23_example.txt"))),
        12521,
    );
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Default)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
struct Move {
    start: Cell,
    end: Cell,
    blocked: Vec<Cell>,
    moves: Cost,
}

impl Move {
    fn new(start: &str, end: &str, blocked: Vec<&str>, moves: Cost) -> Self {
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

                Move {
                    start,
                    blocked,
                    moves: base.moves + index as Cost,
                    ..*base
                }
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

#[derive(PartialEq, Eq, Clone, Default)]
struct State {
    cells: HashMap<Cell, Pod>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .sorted_by_key(|(&cell, _)| cell)
                .map(|(cell, pod)| format!("{}={}", cell, pod))
                .join(",")
        )
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{}", self).hash(state)
    }
}

impl PartialEq<&str> for State {
    fn eq(&self, other: &&str) -> bool {
        format!("{}", self) == *other
    }
}

impl From<&str> for State {
    fn from(input: &str) -> Self {
        Self {
            cells: HashMap::from_iter(input.split(',').map(|assignment| {
                let (cell, pod) = assignment.split_once('=').unwrap();
                (Cell::from(cell), Pod::from(pod))
            })),
        }
    }
}

#[test]
fn test_game_state() {
    assert_eq!(
        generate(include_str!("day23_example.txt")).state,
        "a0=B,a1=A,b0=C,b1=D,c0=B,c1=C,d0=D,d1=A"
    );
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Pod {
    kind: char,
}

impl Pod {
    fn new(kind: char) -> Self {
        Self { kind }
    }

    fn cost(&self) -> Cost {
        match self.kind {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Pod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl From<&str> for Pod {
    fn from(input: &str) -> Self {
        Pod::new(input.chars().next().unwrap())
    }
}

#[aoc_generator(day23, part1)]
fn generate(input: &str) -> Game {
    Game::from(input)
}

#[aoc_generator(day23, part2)]
fn generate_spliced(input: &str) -> Game {
    let pods = pod_chars(input);
    let (left, right) = pods.split_at(4);
    Game::from(
        format!(
            "{} DCBA DBAC {}",
            String::from_iter(left),
            String::from_iter(right)
        )
        .as_str(),
    )
}

#[aoc(day23, part1)]
#[aoc(day23, part2)]
fn part1(game: &Game) -> Cost {
    cheapest_path(game)
}
