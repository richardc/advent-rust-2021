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

use std::io;

fn main() {
    let statuses: Vec<_> = io::stdin().lines().map(|s| s.unwrap()).collect();
    println!("{}", battery(statuses));
}
