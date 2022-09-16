#[derive(Debug, PartialEq)]
enum Validation {
    Ok,
    Invalid(char),
    Incomplete(String),
}

fn validate(syntax: &str) -> Validation {
    let mut stack = vec![];
    for c in syntax.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            '[' => stack.push(']'),
            ')' | '}' | '>' | ']' => {
                if let Some(expected) = stack.pop() {
                    if c != expected {
                        return Validation::Invalid(c);
                    }
                } else {
                    return Validation::Invalid(c);
                }
            }
            _ => return Validation::Invalid(c),
        }
    }
    if !stack.is_empty() {
        return Validation::Incomplete(String::from_iter(stack.into_iter().rev()));
    }
    Validation::Ok
}

#[test]
fn test_validate() {
    assert_eq!(validate("()"), Validation::Ok);
    assert_eq!(validate("<>"), Validation::Ok);
    assert_eq!(validate("{}"), Validation::Ok);
    assert_eq!(validate("[]"), Validation::Ok);
    assert_eq!(validate("([][])"), Validation::Ok);
    assert_eq!(validate("<<[]>>"), Validation::Ok);

    assert_eq!(validate("}"), Validation::Invalid('}'));

    assert_eq!(validate("["), Validation::Incomplete(String::from("]")));

    assert_eq!(validate("[("), Validation::Incomplete(String::from(")]")));

    assert_eq!(
        validate("{([(<{}[<>[]}>{[]{[(<()>"),
        Validation::Invalid('}')
    );
    assert_eq!(validate("[[<[([]))<([[{}[[()]]]"), Validation::Invalid(')'));
}

fn score_invalid(s: &str) -> u32 {
    match validate(s) {
        Validation::Invalid(c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        },
        _ => 0,
    }
}

#[test]
fn test_score_invalid() {
    assert_eq!(score_invalid(">"), 25137);
    assert_eq!(score_invalid("<>"), 0);
}

fn syntax_score(lines: &[&str]) -> u32 {
    lines.iter().map(|&s| score_invalid(s)).sum()
}

#[test]
fn test_syntax_score() {
    let example = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    let lines = example.trim().split('\n').collect::<Vec<_>>();
    assert_eq!(syntax_score(&lines), 26397);
}

fn score_autocomplete(s: &str) -> u64 {
    match validate(s) {
        Validation::Incomplete(completion) => completion
            .chars()
            .map(|c| match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            })
            .fold(0, |acc, v| acc * 5 + v),
        _ => 0,
    }
}

#[test]
fn test_score_autocomplete() {
    assert_eq!(score_autocomplete("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
    assert_eq!(score_autocomplete("[(()[<>])]({[<{<<[]>>("), 5566);
}

fn score_auto_many(lines: &[&str]) -> u64 {
    let scored = lines
        .iter()
        .map(|s| score_autocomplete(s))
        .filter(|&x| x != 0)
        .sorted()
        .collect::<Vec<_>>();
    scored[scored.len() / 2]
}

#[test]
fn test_score_auto_many() {
    let example = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    let lines = example.trim().split('\n').collect::<Vec<_>>();
    assert_eq!(score_auto_many(&lines), 288957);
}

use std::io;

use itertools::Itertools;

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let input = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    let slice: &[&str] = &input;

    println!("{}", syntax_score(slice));
    println!("{}", score_auto_many(slice));
}
