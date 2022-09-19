use std::{collections::HashMap, io, iter::FromIterator};

use itertools::Itertools;

struct Puzzle {
    start: String,
    rules: HashMap<String, String>,
}

impl FromIterator<String> for Puzzle {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut lines = iter.into_iter();
        let start = lines.next().unwrap();
        Puzzle {
            start: start,
            rules: HashMap::from_iter(lines.skip(1).map(|x| {
                let (a, b) = x.split_once(" -> ").unwrap();
                (a.to_string(), b.to_string())
            })),
        }
    }
}

impl Puzzle {
    fn step(&self, s: String) -> String {
        let mut result: Vec<String> = vec![];
        s.chars().tuple_windows::<(_, _)>().for_each(|(a, b)| {
            result.push(a.to_string());
            let key = String::from_iter([a, b]);
            result.push(self.rules.get(&key).unwrap().to_string());
        });
        result.push(s.chars().last().unwrap().to_string());
        result.join("")
    }

    fn steps(&self, step: usize) -> String {
        let mut result = self.start.clone();
        for _ in 0..step {
            result = self.step(result);
        }
        result
    }

    fn step_1(&self) -> usize {
        let polymer = self.steps(10);
        let counts = polymer.chars().counts();
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
}

#[test]
fn test_expand() {
    let example = r#"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    let lines = example.trim().split('\n').map(|s| s.to_string());
    let puzzle = Puzzle::from_iter(lines);
    assert_eq!(puzzle.rules.len(), 16);
    assert_eq!(puzzle.steps(1), "NCNBCHB");
    assert_eq!(puzzle.steps(2), "NBCCNBBBCBHCB");
    assert_eq!(puzzle.steps(3), "NBBBCNCCNBBNBNBBCHBHHBCHB");
    assert_eq!(
        puzzle.steps(4),
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
    );

    assert_eq!(puzzle.step_1(), 1588);
}

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap());
    let puzzle = Puzzle::from_iter(lines);
    println!("{}", puzzle.step_1());
}
