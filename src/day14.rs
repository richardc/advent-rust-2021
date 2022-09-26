use itertools::Itertools;
use std::{collections::HashMap, iter::FromIterator};

struct Puzzle {
    start: String,
    rules: HashMap<(char, char), char>,
}

impl FromIterator<String> for Puzzle {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut lines = iter.into_iter();
        let start = lines.next().unwrap();
        Puzzle {
            start,
            rules: HashMap::from_iter(lines.skip(1).map(|x| {
                let (a, b) = x.split_once(" -> ").unwrap();
                #[allow(clippy::iter_nth_zero)]
                // we want 0 and 1, so nth(0) -> next() makes the nth(1) look funky
                (
                    (a.chars().nth(0).unwrap(), a.chars().nth(1).unwrap()),
                    b.chars().nth(0).unwrap(),
                )
            })),
        }
    }
}

impl Puzzle {
    fn step_simple(&self, s: String) -> String {
        let mut result: Vec<String> = vec![];
        s.chars().tuple_windows::<(_, _)>().for_each(|(a, b)| {
            result.push(a.to_string());
            result.push(self.rules.get(&(a, b)).unwrap().to_string());
        });
        result.push(s.chars().last().unwrap().to_string());
        result.join("")
    }

    fn steps(&self, step: usize) -> String {
        let mut result = self.start.clone();
        for _ in 0..step {
            result = self.step_simple(result);
        }
        result
    }

    fn step_1(&self) -> usize {
        let polymer = self.steps(10);
        let counts = polymer.chars().counts();
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }

    fn step_pairs(&self, counts: HashMap<(char, char), usize>) -> HashMap<(char, char), usize> {
        let mut new_pairs = HashMap::new();
        for (pair, count) in counts {
            let &insert = self.rules.get(&pair).unwrap();
            new_pairs
                .entry((pair.0, insert))
                .and_modify(|v| *v += count)
                .or_insert(count);
            new_pairs
                .entry((insert, pair.1))
                .and_modify(|v| *v += count)
                .or_insert(count);
        }
        new_pairs
    }

    fn step_pairwise(&self, count: usize) -> HashMap<(char, char), usize> {
        let mut counts =
            HashMap::from_iter(self.start.chars().tuple_windows::<(_, _)>().map(|t| (t, 1)));
        for _ in 0..count {
            counts = self.step_pairs(counts);
        }
        counts
    }

    fn step_2(&self) -> usize {
        let pair_counts = self.step_pairwise(40);
        let mut char_counts = HashMap::new();
        pair_counts.iter().for_each(|((a, _), &count)| {
            char_counts
                .entry(a)
                .and_modify(|v| *v += count)
                .or_insert(count);
        });

        let last_char = self.start.chars().last().unwrap();
        char_counts
            .entry(&last_char)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
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
    // Naiive polymer code blows up memory real bad after 40 steps
    assert_eq!(puzzle.step_2(), 2188189693529);
}

#[aoc_generator(day14)]
fn generate(input: &str) -> Puzzle {
    Puzzle::from_iter(input.lines().map(|x| x.to_string()))
}

#[aoc(day14, part1)]
fn step1(p: &Puzzle) -> usize {
    p.step_1()
}

#[aoc(day14, part2)]
fn step2(p: &Puzzle) -> usize {
    p.step_2()
}
