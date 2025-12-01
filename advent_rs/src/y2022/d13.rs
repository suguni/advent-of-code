use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

use colored::Colorize;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::eof;
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::{IResult, Parser};
use regex::Regex;

use PackData::*;

use crate::{interpolate_color, read_file};

const FILE_NAME: &str = "data/2022/input13.txt";

#[derive(Debug, PartialEq, Clone)]
enum PackData {
    V(u32),
    L(Vec<PackData>),
}

impl Eq for PackData {}

impl Ord for PackData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd<Self> for PackData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (V(v1), V(v2)) => v1.partial_cmp(v2),
            (L(vs1), L(vs2)) => vs1
                .iter()
                .zip(vs2)
                .map(|(v1, v2)| v1.partial_cmp(v2).unwrap())
                .find(|ordering| ordering.is_ne())
                .or_else(|| vs1.len().partial_cmp(&vs2.len())),
            (L(_), V(_)) => self.partial_cmp(&L(vec![other.clone()])),
            (V(_), L(_)) => L(vec![self.clone()]).partial_cmp(other),
        }
    }
}

type Packet = Vec<PackData>;

fn v(input: &str) -> IResult<&str, PackData> {
    let (input, n) = complete::u32(input)?;
    Ok((input, V(n)))
}

fn l(input: &str) -> IResult<&str, PackData> {
    let (input, _) = tag("[")(input)?;
    let (input, vs) = separated_list0(tag(","), alt((v, l))).parse(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, L(vs)))
}

fn parse_line(line: &str) -> Packet {
    if let Ok((_, L(vs))) = l(line) {
        vs
    } else {
        panic!()
    }
}

fn load_block(block: &str) -> (Packet, Packet) {
    let mut lines = block.lines();
    (
        parse_line(lines.next().unwrap()),
        parse_line(lines.next().unwrap()),
    )
}

fn load(input: &str) -> Vec<(Packet, Packet)> {
    input.split("\n\n").map(|block| load_block(block)).collect()
}

fn proc1(input: &str) -> u32 {
    load(input)
        .iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1.le(p2))
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

fn quiz1() -> u32 {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(input: &str) -> u32 {
    let mut ps = load(input)
        .into_iter()
        .flat_map(|(p1, p2)| vec![p1, p2])
        .collect::<Vec<Packet>>();

    ps.push(vec![L(vec![V(2)])]);
    ps.push(vec![L(vec![V(6)])]);

    ps.sort();

    ps.into_iter()
        .enumerate()
        .filter_map(|(idx, p)| {
            if p.len() == 1 && (p[0].eq(&L(vec![V(2)])) || p[0].eq(&L(vec![V(6)]))) {
                Some(idx as u32 + 1)
            } else {
                None
            }
        })
        .product()
}

fn quiz2() -> u32 {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("[1,1,3,1,1]"),
            vec![V(1), V(1), V(3), V(1), V(1)]
        );
        assert_eq!(
            parse_line("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            vec![
                V(1),
                L(vec![
                    V(2),
                    L(vec![V(3), L(vec![V(4), L(vec![V(5), V(6), V(7)]),])])
                ]),
                V(8),
                V(9)
            ]
        );
        assert_eq!(parse_line("[[[]]]"), vec![L(vec![L(vec![])])]);
        assert_eq!(
            parse_line("[[1],[2,3,4]]"),
            vec![L(vec![V(1)]), L(vec![V(2), V(3), V(4)])]
        );
    }

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 13);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 5882);
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 140);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 24948);
    }
}
