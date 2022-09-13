type Pos = i32;

fn cost_move(crabs: &[Pos], position: Pos, fuel: fn(Pos) -> Pos) -> Pos {
    crabs
        .iter()
        .map(|&x| {
            if x > position {
                x - position
            } else {
                position - x
            }
        })
        .map(fuel)
        .sum()
}

fn cost(crabs: &[Pos], position: Pos) -> Pos {
    cost_move(crabs, position, |x| x)
}

#[test]
fn test_cost() {
    let example = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(cost(&example, 1), 41);
    assert_eq!(cost(&example, 2), 37);
    assert_eq!(cost(&example, 3), 39);
    assert_eq!(cost(&example, 10), 71);
}

fn cheapest(crabs: &[Pos], cost: fn(&[Pos], Pos) -> Pos) -> Pos {
    let &start = crabs.iter().min().unwrap();
    let &end = crabs.iter().max().unwrap();

    (start..end)
        .into_iter()
        .map(|p| cost(crabs, p))
        .min()
        .unwrap()
}

#[test]
fn test_cheapest() {
    let example = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(cheapest(&example, cost), 37);
}

use memoize::memoize;

// It's fib, so memo it
#[memoize]
fn fuel_cost(distance: Pos) -> Pos {
    (1..=distance).into_iter().fold(0, |d, s| d + s)
}

#[test]
fn test_fuel_cost() {
    assert_eq!(fuel_cost(1), 1);
    assert_eq!(fuel_cost(2), 1 + 2);
    assert_eq!(fuel_cost(3), 1 + 2 + 3);
}

fn real_cost(crabs: &[Pos], position: Pos) -> Pos {
    cost_move(crabs, position, fuel_cost)
}

#[test]
fn test_real_cost() {
    let example = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(real_cost(&example, 2), 206);
    assert_eq!(real_cost(&example, 5), 168);
}

use std::io;

fn main() {
    let str = io::stdin().lines().next().unwrap().unwrap();
    let data = str
        .trim()
        .split(',')
        .map(|s| s.parse::<Pos>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", cheapest(&data, cost));
    println!("{}", cheapest(&data, real_cost));
}
