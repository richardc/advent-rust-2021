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
        _ => panic!("Impossible input"),
    }
}

fn known_outputs(input: &[&str]) -> usize {
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
"#
    .trim();

    assert_eq!(
        known_outputs(&(example.split('\n').collect::<Vec<_>>())),
        26
    );
}

use std::io;
fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let input = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    println!("{}", known_outputs(&input));
}
