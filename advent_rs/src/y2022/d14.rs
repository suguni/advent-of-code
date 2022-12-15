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

const FILE_NAME: &str = "data/2022/input14.txt";

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

#[derive(Debug, PartialEq)]
struct Cave {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
    inf: bool,
    rocks: HashMap<u32, HashSet<u32>>,
    base: HashSet<(u32, u32)>,
}

impl Cave {
    fn have_rock(&self, x: u32, y: u32) -> bool {
        if self.inf && y == self.bottom {
            true
        } else {
            if let Some(hs) = self.rocks.get(&y) {
                hs.contains(&x)
            } else {
                false
            }
        }
    }

    fn add_sand(&mut self, x: u32, y: u32) {
        if y < self.top {
            self.top = y;
        }
        if x < self.left {
            self.left = x;
        } else if x > self.right {
            self.right = x;
        }
        self.rocks.entry(y).or_default().insert(x);
    }

    fn is_safe(&self, x: u32, y: u32) -> bool {
        self.inf || (x >= self.left && x <= self.right && y <= self.bottom)
    }

    fn draw(&self) {
        let mut view = String::new();
        let width = (self.right - self.left + 1) as usize;
        let empty = ".".repeat(width);
        let base_line = "#".repeat(width);
        let bottom = if self.inf {
            self.bottom - 1
        } else {
            self.bottom
        };
        for y in 0..=bottom {
            view.push_str(&format!("{:2} ", y));
            if let Some(hs) = self.rocks.get(&y) {
                for x in self.left..=self.right {
                    if hs.contains(&x) {
                        if self.base.contains(&(x, y)) {
                            view.push('#');
                        } else {
                            view.push('o');
                        }
                    } else {
                        view.push('.');
                    }
                }
            } else {
                view.push_str(&empty);
            }
            view.push('\n');
        }

        if self.inf {
            view.push_str(&format!("{:3} ", self.bottom));
            view.push_str(&base_line);
            view.push('\n');
        }

        println!("{view}");
    }
}

fn to_cave(vss: Vec<Vec<(u32, u32)>>, inf: bool) -> Cave {
    let mut rocks: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut bound = (u32::MAX, u32::MAX, u32::MIN, u32::MIN);

    for vs in vss.iter() {
        for xs in vs.windows(2) {
            let (x1, y1) = xs[0];
            let (x2, y2) = xs[1];

            update_bound(&mut bound, &xs);

            if y1 == y2 {
                for x in range_vec(x1, x2) {
                    rocks.entry(y1).or_default().insert(x);
                }
            } else if x1 == x2 {
                for y in range_vec(y1, y2) {
                    rocks.entry(y).or_default().insert(x1);
                }
            } else {
                panic!();
            }
        }
    }
    let bottom = bound.3 + if inf { 2 } else { 0 };

    let base = rocks
        .iter()
        .flat_map(|(&y, v)| v.iter().map(move |&x| (x, y)))
        .collect::<HashSet<(u32, u32)>>();

    Cave {
        left: bound.0,
        top: bound.1,
        right: bound.2,
        bottom,
        inf,
        rocks,
        base,
    }
}

fn range_vec(x1: u32, x2: u32) -> Vec<u32> {
    if x1 < x2 { (x1..=x2) } else { (x2..=x1) }.collect::<Vec<_>>()
}

fn update_bound((left, top, right, bottom): &mut (u32, u32, u32, u32), xs: &[(u32, u32)]) {
    let (x1, y1) = xs[0];
    let (x2, y2) = xs[1];
    if *left > x1 {
        *left = x1;
    }
    if *left > x2 {
        *left = x2;
    }
    if *right < x1 {
        *right = x1;
    }
    if *right < x2 {
        *right = x2;
    }
    if *top > y1 {
        *top = y1;
    }
    if *top > y2 {
        *top = y2;
    }
    if *bottom < y1 {
        *bottom = y1;
    }
    if *bottom < y2 {
        *bottom = y2;
    }
}

fn fall_1(mut cave: Cave, init_x: u32, init_y: u32) -> u32 {
    let mut sx = init_x;
    let mut sy = init_y;
    let mut count = 0;

    while cave.is_safe(sx, sy) {
        let bb = cave.have_rock(sx, sy + 1);
        if !bb {
            sy += 1;
        } else {
            let bl = cave.have_rock(sx - 1, sy + 1);
            let br = cave.have_rock(sx + 1, sy + 1);
            if bb && !bl {
                sx -= 1;
                sy += 1;
            } else if bb && bl && !br {
                sx += 1;
                sy += 1;
            } else if bb && bl && br {
                cave.add_sand(sx, sy);
                count += 1;
                sx = init_x;
                sy = init_y;
                cave.draw();
            }
        }
    }

    count
}

fn proc1(input: &str) -> u32 {
    let cave = to_cave(load(input), false);
    fall_1(cave, 500, 0)
}

fn quiz1() -> u32 {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn fall_2(mut cave: Cave, init_x: u32, init_y: u32) -> u32 {
    let mut sx = init_x;
    let mut sy = init_y;
    let mut count = 0;

    loop {
        let bb = cave.have_rock(sx, sy + 1);
        if !bb {
            sy += 1;
        } else {
            let bl = cave.have_rock(sx - 1, sy + 1);
            let br = cave.have_rock(sx + 1, sy + 1);
            if bb && !bl {
                sx -= 1;
                sy += 1;
            } else if bb && bl && !br {
                sx += 1;
                sy += 1;
            } else if bb && bl && br {
                cave.add_sand(sx, sy);
                cave.draw();
                count += 1;
                if sx == init_x && sy == init_y {
                    break;
                } else {
                    sx = init_x;
                    sy = init_y;
                }
            }
        }
    }

    count
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

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

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
