#![allow(dead_code)]

use crate::read_file;
use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::{many0, separated_list1};
use nom::sequence::terminated;
use nom::{FindSubstring, IResult, Slice};
use num::{abs, signum};
use std::iter::{Enumerate, FilterMap, Iterator};
use std::ops::{Index, Sub};
use std::str::Chars;

const QUIZ_INPUT: &str = include_str!("../../data/2024/input2.txt");

fn parse_data(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    many0(terminated(
        separated_list1(space1, complete::i32),
        line_ending,
    ))(input)
}

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
}

fn solve1(input: &str) -> usize {
    let (_, data) = parse_data(input).unwrap();
    data.iter().filter(|report| check1(report)).count()
}

fn check1(report: &[i32]) -> bool {
    let mut sign = signum(report[1] - report[0]);
    let mut pre = report[0];
    for &i in report[1..].iter() {
        let d = (i - pre) * sign;
        if d >= 1 && d <= 3 {
            pre = i;
        } else {
            return false;
        }
    }
    true
}

fn solve2(input: &str) -> usize {
    let (_, data) = parse_data(input).unwrap();
    data.iter().filter(|report| check2(report)).count()
}

fn check2(report: &[i32]) -> bool {
    check1(report)
        || (0..report.len()).any(|i| {
            let mut vs = report.to_vec();
            vs.remove(i);
            check1(&vs)
        })
}

fn quiz2() -> usize {
    solve2(QUIZ_INPUT)
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 2);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 218);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(SAMPLE), 4);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 290);
    }
}
