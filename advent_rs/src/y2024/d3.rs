#![allow(dead_code)]

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i32};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::{FindSubstring, IResult, Parser};
use regex::Regex;
use std::iter::Iterator;
use std::ops::{Index, Sub};

const QUIZ_INPUT: &str = include_str!("../../data/2024/input3.txt");

const RE: &str = r"mul\(\d{1,3},\d{1,3}\)";

fn parse_data1(input: &str) -> Vec<(i32, i32)> {
    Regex::new(RE)
        .unwrap()
        .find_iter(input)
        .map(|m| m.as_str())
        .map(|mul| {
            let (_, p) = parse_mul(mul).unwrap();
            p
        })
        .collect()
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(
        tag("mul"),
        delimited(tag("("), separated_pair(i32, char(','), i32), tag(")")),
    ).parse(input)
}

fn quiz1() -> i32 {
    solve1(QUIZ_INPUT)
}

fn solve1(input: &str) -> i32 {
    let vs = parse_data1(input);
    vs.iter().map(|(a, b)| *a * *b).sum()
}

const RE2: &str = r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))";

fn parse_data2(input: &str) -> Vec<&str> {
    Regex::new(RE2).unwrap()
        .find_iter(input)
        .map(|m| m.as_str())
        .collect()
}

fn solve2(input: &str) -> i32 {
    let mut enabled = true;
    let mut result = 0;
    for m in parse_data2(input) {
        if m.starts_with("mul") && enabled {
            let (_, (a, b)) = parse_mul(m).unwrap();
            result += a * b;
        } else {
            enabled =  m == "do()";
        }
    }

    result
}

fn quiz2() -> i32 {
    solve2(QUIZ_INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_parse_data1() {
        let ms = parse_data1(SAMPLE);
        assert_eq!(ms, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 161);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 161085926);
    }

    const SAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_data2() {
        let ms = parse_data2(SAMPLE2);
        assert_eq!(ms, vec!("mul(2,4)", "don't()", "mul(5,5)", "mul(11,8)", "do()", "mul(8,5)"))
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(SAMPLE2), 48);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 82045421);
    }
}
