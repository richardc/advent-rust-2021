use std::{fmt, ops};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    sequence::{delimited, separated_pair},
    IResult,
};

type Number = u32;

#[derive(Debug, PartialEq, Clone)]
enum Pair {
    Number(Number),
    Pair(Box<Self>, Box<Self>),
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Number(v) => write!(f, "{}", v),
            Pair::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

#[test]
fn test_pair_display() {
    assert_eq!("2", format!("{}", Pair::Number(2)));
    assert_eq!(
        "[2,3]",
        format!(
            "{}",
            Pair::Pair(Box::new(Pair::Number(2)), Box::new(Pair::Number(3)))
        )
    );

    assert_eq!(
        "[2,[3,4]]",
        format!(
            "{}",
            Pair::Pair(
                Box::new(Pair::Number(2)),
                Box::new(Pair::Pair(
                    Box::new(Pair::Number(3)),
                    Box::new(Pair::Number(4))
                ))
            )
        )
    );
}
fn parse_number(input: &str) -> IResult<&str, Pair> {
    let (input, value) = digit1(input)?;

    Ok((input, Pair::Number(value.parse::<Number>().unwrap())))
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

#[test]
fn test_parse_pair() {
    assert_eq!(
        Ok((
            "",
            Pair::Pair(Box::new(Pair::Number(2)), Box::new(Pair::Number(3)))
        )),
        parse_pair("[2,3]")
    );

    assert_eq!(
        Ok((
            "",
            Pair::Pair(
                Box::new(Pair::Number(2)),
                Box::new(Pair::Pair(
                    Box::new(Pair::Number(3)),
                    Box::new(Pair::Number(4))
                ))
            )
        )),
        parse_pair("[2,[3,4]]")
    );
}

impl From<&str> for Pair {
    fn from(input: &str) -> Self {
        if let Ok((_, pair)) = parse_pair(input) {
            pair
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test_pair_from_strref() {
    assert_eq!(
        Pair::from("[1,2]]"),
        Pair::Pair(Box::new(Pair::Number(1)), Box::new(Pair::Number(2)))
    );
}

impl PartialEq<&str> for Pair {
    fn eq(&self, other: &&str) -> bool {
        Pair::from(*other) == *self
    }
}

#[test]
fn test_pair_eq_strref() {
    assert_eq!(Pair::from("[1,2]"), "[1,2]");
}

impl ops::Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair::Pair(Box::new(self), Box::new(rhs))
    }
}

#[test]
fn test_pair_add() {
    assert_eq!(
        Pair::from("[1,2]") + Pair::from("[[3,4],5]"),
        "[[1,2],[[3,4],5]]"
    );
}

impl Pair {
    fn explode(self) -> Self {
        self
    }
}

#[test]
fn test_pair_explode() {
    assert_eq!(
        Pair::from("[[[[[9,8],1],2],3],4]").explode(),
        "[[[[0,9],2],3],4]"
    );

    assert_eq!(
        Pair::from("[7,[6,[5,[4,[3,2]]]]]").explode(),
        "[7,[6,[5,[7,0]]]]"
    );

    assert_eq!(
        Pair::from("[[6,[5,[4,[3,2]]]],1]").explode(),
        "[[6,[5,[7,0]]],3]"
    );

    assert_eq!(
        Pair::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").explode(),
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    );

    assert_eq!(
        Pair::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").explode(),
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
    );
}

impl Pair {
    fn split(self) -> Self {
        match self {
            Pair::Number(x) if x > 9 => Pair::Pair(
                Box::new(Pair::Number(x / 2)),
                Box::new(Pair::Number((x + 1) / 2)),
            ),
            Pair::Pair(left, right) => Pair::Pair(Box::new(left.split()), Box::new(right.split())),
            _ => self,
        }
    }
}

#[test]
fn test_pair_split() {
    assert_eq!(Pair::from("[0,9]").split(), "[0,9]");
    assert_eq!(Pair::from("[0,10]").split(), "[0,[5,5]]");
    assert_eq!(Pair::from("[0,11]").split(), "[0,[5,6]]");
    assert_eq!(Pair::from("[0,12]").split(), "[0,[6,6]]");
}

impl Pair {
    fn reduce(self) -> Self {
        let old = self.clone();

        let exploded = self.clone().explode();
        if exploded != old {
            return exploded.reduce();
        }

        let split = self.clone().split();
        if split != old {
            return split.reduce();
        }
        return self;
    }
}

#[test]
fn test_pair_reduce() {
    assert_eq!(
        (Pair::from("[[[[4,3],4],4],[7,[[8,4],9]]]") + Pair::from("[1,1]")).reduce(),
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    );
}

fn main() {}
