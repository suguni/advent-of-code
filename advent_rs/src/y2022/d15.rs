use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeInclusive;

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

type Coord = (i64, i64);
type Bound = (i64, i64, i64, i64);

fn load(input: &str) -> (Vec<Coord>, Vec<Coord>) {
    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let cas = regex.captures(line).unwrap();
            let l1 = *&cas[1].parse::<i64>().unwrap();
            let l2 = *&cas[2].parse::<i64>().unwrap();
            let r1 = *&cas[3].parse::<i64>().unwrap();
            let r2 = *&cas[4].parse::<i64>().unwrap();
            (l1, l2, r1, r2)
        })
        .fold(
            (vec![], vec![]),
            |(mut sensors, mut beacons), (sx, sy, bx, by)| {
                sensors.push((sx, sy));
                beacons.push((bx, by));
                (sensors, beacons)
            },
        )
}

fn proc1(input: &str, y: i64) -> usize {
    let (sensors, beacons) = load(input);
    let bounds = sensors_bound(&sensors, &beacons);
    let (left, top, right, bottom) = boundary(&bounds);

    println!("bound: {left}, {top}, {right}, {bottom}");

    let candidate_sensors_idx = bounds
        .iter()
        .enumerate()
        .filter(|(_, ((_, t, _, b), _))| *t <= y && y <= *b)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();

    println!("candidates {:?}", candidate_sensors_idx);

    let mut line = vec!['.'; (right - left + 1) as usize];
    beacons.iter().for_each(|(bx, by)| {
        if *by == y {
            line[(*bx - left) as usize] = 'B';
        }
    });

    for i in candidate_sensors_idx {
        let (sx, sy) = sensors[i];
        let (_, radius) = bounds[i];
        let dy = (y - sy).abs();
        let left_dx = radius - dy;

        let width = left_dx * 2 + 1;
        let start = sx - left_dx - left;

        for x in start..start + width {
            if line[x as usize] != 'B' {
                line[x as usize] = '#';
            }
        }
    }

    // println!("{:?}", line.iter().collect::<String>());

    line.iter().filter(|x| **x == '#').count()
}

fn boundary(bounds: &Vec<(Bound, i64)>) -> (i64, i64, i64, i64) {
    bounds.iter().fold(
        (i64::MAX, i64::MAX, i64::MIN, i64::MIN),
        |(al, at, ar, ab), ((bl, bt, br, bb), _)| {
            (al.min(*bl), at.min(*bt), ar.max(*br), ab.max(*bb))
        },
    )
}

fn dist((sx, sy): Coord, (bx, by): Coord) -> i64 {
    (sx - bx).abs() + (sy - by).abs()
}

fn quiz1() -> usize {
    let input = read_file(FILE_NAME);
    proc1(&input, 2000000)
}

fn sensors_bound(sensors: &Vec<Coord>, beacons: &Vec<Coord>) -> Vec<(Bound, i64)> {
    sensors
        .into_iter()
        .zip(beacons)
        .map(|(s, b)| {
            let d = dist(*s, *b);
            let (sx, sy) = *s;
            ((sx - d, sy - d, sx + d, sy + d), d)
        })
        .collect()
}

fn candidate(&(l, t, r, b): &Bound, size: i64) -> bool {
    let sx = l + (r - l) / 2;
    let sy = t + (b - t) / 2;
    let sr = sx - l;

    if l > size || b < 0 || r < 0 || t > size {
        // outside
        false
    } else if 0 <= sx && sx <= size && 0 <= sy && sy <= size {
        // inside
        true
    } else if 0 <= r && r <= size && 0 <= sy && sy <= size {
        // left
        true
    } else if 0 <= l && l <= size && 0 <= sy && sy <= size {
        // right
        true
    } else if 0 <= b && b <= size && 0 <= sx && sx <= size {
        // top
        true
    } else if 0 <= t && t <= size && 0 <= sx && sx <= size {
        // bottom
        true
    } else {
        if sx > size && sy < 0 {
            // top right
            sx - size + 0 - sy <= sr
        } else if sx > size && sy > b {
            // bottom right
            sx - size + sy - b <= sr
        } else if sx < 0 && sy < 0 {
            // top left
            0 - sx + 0 - sy <= sr
        } else if sx < 0 && sy > b {
            // bottom left
            0 - sx + sy - b <= sr
        } else {
            panic!();
        }
    }
}

fn contain_spot((x, y): Coord, (sx, sy): Coord) -> bool {
    true
}

fn proc2(input: &str, sz: i64) -> usize {
    let (sensors, beacons) = load(input);
    let bounds = sensors_bound(&sensors, &beacons);

    for y in 0..=sz {
        println!("{y}");

        let mut ranges: Vec<(i64, i64)> = vec![];
        for i in 0..sensors.len() {
            let (sx, sy) = sensors[i];
            let (_, radius) = bounds[i];
            let dy = (y - sy).abs();
            let left_dx = radius - dy;
            if left_dx < 0 {
                continue;
            }
            let mut s = (sx - left_dx);
            let mut e = (sx + left_dx);

            if s > sz || e < 0 {
                continue;
            }

            if s < 0 {
                s = 0;
            }
            if e > sz {
                e = sz
            }
            ranges.push((s, e));
        }
        ranges.sort();

        println!("{:?}", ranges);

        let mut range = ranges[0];

        if range.0 != 0 {
            return (range.0 * 4000000 + y) as usize;
        }

        for (s, e) in ranges[1..].iter() {
            if *s > range.1 + 1 {
                println!("distress : {} / {}", range.1 + 1, y);
                return ((range.1 + 1) * 4000000 + y) as usize;
            }

            if *e > range.1 {
                range.1 = *e;
            }
        }

        if range.1 != sz {
            return (range.1 * 4000000 + y) as usize;
        }
    }

    panic!()
}

fn quiz2() -> usize {
    let input = read_file(FILE_NAME);
    proc2(&input, 4000000)
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
            load(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3"
            ),
            (
                vec![(2, 18), (9, 16), (13, 2)],
                vec![(-2, 15), (10, 16), (15, 3)]
            )
        );
    }

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT, 10), 26);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 5461729);
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT, 20), 56000011);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 10621647166538);
    }
}
