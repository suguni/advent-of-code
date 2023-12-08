use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../data/2023/input8.txt");

#[derive(Debug, Eq, PartialEq)]
struct Docs {
    inst: String,
    start: String,
    networks: HashMap<String, LR>,
}

#[derive(Debug, Eq, PartialEq)]
struct LR {
    left: String,
    right: String,
}

fn load(data: &str) -> IResult<&str, Docs> {
    map(
        separated_pair(alpha1, tuple((newline, newline)), networks_parser),
        |(inst, (start, networks))| Docs {
            inst: inst.to_string(),
            start,
            networks,
        },
    )(data)
}

fn networks_parser(data: &str) -> IResult<&str, (String, HashMap<String, LR>)> {
    map(separated_list1(newline, network_parser), |vs| {
        (vs[0].0.clone(), vs.into_iter().collect())
    })(data)
}

fn network_parser(line: &str) -> IResult<&str, (String, LR)> {
    map(
        tuple((
            alpha1,
            tuple((space1, tag("="), space1, tag("("))),
            alpha1,
            tuple((tag(","), space1)),
            alpha1,
            tag(")"),
        )),
        |t: (&str, _, &str, _, &str, _)| {
            (
                t.0.to_string(),
                LR {
                    left: t.2.to_string(),
                    right: t.4.to_string(),
                },
            )
        },
    )(line)
}

fn solve1(data: &str) -> u32 {
    let (_, docs) = load(data).unwrap();

    let mut steps: usize = 0;
    let mut current: String = docs.start.clone();

    for c in docs.inst.chars().cycle() {
        if current == "ZZZ".to_string() {
            break;
        }

        let LR { left, right } = docs.networks.get(&current).expect("");

        if c == 'L' {
            current = left.clone();
        } else {
            current = right.clone();
        }

        steps += 1;
    }

    steps as u32
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const EX2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn solve1_test() {
        assert_eq!(solve1(EX1), 2);
        assert_eq!(solve1(EX2), 6);
    }

    #[test]
    fn quiz1_test() {
        assert_eq!(solve1(INPUT), 2);
    }

    #[test]
    fn network_parser_test() {
        let (_, (id, LR { left, right })) = network_parser("CCC = (ZZZ, GGG)").unwrap();
        assert_eq!(id, "CCC".to_string());
        assert_eq!(left, "ZZZ".to_string());
        assert_eq!(right, "GGG".to_string());
    }

    #[test]
    fn load_test() {
        let (_, docs) = load(EX2).unwrap();
        assert_eq!(docs.inst, "LLR".to_string());

        assert_eq!(
            docs.networks.get("AAA"),
            Some(&LR {
                left: "BBB".to_string(),
                right: "BBB".to_string(),
            })
        );
    }
}
