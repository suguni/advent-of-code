#![allow(dead_code)]

use itertools::Itertools;
use nom::{FindSubstring, Parser};
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::ops::{Index, Sub};

const QUIZ_INPUT: &str = include_str!("../../data/2024/input8.txt");

type Point = (u32, u32);
type Size = (u32, u32);
type Vector = (Point, Point);
type Pair = (Point, Point);

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
}

fn quiz2() -> usize {
    solve2(QUIZ_INPUT)
}

fn parse_data(input: &str) -> (Size, Vec<Vec<Point>>) {
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c != '.' {
                let positions = map.entry(c).or_insert(vec![]);
                positions.push((row as u32, col as u32));
            }
        });
    });

    let pos = map.into_iter().map(|(_, pos)| pos).collect();

    ((rows as u32, cols as u32), pos)
}

fn pairing(locs: &[Point]) -> Vec<Pair> {
    if locs.len() < 2 {
        panic!();
    }

    if locs.len() == 2 {
        return vec![(locs[0].clone(), locs[1].clone())];
    }

    let mut paired: Vec<Pair> = locs[1..]
        .iter()
        .map(|&pos| (locs[0].clone(), pos.clone()))
        .collect();

    paired.append(&mut pairing(&locs[1..]));
    paired
}

fn multiplier_points((from, to): &Vector, (rows, cols): &Size) -> Vec<Point> {
    let (x1, y1) = *from;
    let (x2, y2) = *to;

    let dx = (x2 as i32) - (x1 as i32);
    let dy = (y2 as i32) - (y1 as i32);

    let mut pts = vec![];
    let mut pt = (x2 as i32 + dx, y2 as i32 + dy);

    while pt.0 < (*rows as i32) && pt.1 < (*cols as i32) && pt.0 >= 0 && pt.1 >= 0 {
        pts.push((pt.0 as u32, pt.1 as u32));
        pt = (pt.0 + dx, pt.1 + dy);
    }

    pts
}

fn anti_nodes(&(p1, p2): &Pair, size: &Size) -> Vec<Point> {
    let d1 = multiplier_points(&(p1, p2), size);
    let d2 = multiplier_points(&(p2, p1), size);
    let mut result = vec![];

    if d1.len() > 0 {
        result.push(d1[0]);
    }

    if d2.len() > 0 {
        result.push(d2[0]);
    }

    result
}

fn multiple_anti_nodes(&(p1, p2): &Pair, size: &Size) -> Vec<Point> {
    let mut d1 = multiplier_points(&(p1, p2), size);
    let mut d2 = multiplier_points(&(p2, p1), size);
    let mut result = vec![];
    result.append(&mut d1);
    result.append(&mut d2);
    result
}

fn solve1(input: &str) -> usize {
    let (size, locations) = parse_data(input);

    let nodes: HashSet<Point> = locations
        .iter()
        .flat_map(|loc| {
            pairing(loc)
                .iter()
                .flat_map(|pair| anti_nodes(pair, &size))
                .collect::<Vec<_>>()
        })
        .collect();

    nodes.len()
}

fn solve2(input: &str) -> usize {
    let (size, locations) = parse_data(input);

    let anti_nodes: HashSet<Point> = locations
        .iter()
        .flat_map(|loc| {
            pairing(loc)
                .iter()
                .flat_map(|pair| multiple_anti_nodes(pair, &size))
                .collect::<Vec<_>>()
        })
        .collect();

    let antennas = locations
        .iter()
        .flatten()
        .map(|p| *p)
        .collect::<HashSet<_>>();

    anti_nodes.union(&antennas).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse_data() {
        let (size, pos) = parse_data(SAMPLE);

        assert_eq!(
            eq_vec(
                &pos,
                &vec![
                    vec![(1, 8), (2, 5), (3, 7), (4, 4)],
                    vec![(5, 6), (8, 8), (9, 9)]
                ],
                PartialEq::eq
            ),
            true
        );

        assert_eq!(size, (12, 12))
    }

    #[test]
    fn test_pairing() {
        let paired = pairing(&vec![(1, 8), (2, 5)]);
        assert_eq!(paired, vec![((1, 8), (2, 5))]);

        let paired = pairing(&vec![(1, 8), (2, 5), (3, 7)]);
        assert_eq!(
            eq_vec(
                &paired,
                &vec![((1, 8), (2, 5)), ((2, 5), (3, 7)), ((1, 8), (3, 7))],
                eq_pair
            ),
            true
        );

        let paired = pairing(&vec![(1, 8), (2, 5), (3, 7), (4, 4)]);
        assert_eq!(
            eq_vec(
                &paired,
                &vec![
                    ((1, 8), (2, 5)),
                    ((2, 5), (3, 7)),
                    ((3, 7), (4, 4)),
                    ((1, 8), (3, 7)),
                    ((2, 5), (4, 4)),
                    ((1, 8), (4, 4))
                ],
                eq_pair
            ),
            true
        );
    }

    #[test]
    fn test_anti_nodes() {
        let a_nodes = anti_nodes(&((0, 0), (1, 0)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(2, 0)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((0, 0), (0, 1)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(0, 2)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((0, 10), (0, 11)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(0, 9)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((1, 11), (0, 11)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(2, 11)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((10, 0), (11, 0)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(9, 0)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((11, 0), (11, 1)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(11, 2)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((11, 10), (11, 11)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(11, 9)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((10, 11), (11, 11)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(9, 11)], PartialEq::eq), true);

        let a_nodes = anti_nodes(&((1, 8), (2, 5)), &(12, 12));
        assert_eq!(
            eq_vec(&a_nodes, &vec![(0, 11), (3, 2)], PartialEq::eq),
            true
        );

        let a_nodes = anti_nodes(&((1, 8), (3, 7)), &(12, 12));
        assert_eq!(eq_vec(&a_nodes, &vec![(5, 6)], PartialEq::eq), true);
    }

    fn eq_vec<T>(v1: &[T], v2: &[T], eq: fn(&T, &T) -> bool) -> bool
    where
        T: PartialEq + Clone,
    {
        v1.iter().all(|p1| v2.iter().any(|p2| eq(p1, p2)))
            && v2.iter().all(|p2| v1.iter().any(|p1| eq(p1, p2)))
    }

    fn eq_pair(pair1: &Vector, pair2: &Vector) -> bool {
        let (p3, p4) = pair2;
        *pair1 == *pair2 || *pair1 == (*p4, *p3)
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 14);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 214);
    }

    #[test]
    fn test_multiplier_points() {
        assert_eq!(
            multiplier_points(&((0, 0), (2, 2)), &(10, 10)),
            vec![(4, 4), (6, 6), (8, 8)]
        );
        assert_eq!(
            multiplier_points(&((9, 9), (7, 7)), &(10, 10)),
            vec![(5, 5), (3, 3), (1, 1)]
        );
        assert_eq!(
            multiplier_points(&((0, 0), (3, 1)), &(10, 10)),
            vec![(6, 2), (9, 3)]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(SAMPLE), 34);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 809);
    }
}
