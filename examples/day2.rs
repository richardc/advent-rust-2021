enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    Err,
}

impl From<String> for Command {
    fn from(command: String) -> Self {
        if let Some((verb, distance)) = command.split_once(" ") {
            let d = distance.parse::<i32>().unwrap();
            match verb {
                "forward" => Command::Forward(d),
                "down" => Command::Down(d),
                "up" => Command::Up(d),
                _ => Command::Err,
            }
        } else {
            Command::Err
        }
    }
}

fn position(commands: Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        match Command::from(command) {
            Command::Forward(x) => horizontal += x,
            Command::Up(d) => depth -= d,
            Command::Down(d) => depth += d,
            Command::Err => {}
        }
    }

    horizontal * depth
}

#[test]
fn test_position() {
    assert_eq!(
        position(
            vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        150
    );
}

fn position_aimed(commands: Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match Command::from(command) {
            Command::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Command::Up(d) => aim -= d,
            Command::Down(d) => aim += d,
            Command::Err => {}
        }
    }

    horizontal * depth
}

#[test]
fn test_position_aimed() {
    assert_eq!(
        position_aimed(
            vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        900
    );
}

use std::io;

fn main() {
    let directions: Vec<_> = io::stdin().lines().map(|s| s.unwrap()).collect();
    println!("{}", position_aimed(directions));
}
