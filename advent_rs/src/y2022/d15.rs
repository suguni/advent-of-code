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

const FILE_NAME: &str = "data/2022/input15.txt";

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
    let cave = to_cave(load(input), false);
    fall_1(cave, 500, 0)
}

fn quiz1() -> u32 {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(input: &str) -> u32 {
    let cave = to_cave(load(input), true);
    fall_2(cave, 500, 0)
}

fn quiz2() -> u32 {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

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
    fn test_to_cave() {
        let vs = load(INPUT);
        let cave = to_cave(vs, false);
        let mut rocks: HashMap<u32, HashSet<u32>> = HashMap::new();
        rocks.insert(4, set![498, 502, 503]);
        rocks.insert(5, set![498, 502]);
        rocks.insert(6, set![496, 497, 498, 502]);
        rocks.insert(7, set![502]);
        rocks.insert(8, set![502]);
        rocks.insert(9, set![494, 495, 496, 497, 498, 499, 500, 501, 502]);
        let base = set![
            (498, 4),
            (502, 4),
            (503, 4),
            (498, 5),
            (502, 5),
            (496, 6),
            (497, 6),
            (498, 6),
            (502, 6),
            (502, 7),
            (502, 8),
            (494, 9),
            (495, 9),
            (496, 9),
            (497, 9),
            (498, 9),
            (499, 9),
            (500, 9),
            (501, 9),
            (502, 9)
        ];
        assert_eq!(
            cave,
            Cave {
                left: 494,
                top: 4,
                right: 503,
                bottom: 9,
                inf: false,
                rocks,
                base
            }
        );
    }

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 24);
    }
    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 843);
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 93);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 27625);
    }
}
