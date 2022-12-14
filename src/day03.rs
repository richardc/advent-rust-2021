#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
fn battery(lines: &[String]) -> i32 {
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

#[cfg(test)]
const EXAMPLE: &str = include_str!("day3_example1.txt");

#[test]
fn test_battery() {
    assert_eq!(battery(&generate(EXAMPLE)), 198);
}

fn bitcount(lines: &[String], column: usize) -> i32 {
    lines
        .iter()
        .filter(|s| matches!(s.chars().nth(column), Some(c) if c == '1'))
        .count() as i32
}

fn bitselect(lines: &[String], criteria: fn(i32, i32) -> char) -> i32 {
    let bits = lines.first().unwrap().len();

    let mut set: Vec<_> = lines.iter().map(|s| s.to_string()).collect();
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

fn oxygen(lines: &[String]) -> i32 {
    bitselect(lines, |ones, zeroes| if ones >= zeroes { '1' } else { '0' })
}

#[test]
fn test_oxygen() {
    assert_eq!(oxygen(&generate(EXAMPLE)), 23);
}

fn co2(lines: &[String]) -> i32 {
    bitselect(lines, |ones, zeroes| if ones >= zeroes { '0' } else { '1' })
}

#[test]
fn test_co2() {
    assert_eq!(co2(&generate(EXAMPLE)), 10);
}

#[aoc(day3, part2)]
fn lifesupport(lines: &[String]) -> i32 {
    oxygen(lines) * co2(lines)
}

#[test]
fn test_lifesupport() {
    assert_eq!(lifesupport(&generate(EXAMPLE)), 230);
}
