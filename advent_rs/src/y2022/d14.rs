use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

use colored::Colorize;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use regex::Regex;

use crate::{interpolate_color, read_file};

const FILE_NAME: &str = "data/2022/input14.txt";

fn coord(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, tag(","), complete::u32)(input)
}

fn paths(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    separated_list0(newline, separated_list0(tag(" -> "), coord))(input)
}

fn load(input: &str) -> Vec<Vec<(u32, u32)>> {
    let (_, ps) = paths(input).unwrap();
    ps
}

fn proc1(input: &str) -> u32 {
    todo!();
}

fn quiz1() -> u32 {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(input: &str) -> u32 {
    todo!();
}

fn quiz2() -> u32 {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_load() {
        assert_eq!(
            load(INPUT),
            vec![
                vec![(498, 4), (498, 6), (496, 6)],
                vec![(503, 4), (502, 4), (502, 9), (494, 9)],
            ]
        );
    }

    #[test]
    #[ignore]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 24);
    }

    #[test]
    #[ignore]
    fn test_quiz1() {
        assert_eq!(quiz1(), 0);
    }

    #[test]
    #[ignore]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 0);
    }

    #[test]
    #[ignore]
    fn test_quiz2() {
        assert_eq!(quiz2(), 0);
    }
}
