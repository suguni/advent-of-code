#![allow(dead_code)]

use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use std::iter::Iterator;
use std::ops::{Index, Sub};

const QUIZ_INPUT: &str = include_str!("../../data/2025/input1.txt");

fn parse_data(input: &str) -> IResult<&str, Vec<(char, i32)>> {
    separated_list1(
        newline,
        (alt((char('L'), char('R'))), nom::character::complete::i32),
    )
    .parse(input)
}

fn quiz1() -> i32 {
    solve1(QUIZ_INPUT)
}

fn solve1(input: &str) -> i32 {
    let mut p: i32 = 50;
    let mut count = 0;
    let (_, data) = parse_data(input).unwrap();
    for (dir, deg) in data {
        match dir {
            'L' => {
                p -= deg;
                p %= 100;
                if p == 0 {
                    count += 1;
                } else if p < 0 {
                    p = 100 + p;
                }
            },
            'R' => {
                p += deg;
                p %= 100;
                if p == 0 {
                    count += 1;
                }
            },
            _ => panic!("unknown direction: {}", dir),
        }

    }
    count
}

fn solve2(input: &str) -> i32 {
    let mut p: i32 = 50;
    let mut count = 0;
    let (_, data) = parse_data(input).unwrap();
    for (dir, deg) in data {
        let op = p;
        match dir {
            'L' => {
                p -= deg;
                if op != 0 && p <= 0 {
                    count += 1;
                }

                count += -p / 100;

                if p < 0 {
                    p = (100 + p % 100) % 100;
                }
            },
            'R' => {
                p += deg;
                count += p / 100;
                p %= 100;
            },
            _ => panic!("unknown direction: {}", dir),
        }
    }
    count
}

fn quiz2() -> i32 {
    solve2(QUIZ_INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 3);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 1145);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(SAMPLE), 6);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 6561);
    }
}
