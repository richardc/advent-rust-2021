#[derive(Default)]
struct Board {
    rows: Vec<Vec<u32>>,
    marked: Vec<u32>,
}

impl Board {
    fn mark(&self, _num: u32) {}

    fn winning(self) -> bool {
        false
    }
}

struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

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
        let mut board = Board::default();
        for l in line {
            if l.is_empty() {
                boards.push(board);
                board = Board::default();
            } else {
                board.rows.push(
                    l.split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect(),
                );
            }
        }
        boards.push(board);

        Game {
            numbers: numbers,
            boards: boards,
        }
    }
}

impl Game {
    fn winning_score(self) -> i32 {
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

    let game = Game::from(
        example
            .to_string()
            .split('\n')
            .skip(1)
            .map(|x| x.to_string())
            .collect::<Vec<_>>(),
    );
    assert_eq!(game.winning_score(), 4512)
}

use std::io;

fn main() {
    let bingo = Game::from(io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>());
    println!("{}", bingo.winning_score());
}
