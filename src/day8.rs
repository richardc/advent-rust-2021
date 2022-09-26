#[aoc_generator(day8)]
fn generate(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

type Digit = u8;

enum Output {
    Is(Digit),
    Maybe(Vec<Digit>),
}

fn output_is(s: &str) -> Output {
    match s.len() {
        2 => Output::Is(1),
        3 => Output::Is(7),
        4 => Output::Is(4),
        5 => Output::Maybe(vec![2, 3, 5]),
        6 => Output::Maybe(vec![0, 6, 9]),
        7 => Output::Is(8),
        _ => unreachable!(),
    }
}

#[aoc(day8, part1)]
fn known_outputs(input: &[String]) -> usize {
    input
        .iter()
        .flat_map(|l| match l.split_once('|') {
            Some((_, out)) => out.trim().split(' ').collect(),
            None => vec![],
        })
        .map(output_is)
        .filter(|x| matches!(x, Output::Is(_)))
        .count()
}

#[test]
fn test_known_outputs() {
    let example = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    assert_eq!(known_outputs(&generate(example.trim())), 26);
}

fn decode_segments(s: &str) -> u32 {
    let one = s.split(' ').find(|s| s.len() == 2).unwrap();
    let four = s.split(' ').find(|s| s.len() == 4).unwrap();
    let digits = s.split_once('|').unwrap().1.trim();

    digits
        .split(' ')
        .map(|s| match s.len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            len => {
                let ones = one.chars().filter(|&c| s.contains(c)).count();
                let fours = four.chars().filter(|&c| s.contains(c)).count();
                match (len, ones, fours) {
                    (6, _, 3) => 0,
                    (5, 1, 2) => 2,
                    (5, 2, 3) => 3,
                    (5, 1, 3) => 5,
                    (6, 1, _) => 6,
                    (6, _, 4) => 9,
                    _ => unreachable!(),
                }
            }
        })
        .fold(0, |acc, x| acc * 10 + x)
}

#[test]
fn test_decode_segments() {
    let example =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    assert_eq!(decode_segments(example), 5353);
}

#[aoc(day8, part2)]
fn decode_all_segments(input: &[String]) -> u32 {
    input.iter().map(|s| decode_segments(s)).sum()
}
