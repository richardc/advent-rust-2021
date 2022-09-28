use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Clone)]
struct Move {}

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
struct State {
    cells: [u8; 1],
}

#[derive(PartialEq, Eq, Clone, Default)]
struct Game {
    state: State,
    moves: Vec<Move>,
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
