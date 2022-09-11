use std::io;

fn increases(samples: Vec<u32>) -> u32 {
    if samples.len() == 0 {
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

fn main() {
    let samples: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();
    println!("{}", increases(samples));
}
