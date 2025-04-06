#![allow(dead_code)]

use crate::{read_file, set};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::u64 as cu64;
use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{FindSubstring, IResult, Slice};
use num::abs;
use std::collections::{HashMap, HashSet};
use std::iter::{Enumerate, FilterMap, Iterator};
use std::ops::{Index, Sub};
use std::str::Chars;

const QUIZ_INPUT: &str = include_str!("../../data/2024/input7.txt");

fn quiz1() -> u64 {
    solve1(QUIZ_INPUT)
}

fn quiz2() -> u64 {
    solve2(QUIZ_INPUT)
}

fn parse_data(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        newline,
        separated_pair(cu64, tag(": "), separated_list1(space1, cu64)),
    )(input)
}

fn num_digit(mut num: u64) -> u32 {
    let mut i = 1;
    while num >= 10 {
        num /= 10;
        i += 1;
    }
    i
}

fn reduced(result: u64, num: u64, op: char) -> Option<u64> {
    match op {
        '*' => {
            if result >= num && result % num == 0 {
                Some(result / num)
            } else {
                None
            }
        }
        '|' => {
            let divider = 10_u64.pow(num_digit(num));
            if result >= num && result % divider == num {
                Some(result / divider)
            } else {
                None
            }
        }
        '+' => {
            if result >= num {
                Some(result - num)
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn check(result: u64, nums: &[u64], ops: &[char]) -> bool {

    if nums.len() == 1 {
        return result == nums[0];
    }

    if result == 0 && nums.len() > 0 {
        return false;
    }

    let num = nums[nums.len() - 1];
    for op in ops {
        if let Some(r) = reduced(result, num, *op) {
            if check(r, &nums[0..(nums.len() - 1)], ops) {
                return true;
            }
        }
    }

    false
}

fn solve(input: &str, ops: &[char]) -> u64 {
    let (_, data) = parse_data(input).unwrap();
    data.iter()
        .filter(|(res, vs)| check(*res, vs, ops))
        .map(|(res, _)| res)
        .sum()
}

fn solve1(input: &str) -> u64 {
    solve(input, &['+', '*'])
}

fn solve2(input: &str) -> u64 {
    solve(input, &['+', '*', '|'])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_parse_data() {
        let (_, data) = parse_data(
            "190: 10 19
3267: 81 40 27",
        )
        .unwrap();
        assert_eq!(data, vec![(190, vec![10, 19]), (3267, vec![81, 40, 27])])
    }

    #[test]
    fn reduced_test() {
        assert_eq!(reduced(156, 6, '|'), Some(15));
        assert_eq!(reduced(7290, 15, '*'), Some(486));
        assert_eq!(reduced(3267, 27, '|'), None);
        assert_eq!(reduced(83, 5, '*'), None);
        assert_eq!(reduced(12345, 345, '|'), Some(12));
        assert_eq!(reduced(12345, 12345, '|'), Some(0));
    }

    #[test]
    fn check2_test() {
        assert_eq!(check(156, &vec![15, 6], &vec!['+', '*', '|']), true);
        assert_eq!(check(3222143, &vec![98, 427, 77, 4, 1], &vec!['+', '*']), false);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 3749);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 20665830408335);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(SAMPLE), 11387);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 354060705047464);
    }
}
