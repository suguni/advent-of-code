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

const QUIZ_INPUT: &str = include_str!("../../data/2024/input8.txt");

type Point = (u32, u32);
type Pair = (Point, Point);

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
}

fn quiz2() -> usize {
    solve2(QUIZ_INPUT)
}

fn parse_data(input: &str) -> ((u32, u32), Vec<Vec<(u32, u32)>>) {
    let mut map: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

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

fn pairing(locs: &[(u32, u32)]) -> Vec<((u32, u32), (u32, u32))> {
    if locs.len() < 2 {
        panic!();
    }

    if locs.len() == 2 {
        return vec![(locs[0].clone(), locs[1].clone())];
    }

    let mut paired: Vec<((u32, u32), (u32, u32))> = locs[1..]
        .iter()
        .map(|&pos| (locs[0].clone(), pos.clone()))
        .collect();

    paired.append(&mut pairing(&locs[1..]));
    paired
}

fn anti_nodes((p1, p2): &Pair, (rows, cols): &(u32, u32)) -> Vec<Point> {
    let (r1, _) = p1;
    let (r2, _) = p2;

    let pt1: &Point;
    let pt2: &Point;

    if r1 < r2 {
        pt1 = p1;
        pt2 = p2;
    } else {
        pt1 = p2;
        pt2 = p1;
    }

    let &(r1, c1) = pt1;
    let &(r2, c2) = pt2;

    let dr = r1.abs_diff(r2);
    let dc = c1.abs_diff(c2);

    let mut result: Vec<Point> = vec![];

    if c1 < c2 {
        if r1 >= dr && c1 >= dc {
            result.push((r1 - dr, c1 - dc))
        }
        if rows - r2 > dr && cols - c2 > dc {
            result.push((r2 + dr, c2 + dc))
        }
    } else {
        if r1 >= dr && cols - c1 > dc {
            result.push((r1 - dr, c1 + dc))
        }
        if rows - r2 > dr && c2 >= dc {
            result.push((r2 + dr, c2 - dc))
        }
    }

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
    todo!()
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
        assert_eq!(eq_vec(&a_nodes, &vec![(0, 11), (3, 2)], PartialEq::eq), true);

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

    fn eq_pair(pair1: &Pair, pair2: &Pair) -> bool {
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
}
