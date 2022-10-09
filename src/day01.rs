#[aoc_generator(day1)]
fn generate(input: &str) -> Vec<u32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1,part1)]
fn increases(samples: &[u32]) -> u32 {
    if samples.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut prev = samples[0];

    for &sample in samples {
        if sample > prev {
            count += 1
        }
        prev = sample
    }
    count
}

#[test]
fn test_increases() {
    assert_eq!(
        increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}

use itertools::Itertools;

#[aoc(day1,part2)]
fn sliding_increases(samples: &[u32]) -> u32 {
    increases(
        &samples
            .iter()
            .tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| a + b + c)
            .collect::<Vec<_>>(),
    )
}

#[test]
fn test_sliding_increases() {
    assert_eq!(
        sliding_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5
    );
}
