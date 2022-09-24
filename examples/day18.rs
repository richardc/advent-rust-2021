use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    sequence::{delimited, separated_pair},
    IResult,
};
use regex::{Captures, Regex};

fn add(lhs: &str, rhs: &str) -> String {
    format!("[{},{}]", lhs, rhs)
}

#[test]
fn test_add() {
    assert_eq!(add("[1,2]", "[2,3]"), "[[1,2],[2,3]]")
}

fn split(s: &str) -> String {
    let re = Regex::new(r"\d{2,}").unwrap();
    if let Some(m) = re.find(&s) {
        let left = &s[0..m.start()];
        let num = m.as_str().parse::<i32>().unwrap();
        let right = &s[m.end()..];

        format!("{}[{},{}]{}", left, num / 2, (num + 1) / 2, right)
    } else {
        s.to_string()
    }
}

#[test]
fn test_split() {
    assert_eq!(split("[0,9]"), "[0,9]");
    assert_eq!(split("[0,10]"), "[0,[5,5]]");
    assert_eq!(split("[10,10]"), "[[5,5],10]");
    assert_eq!(split("[0,11]"), "[0,[5,6]]");
    assert_eq!(split("[0,12]"), "[0,[6,6]]");
}

fn explode(s: &str) -> String {
    fn exploding_pair(s: &str) -> Option<(usize, usize)> {
        let mut depth = 0;
        let mut start: Option<usize> = None;
        for (i, c) in s.char_indices() {
            if let Some(start) = start {
                if c == ']' {
                    return Some((start, i + 1));
                }
            } else if c == '[' {
                depth += 1;

                if depth == 5 {
                    start = Some(i)
                }
            } else if c == ']' {
                depth -= 1;
            }
        }
        None
    }

    if let Some((start, end)) = exploding_pair(&s) {
        let left = &s[0..start];
        let right = &s[end..];

        // Get the values from the exploding pair
        let (vl, vr) = &s[start + 1..end - 1].split_once(',').unwrap();
        let vl = vl.parse::<i32>().unwrap();
        let vr = vr.parse::<i32>().unwrap();

        // replace rightmost number with its value plus the right value of the exploding pair
        let re = Regex::new(r"\d+").unwrap();
        let right = re.replace(right, |c: &Captures| {
            let val = c.get(0).unwrap().as_str().parse::<i32>().unwrap();
            format!("{}", val + vr)
        });

        // like replacing on the right, but we must match from the end
        let re = Regex::new(r"(\d+)([^\d]*)\z").unwrap();
        let left = re.replace(left, |c: &Captures| {
            let val = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            format!("{}{}", val + vl, c.get(2).unwrap().as_str())
        });

        format!("{}0{}", left, right)
    } else {
        s.to_string()
    }
}

#[test]
fn test_explode() {
    assert_eq!(explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
    assert_eq!(explode("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");
    assert_eq!(explode("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");
    assert_eq!(
        explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    );
    assert_eq!(
        explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
    );

    assert_eq!(
        explode("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"),
        "[[[[0,7],4],[15,[0,13]]],[1,1]]"
    );
}

fn reduce(s: &str) -> String {
    let exploded = explode(s);
    if exploded != s {
        return reduce(&exploded);
    }

    let split = split(s);
    if split != s {
        return reduce(&split);
    }

    s.to_string()
}

#[test]
fn test_reduce() {
    assert_eq!(
        reduce("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    )
}

fn add_set(s: &[&str]) -> String {
    let mut res: String = s[0].to_string();
    for s in s.iter().skip(1) {
        res = reduce(&add(&res, &s))
    }
    res
}

#[test]
fn test_add_set() {
    assert_eq!(
        add_set(&["[1,1]", "[2,2]", "[3,3]", "[4,4]"]),
        "[[[[1,1],[2,2]],[3,3]],[4,4]]"
    );

    assert_eq!(
        add_set(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]),
        "[[[[3,0],[5,3]],[4,4]],[5,5]]"
    );

    assert_eq!(
        add_set(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]),
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
    );

    assert_eq!(
        add_set(&[
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]"
        ]),
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    );
}

fn magnitude(s: &str) -> u64 {
    type Number = u64;
    enum Pair {
        Value(Number),
        Pair(Box<Pair>, Box<Pair>),
    }

    fn parse_number(input: &str) -> IResult<&str, Pair> {
        let (input, value) = digit1(input)?;

        Ok((input, Pair::Value(value.parse::<Number>().unwrap())))
    }

    fn parse_pair(input: &str) -> IResult<&str, Pair> {
        let (input, (left, right)) = delimited(
            char('['),
            separated_pair(
                alt((parse_number, parse_pair)),
                tag(","),
                alt((parse_number, parse_pair)),
            ),
            char(']'),
        )(input)?;

        Ok((input, Pair::Pair(Box::new(left), Box::new(right))))
    }

    fn value(p: Pair) -> Number {
        match p {
            Pair::Value(v) => v,
            Pair::Pair(l, r) => 3 * value(*l) + 2 * value(*r),
        }
    }

    if let Ok((_, tree)) = parse_pair(s) {
        value(tree)
    } else {
        0
    }
}

#[test]
fn test_magnitude() {
    assert_eq!(magnitude("[9,1]"), 29);
    assert_eq!(magnitude("[1,9]"), 21);
    assert_eq!(magnitude("[[9,1],[1,9]]"), 129);
    assert_eq!(magnitude("[[1,2],[[3,4],5]]"), 143);
    assert_eq!(magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
    assert_eq!(magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
    assert_eq!(magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791);
    assert_eq!(magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137);
    assert_eq!(
        magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
        3488
    );
}

fn main() {}
