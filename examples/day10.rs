#[derive(Debug, PartialEq)]
enum Validation {
    Ok,
    Invalid(char),
    Incomplete,
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
        return Validation::Incomplete;
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

    assert_eq!(validate("["), Validation::Incomplete);
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

use std::io;

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let input = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    let slice: &[&str] = &input;

    println!("{}", syntax_score(slice));
}
