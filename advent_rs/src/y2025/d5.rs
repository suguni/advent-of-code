use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

const QUIZ_INPUT: &str = include_str!("../../data/2025/input5.txt");

fn quiz1() -> u64 {
    solve1(QUIZ_INPUT)
}

fn quiz2() -> u64 {
    solve2(QUIZ_INPUT)
}

fn solve1(data: &str) -> u64 {
    let (_, data) = parse_data(data).unwrap();
    let mut count = 0;
    for id in data.ids {
        if data.ranges.iter().find(|(l, r)| id >= *l && id <= *r).is_some() {
            count += 1;
        }
    }
    count
}

fn solve2(data: &str) -> u64 {
    let (_, mut data) = parse_data(data).unwrap();
    let mut merged_ranges: Vec<(u64, u64)> = vec![];

    data.ranges.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));

    let mut i = 0;
    let mut j = 1;
    while i < data.ranges.len() - 1 {
        let mut r1 = data.ranges[i].clone();
        j = i + 1;
        while j < data.ranges.len() {
            let r2 = data.ranges[j];
            if intersect(&r1, &r2) {
                r1 = merge(&r1, &r2);
                j += 1;
            } else {
                break;
            }
        }
        i = j;
        merged_ranges.push(r1);
    }

    merged_ranges.into_iter()
        .map(|(s, e)| e - s + 1)
        .sum()
}

fn intersect(r1: &(u64, u64), r2: &(u64, u64)) -> bool {
    !(r1.1 < r2.0 || r2.1 < r1.0)
}

fn merge(r1: &(u64, u64), r2: &(u64, u64)) -> (u64, u64) {
    (r1.0.min(r2.0), r1.1.max(r2.1))
}

fn parse_data(input: &str) -> IResult<&str, Data> {
    separated_pair(
        separated_list1(
            newline,
            separated_pair(complete::u64, tag("-"), complete::u64).map(|(a, b)| (a, b)),
        ),
        (newline, newline),
        separated_list1(newline, complete::u64),
    )
    .map(|(ranges, ids)| Data { ranges, ids })
    .parse(input)
}

struct Data {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    use super::*;

    #[test]
    fn test_parse() {
        let (_, data) = parse_data(SAMPLE).unwrap();
        assert_eq!(data.ranges, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
        assert_eq!(data.ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 3);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 640);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 14);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 365804144481581);
    }
}
