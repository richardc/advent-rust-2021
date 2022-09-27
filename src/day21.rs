use std::cmp::min;

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
