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

fn check(result: u64, nums: &Vec<u64>) -> bool {
    for ops in perms(nums.len() - 1) {
        let mut computed = nums[0];
        for (idx, c) in ops.chars().enumerate().collect::<Vec<_>>() {
            computed = operate(computed, c, nums[idx+1]);
        }
        println!("{result} == {computed}, nums = {nums:?}, {ops}");
        if result == computed {
            return true;
        }
    }
    false
}

fn operate(lh: u64, op: char, rh: u64) -> u64 {
    if op == '*' {
        lh * rh
    } else {
        lh + rh
    }
}

fn perms(size: usize) -> HashSet<String> {
    if size < 1 {
        set![]
    } else if size == 1 {
        set!["*".to_string(), "+".to_string()]
    } else {
        perms(size - 1)
            .iter()
            .flat_map(|perm| {
                let mut v1 = (*perm).clone();
                let mut v2 = (*perm).clone();
                v1.insert(0, '*');
                v2.insert(0, '+');
                vec![v1, v2]
            })
            .collect()
    }
}

fn solve1(input: &str) -> u64 {
    let (_, data) = parse_data(input).unwrap();
    data.iter()
        .filter(|(res, vs)| check(*res, vs))
        .map(|(res, _)| res)
        .sum()
}

fn solve2(input: &str) -> u64 {
    todo!()
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
    fn test1() {
        assert_eq!(solve1(SAMPLE), 3749);
    }

    #[test]
    fn test_perm() {
        assert_eq!(
            perms(2),
            set![
                "**".to_string(),
                "*+".to_string(),
                "+*".to_string(),
                "++".to_string()
            ]
        );
        assert_eq!(
            perms(3),
            set![
                "***".to_string(),
                "**+".to_string(),
                "*+*".to_string(),
                "+**".to_string(),
                "*++".to_string(),
                "+*+".to_string(),
                "++*".to_string(),
                "+++".to_string()
            ]
        );
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 1882714);
    }

    // #[test]
    // fn test2() {
    //     assert_eq!(solve2(SAMPLE), 31);
    // }
    //
    // #[test]
    // fn run_quiz2() {
    //     assert_eq!(quiz2(), 19437052);
    // }
}
