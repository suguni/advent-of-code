#![allow(dead_code)]

use crate::read_file;
use itertools::Itertools;
use nom::{FindSubstring};
use num::abs;
use std::iter::{Enumerate, FilterMap, Iterator};
use std::ops::{Index, Sub};
use std::str::Chars;

const QUIZ_INPUT: &str = include_str!("../../data/2024/input1.txt");

fn parse_data(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let nums = line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (nums[0], nums[1])
        })
        .unzip()
}

fn quiz1() -> i32 {
    solve1(QUIZ_INPUT)
}

fn solve1(input: &str) -> i32 {
    parse_data(input)
        .0
        .iter()
        .sorted()
        .zip(parse_data(input).1.iter().sorted())
        .map(|(a, b)| abs(*a - *b))
        .sum()
}

fn solve2(input: &str) -> i32 {
    let (left, right) = parse_data(input);

    let left_counts = left.iter().counts_by(|v| *v);
    let right_counts = right.iter().counts_by(|v| *v);

    left_counts.iter().fold(0, |acc, (&k, &v)| {
        acc + (k * v as i32) * (*(right_counts.get(&k).unwrap_or(&0)) as i32)
    })
}

fn quiz2() -> i32 {
    solve2(QUIZ_INPUT)
}


#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 11);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 1882714);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(SAMPLE), 31);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 19437052);
    }
}

