type Count = u64;
type Population = [Count; 9];

fn parse(str: String) -> Population {
    let mut pop = [0; 9];
    for age in str.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        pop[age] += 1;
    }

    pop
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(String::from("3,4,3,1,2")),
        [0, 1, 1, 2, 1, 0, 0, 0, 0]
    )
}

fn step(pop: Population) -> Population {
    let mut next = pop;
    next.rotate_left(1);
    next[6] += pop[0];
    next
}

#[test]
fn test_step() {
    assert_eq!(
        step([1, 0, 0, 0, 0, 0, 0, 0, 0]),
        [0, 0, 0, 0, 0, 0, 1, 0, 1]
    );
}

fn run(pop: Population, steps: u32) -> Count {
    let mut last = pop;
    for _ in 0..steps {
        last = step(last)
    }
    last.iter().sum()
}

#[test]
fn test_run() {
    assert_eq!(run([1, 0, 0, 0, 0, 0, 0, 0, 0], 0), 1);
    assert_eq!(run([1, 0, 0, 0, 0, 0, 0, 0, 0], 1), 2);
}

#[test]
fn test_example() {
    assert_eq!(run(parse(String::from("3,4,3,1,2")), 80), 5934);
    assert_eq!(run(parse(String::from("3,4,3,1,2")), 256), 26984457539);
}

use std::io;

fn main() {
    let start = io::stdin().lines().next().unwrap().unwrap();
    println!("{}", run(parse(start.clone()), 80));
    println!("{}", run(parse(start), 256));
}
