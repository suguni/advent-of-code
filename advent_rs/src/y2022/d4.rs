use std::{convert::TryInto, ops::RangeInclusive};

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, *,
};
use regex::Regex;

use crate::read_file;

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (s, e)) = separated_pair(
        nom::character::complete::u32,
        tag("-"),
        nom::character::complete::u32,
    )(input)?;
    IResult::Ok((input, s..=e))
}

fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    separated_list1(newline, parse_line)(input)
}

fn load(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (_, ranges) = parse_lines(input).unwrap();
    ranges
}

fn load_regex(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    let regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let cas = regex.captures(line).unwrap();
            let l1 = *&cas[1].parse::<usize>().unwrap();
            let l2 = *&cas[2].parse::<usize>().unwrap();
            let r1 = *&cas[3].parse::<usize>().unwrap();
            let r2 = *&cas[4].parse::<usize>().unwrap();
            (l1..=l2, r1..=r2)
        })
        .collect()
}

fn process1(input: &str) -> usize {
    let rs = load(input);
    rs.iter()
        .filter(|(a, b)| {
            (a.start() <= b.start() && b.end() <= a.end())
                || (b.start() <= a.start() && a.end() <= b.end())
        })
        .count()
}

fn quiz1() -> usize {
    let input = read_file("data/2022/input4.txt");
    process1(&input)
}

fn process2(input: &str) -> usize {
    let rs = load(input);
    rs.iter()
        .filter(|(a, b)| {
            (a.start() <= b.start() && b.start() <= a.end())
                || (b.start() <= a.start() && a.start() <= b.end())
        })
        .count()
}

fn quiz2() -> usize {
    let input = read_file("data/2022/input4.txt");
    process2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_process1() {
        assert_eq!(process1(INPUT), 2);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 464);
    }

    #[test]
    fn test_process2() {
        assert_eq!(process2(INPUT), 4);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 770);
    }
}
