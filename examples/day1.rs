use std::io;

fn increases(samples: Vec<u32>) -> u32 {
    if samples.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut prev = samples[0];

    for sample in samples {
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
        increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}

use itertools::Itertools;

fn sliding_increases(samples: Vec<u32>) -> u32 {
    increases(
        samples
            .iter()
            .tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| a + b + c)
            .collect(),
    )
}

#[test]
fn test_sliding_increases() {
    assert_eq!(
        sliding_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5
    );
}

fn main() {
    let samples: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();
    //println!("{}", increases(samples));
    println!("{}", sliding_increases(samples));
}
