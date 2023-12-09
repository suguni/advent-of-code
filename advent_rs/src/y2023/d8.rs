use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, newline, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../data/2023/input8.txt");

#[derive(Debug, Eq, PartialEq)]
struct Docs {
    inst: String,
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
        |(inst, networks)| Docs {
            inst: inst.to_string(),
            networks,
        },
    )(data)
}

fn networks_parser(data: &str) -> IResult<&str, HashMap<String, LR>> {
    map(separated_list1(newline, network_parser), |vs| {
        vs.into_iter().collect()
    })(data)
}

fn network_parser(line: &str) -> IResult<&str, (String, LR)> {
    map(
        tuple((
            alphanumeric1,
            tuple((space1, tag("="), space1, tag("("))),
            alphanumeric1,
            tuple((tag(","), space1)),
            alphanumeric1,
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

fn solve1(data: &str) -> u64 {
    let (_, docs) = load(data).unwrap();
    count_steps(&docs, "AAA", |s| s == "ZZZ")
}

fn count_steps(docs: &Docs, start: &str, is_ending: fn(&str) -> bool) -> u64 {
    let mut current = start;
    let mut steps: u64 = 0;

    for c in docs.inst.chars().cycle() {
        if is_ending(current) {
            break;
        }

        let LR { left, right } = docs.networks.get(current).expect("");

        if c == 'L' {
            current = left;
        } else {
            current = right;
        }

        steps += 1;
    }

    steps
}

fn solve2(data: &str) -> u64 {
    let (_, docs) = load(data).unwrap();

    let mut currents = find_starts(&docs.networks);

    let steps = currents
        .iter()
        .map(|current| count_steps(&docs, current, |v| v.ends_with("Z")))
        .collect::<Vec<u64>>();

    lcm(steps)
}

fn find_starts(networks: &HashMap<String, LR>) -> Vec<String> {
    networks
        .keys()
        .filter_map(|key| {
            if key.ends_with("A") {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect()
}

fn lcm(ns: Vec<u64>) -> u64 {
    ns.into_iter()
        .reduce(|acc, n| acc * n / gcd(acc, n))
        .unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut c = 0;
    while b != 0 {
        c = a % b;
        a = b;
        b = c;
    }
    a
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
        assert_eq!(solve1(INPUT), 12361);
    }

    const EX3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn solve2_test() {
        assert_eq!(solve2(EX3), 6);
    }

    #[test]
    fn quiz2_test() {
        assert_eq!(solve2(INPUT), 18215611419223);
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
