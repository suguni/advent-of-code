use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct P(u64, u64, u64);

impl P {
    fn dist(&self, p: &P) -> u64 {
        let d1 = self.0.abs_diff(p.0);
        let d2 = self.1.abs_diff(p.1);
        let d3 = self.2.abs_diff(p.2);
        d1 * d1 + d2 * d2 + d3 * d3
    }
}

const QUIZ_INPUT: &str = include_str!("../../data/2025/input8.txt");

fn quiz1() -> usize {
    solve1(QUIZ_INPUT, 1000)
}

fn quiz2() -> u64 {
    solve2(QUIZ_INPUT)
}

fn parse_data(input: &str) -> Vec<P> {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split(",")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            P(nums[0], nums[1], nums[2])
        })
        .collect()
}

fn distances(pts: &[P]) -> Vec<(u64, P, P)> {
    let mut result = vec![];
    for i in 0..pts.len() - 1 {
        let p1 = &pts[i];
        for j in i + 1..pts.len() {
            let p2 = &pts[j];
            let d = p1.dist(p2);
            result.push((d, *p1, *p2));
        }
    }
    result.sort_by_key(|(d, _, _)| *d);
    result
}

fn solve1(input: &str, count: usize) -> usize {
    let pts = parse_data(input);
    let mut circuits: Vec<HashSet<P>> = vec![];

    for (i, (_, p1, p2)) in distances(&pts).into_iter().enumerate() {
        if i >= count {
            break;
        }

        connect(&mut circuits, p1, p2);
    }

    circuits
        .iter()
        .map(|c| c.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn solve2(input: &str) -> u64 {
    let pts = parse_data(input);
    let mut circuits: Vec<HashSet<P>> = vec![];

    for (_, p1, p2) in distances(&pts) {
        connect(&mut circuits, p1, p2);
        if circuits.len() == 1 && circuits[0].len() == pts.len() {
            return p1.0 * p2.0;
        }
    }

    panic!("???")
}

fn connect(circuits: &mut Vec<HashSet<P>>, p1: P, p2: P) -> bool {
    let c1 = circuits
        .iter()
        .find_position(|set| set.contains(&p1))
        .map(|(i, _)| i);
    let c2 = circuits
        .iter()
        .find_position(|set| set.contains(&p2))
        .map(|(i, _)| i);

    if c1.is_none() && c2.is_none() {
        circuits.push(HashSet::from([p1, p2]));
    } else if c1.is_some() && c2.is_none() {
        circuits[c1.unwrap()].insert(p2);
    } else if c1.is_none() && c2.is_some() {
        circuits[c2.unwrap()].insert(p1);
    } else {
        let p1 = c1.unwrap();
        let p2 = c2.unwrap();
        if p1 == p2 {
            return false
        }

        let (p1, p2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

        let s2 = circuits.remove(p2);
        circuits[p1].extend(s2);
    }
    true
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    use super::*;

    #[test]
    fn test_parse() {
        let pts = parse_data(SAMPLE);
        assert_eq!(pts[0], P(162, 817, 812));
        assert_eq!(pts[1], P(57, 618, 57));
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE, 10), 40);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 66640);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 25272);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 78894156);
    }
}
