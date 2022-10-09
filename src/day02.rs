enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    Err,
}

#[aoc_generator(day2)]
fn generate(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| Command::from(l.to_string()))
        .collect()
}

impl From<String> for Command {
    fn from(command: String) -> Self {
        if let Some((verb, distance)) = command.split_once(' ') {
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

#[aoc(day2, part1)]
fn position(commands: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command::Forward(x) => horizontal += x,
            Command::Up(d) => depth -= d,
            Command::Down(d) => depth += d,
            Command::Err => {}
        }
    }

    horizontal * depth
}

#[cfg(test)]
const EXAMPLE: &str = include_str!("day2_example1.txt");

#[test]
fn test_position() {
    assert_eq!(position(&generate(EXAMPLE)), 150);
}

#[aoc(day2, part2)]
fn position_aimed(commands: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
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
    assert_eq!(position_aimed(&generate(EXAMPLE)), 900);
}
