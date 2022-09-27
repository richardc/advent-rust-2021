use std::cmp::{max, min};

use itertools::Itertools;

#[derive(Clone)]
struct Game {
    positions: [i32; 2],
    scores: [i32; 2],
    die: i32,
    rolls: i32,
}

impl Game {
    fn new(player1: i32, player2: i32) -> Self {
        Self {
            positions: [player1, player2],
            scores: [0, 0],
            die: 0,
            rolls: 0,
        }
    }
}

impl Game {
    fn roll(&mut self) -> i32 {
        self.rolls += 1;
        self.die += 1;
        self.die %= 100;
        if self.die == 0 {
            100
        } else {
            self.die
        }
    }

    fn turn(&mut self, player: usize) {
        let rolled = self.roll() + self.roll() + self.roll();
        self.positions[player] += rolled;
        self.positions[player] %= 10;
        self.scores[player] += self.positions[player] + 1;
    }

    fn game_won(&self) -> bool {
        self.scores[0] >= 1000 || self.scores[1] >= 1000
    }
}

#[test]
fn test_die() {
    let mut game = Game::new(0, 0);

    assert_eq!(game.roll(), 1);
    assert_eq!(game.roll(), 2);
    game.die = 99;
    assert_eq!(game.roll(), 100);
    assert_eq!(game.roll(), 1);
    assert_eq!(game.rolls, 4);
}

#[test]
fn test_game() {
    let mut game = Game::new(3, 7);
    game.turn(0);
    assert_eq!(game.positions, [9, 7]);
    assert_eq!(game.scores, [10, 0]);
    assert_eq!(game.rolls, 3);

    game.turn(1);
    assert_eq!(game.positions, [9, 2]);
    assert_eq!(game.scores, [10, 3]);
    assert_eq!(game.rolls, 6);

    game.play_till_won();
    assert_eq!(game.positions, [9, 2]);
    assert_eq!(game.scores, [1000, 745]);
    assert_eq!(game.rolls, 993);
}

#[aoc_generator(day21)]
fn generate(input: &str) -> Game {
    let positions = input
        .lines()
        .map(|l| l.split(' ').last().unwrap().parse::<i32>().unwrap())
        .collect_vec();
    Game::new(positions[0] - 1, positions[1] - 1)
}

#[test]
fn test_generate() {
    let game = generate(include_str!("day21_example.txt"));
    assert_eq!(game.positions, [3, 7]);
}

impl Game {
    fn play_till_won(&mut self) {
        let mut player = 0;
        while !self.game_won() {
            self.turn(player);
            player += 1;
            player %= 2;
        }
    }
}

#[aoc(day21, part1)]
fn losing_factor(game: &Game) -> i32 {
    let mut game = (*game).clone();
    game.play_till_won();
    min(game.scores[0], game.scores[1]) * game.rolls
}

fn make_move(
    positions: [i32; 2],
    scores: [i32; 2],
    roll: i32,
    to_play: usize,
) -> ([i32; 2], [i32; 2]) {
    let mut pos = positions;
    let mut score = scores;

    pos[to_play] += roll;
    pos[to_play] %= 10;

    score[to_play] += pos[to_play] + 1;

    (pos, score)
}

use memoize::memoize;
#[memoize]
fn wins_in_space(positions: [i32; 2], scores: [i32; 2], to_play: usize) -> (u64, u64) {
    // Base case, this is a won game for player 1
    if scores[0] >= 21 {
        return (1, 0);
    }

    // player 2 win
    if scores[1] >= 21 {
        return (0, 1);
    }

    let mut winners = (0, 0);
    // When we roll 3 Dirac dice, we get one of the following total rolls at each
    // given frequency, so we recurse just on the roll totals and current game state.
    for [roll, frequency] in [[3, 1], [4, 3], [5, 6], [6, 7], [7, 6], [8, 3], [9, 1]] {
        let (positions, scores) = make_move(positions, scores, roll, to_play);
        let wins = wins_in_space(positions, scores, 1 - to_play);

        // Now we know who won in that branch of the game, multiply it back up
        // by frequency to count the number of universes that happened in.
        winners.0 += wins.0 * frequency as u64;
        winners.1 += wins.1 * frequency as u64;
    }
    winners
}

#[aoc(day21, part2)]
fn winning_universes(game: &Game) -> u64 {
    let (p1, p2) = wins_in_space(game.positions, game.scores, 0);

    max(p1, p2)
}

#[test]
fn test_winning_universes() {
    assert_eq!(winning_universes(&Game::new(3, 7)), 444356092776315);
}
