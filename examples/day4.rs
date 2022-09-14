#[derive(Debug, PartialEq, Clone, Copy)]
enum Value {
    Matched(u32),
    Unmatched(u32),
}

#[derive(Default)]
struct Board {
    rows: Vec<Vec<Value>>,
}

impl Board {
    fn mark(&mut self, num: u32) {
        for r in 0..self.rows.len() {
            for c in 0..self.rows[r].len() {
                if let Value::Unmatched(v) = self.rows[r][c] {
                    if v == num {
                        self.rows[r][c] = Value::Matched(num);
                    }
                }
            }
        }
    }

    fn row_matched(&self) -> bool {
        for row in &self.rows {
            if row.iter().all(|x| matches!(x, Value::Matched(_))) {
                return true;
            }
        }
        false
    }

    fn column_matched(&self) -> bool {
        let columns = self.rows[0].len();
        for c in 0..columns {
            if self
                .rows
                .iter()
                .map(|row| row[c])
                .all(|x| matches!(x, Value::Matched(_)))
            {
                return true;
            }
        }
        false
    }

    fn winning(&self) -> bool {
        self.row_matched() || self.column_matched()
    }

    fn score(&self) -> u32 {
        self.rows
            .iter()
            .flatten()
            .map(|v| match v {
                Value::Unmatched(v) => *v,
                _ => 0 as u32,
            })
            .sum()
    }
}

impl From<Vec<String>> for Board {
    fn from(lines: Vec<String>) -> Self {
        let mut board = Board::default();
        for l in lines {
            board.rows.push(
                l.split_whitespace()
                    .map(|x| Value::Unmatched(x.parse::<u32>().unwrap()))
                    .collect(),
            );
        }
        board
    }
}

#[test]
fn test_board() {
    let mut board = Board::from(vec!["1 2 3".to_string()]);
    assert_eq!(board.rows.len(), 1);
    assert_eq!(board.rows[0][0], Value::Unmatched(1));
    assert_eq!(board.rows[0][1], Value::Unmatched(2));
    assert_eq!(board.rows[0][2], Value::Unmatched(3));
    assert_eq!(board.winning(), false);

    board.mark(2);
    assert_eq!(board.rows[0][0], Value::Unmatched(1));
    assert_eq!(board.rows[0][1], Value::Matched(2));
    assert_eq!(board.rows[0][2], Value::Unmatched(3));
    assert_eq!(board.winning(), true);
    assert_eq!(board.score(), 4);
}

struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}
use itertools::Itertools;

impl From<Vec<String>> for Game {
    fn from(lines: Vec<String>) -> Self {
        let mut line = lines.iter();
        let numbers = line
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut boards = vec![];
        for (key, group) in &line.group_by(|s| !s.is_empty()) {
            if key {
                boards.push(Board::from(
                    group.map(|x| x.to_string()).collect::<Vec<String>>(),
                ))
            }
        }

        Game { numbers, boards }
    }
}

use std::collections::HashSet;

impl Game {
    fn winning_score(&mut self) -> u32 {
        for number in &self.numbers {
            for board in &mut self.boards {
                board.mark(*number);
                if board.winning() {
                    return *number * board.score();
                }
            }
        }
        0
    }

    fn losing_score(&mut self) -> u32 {
        let mut winners = HashSet::new();
        let count = self.boards.len();
        for number in &self.numbers {
            for (b, board) in &mut self.boards.iter_mut().enumerate() {
                board.mark(*number);
                if board.winning() {
                    winners.insert(b);
                    if winners.len() == count {
                        return *number * board.score();
                    }
                }
            }
        }
        0
    }
}

#[test]
fn test_bingo() {
    let example = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    let mut game = Game::from(
        example
            .to_string()
            .split('\n')
            .skip(1)
            .map(|x| x.to_string())
            .collect::<Vec<_>>(),
    );

    assert_eq!(game.boards.len(), 3);
    assert_eq!(game.boards[0].rows[0][0], Value::Unmatched(22));
    assert_eq!(game.boards[2].rows[4][4], Value::Unmatched(7));
    assert_eq!(game.winning_score(), 4512);
    assert_eq!(game.losing_score(), 1924);
}

use std::io;

fn main() {
    //  Curious how much overhead is in the enum type for our simple u32 value:
    // dbg!(std::mem::size_of::<Value>());
    // dbg!(std::mem::size_of::<&Value>());
    // on a 64-bit system both are 8 bytes

    let mut bingo = Game::from(io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    println!("{}", bingo.winning_score());
    println!("{}", bingo.losing_score());
}
