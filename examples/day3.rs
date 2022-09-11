fn battery(lines: Vec<String>) -> i32 {
    let mut counts: Vec<_> = lines.first().unwrap().chars().map(|_| 0).collect();
    let half = lines.len() / 2;

    for x in lines {
        for (i, c) in x.char_indices() {
            if c == '1' {
                counts[i] += 1
            }
        }
    }
    let gamma: String = counts
        .iter()
        .map(|c| if c > &half { '1' } else { '0' })
        .collect();

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();

    let epsilon: String = counts
        .iter()
        .map(|c| if c > &half { '0' } else { '1' })
        .collect();

    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

#[test]
fn test_battery() {
    assert_eq!(
        battery(
            vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        198
    );
}

fn bitcount(lines: &Vec<String>, column: usize) -> i32 {
    lines
        .iter()
        .map(|s| match s.chars().nth(column) {
            Some(c) if c == '1' => 1,
            _ => 0,
        })
        .sum()
}

fn bitselect(lines: Vec<String>, criteria: fn(i32, i32) -> char) -> i32 {
    let bits = lines.first().unwrap().len();

    let mut set = lines;
    for position in 0..=bits {
        let ones = bitcount(&set, position);
        let zeroes = set.len() as i32 - ones;
        let keep = criteria(ones, zeroes);

        set = set
            .iter()
            .filter(|l| l.chars().nth(position) == Some(keep))
            .map(|s| s.to_string())
            .collect();

        if set.len() == 1 {
            return i32::from_str_radix(set.first().unwrap(), 2).unwrap();
        }
    }
    0
}

fn oxygen(lines: Vec<String>) -> i32 {
    bitselect(lines, |ones, zeroes| if ones >= zeroes { '1' } else { '0' })
}

#[test]
fn test_oxygen() {
    assert_eq!(
        oxygen(
            vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        23
    );
}

fn co2(lines: Vec<String>) -> i32 {
    bitselect(lines, |ones, zeroes| if ones >= zeroes { '0' } else { '1' })
}

#[test]
fn test_co2() {
    assert_eq!(
        co2(vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()),
        10
    );
}

fn lifesupport(lines: Vec<String>) -> i32 {
    oxygen(lines.clone()) * co2(lines)
}

#[test]
fn test_lifesupport() {
    assert_eq!(
        lifesupport(
            vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        230
    );
}

use std::io;

fn main() {
    let statuses: Vec<_> = io::stdin().lines().map(|s| s.unwrap()).collect();
    println!("{}", lifesupport(statuses));
}
