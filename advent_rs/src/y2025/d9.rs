use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct P(u64, u64);

impl P {
    fn area(&self, p: &P) -> u64 {
        (self.0.abs_diff(p.0) + 1) * (self.1.abs_diff(p.1) + 1)
    }

    fn dist(&self, p: &P) -> u64 {
        let d1 = self.0.abs_diff(p.0);
        let d2 = self.1.abs_diff(p.1);
        d1 * d1 + d2 * d2
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct S {
    start: P,
    end: P,
    is_vertical: bool,
    range: (u64, u64),
}

impl S {
    fn new(start: P, end: P) -> Self {
        let is_vertical = start.0 == end.0;
        let range = if is_vertical {
            (u64::min(start.1, end.1), u64::max(start.1, end.1))
        } else {
            (u64::min(start.0, end.0), u64::max(start.0, end.0))
        };
        Self {
            start,
            end,
            is_vertical,
            range,
        }
    }

    fn contains(&self, p: &P) -> bool {
        if self.is_vertical {
            self.start.0 == p.0 && (self.range.0 <= p.1 && p.1 <= self.range.1)
        } else {
            self.start.1 == p.1 && (self.range.0 <= p.0 && p.0 <= self.range.1)
        }
    }

    fn intersect(&self, pt: &P) -> u32 {
        assert!(self.is_vertical);
        let x = self.start.0;
        if x > pt.0 || pt.1 < self.range.0 || pt.1 > self.range.1 {
            0
        } else {
            if pt.1 == self.range.0 || pt.1 == self.range.1 {
                1
            } else {
                2
            }
        }
    }
}

const QUIZ_INPUT: &str = include_str!("../../data/2025/input9.txt");

fn quiz1() -> u64 {
    solve1(QUIZ_INPUT)
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
            P(nums[0], nums[1])
        })
        .collect()
}

fn segments(pts: &[P]) -> Vec<S> {
    let mut result = vec![];
    for i in 1..pts.len() {
        result.push(S::new(pts[i - 1], pts[i]));
    }
    result.push(S::new(pts[pts.len() - 1], pts[0]));
    result
}

fn areas(pts: &[P]) -> Vec<(u64, P, P)> {
    let mut result = vec![];
    for i in 0..pts.len() - 1 {
        let p1 = &pts[i];
        for j in i + 1..pts.len() {
            let p2 = &pts[j];
            let d = p1.area(p2);
            result.push((d, *p1, *p2));
        }
    }
    result
}

fn solve1(input: &str) -> u64 {
    let pts = parse_data(input);

    areas(&pts)
        .iter()
        .sorted_by_key(|(a, _, _)| *a)
        .rev()
        .next()
        .unwrap()
        .0
}

fn solve2(input: &str) -> u64 {
    let pts = parse_data(input);
    let segments = segments(&pts);
    let areas = areas(&pts);

    areas
        .iter()
        .sorted_by_key(|(a, _, _)| *a)
        .rev()
        .find_or_first(|(area, p1, p2)| {
            let x1 = u64::min(p1.0, p2.0);
            let x2 = u64::max(p1.0, p2.0);
            let y1 = u64::min(p1.1, p2.1);
            let y2 = u64::max(p1.1, p2.1);

            for x in x1..=x2 {
                for y in y1..=y2 {
                    let c = contains(&segments, &P(x, y));
                    if !c {
                        return false;
                    }
                }
            }

            true
            // let c1 = P(p1.0, p2.1);
            // let c2 = P(p2.0, p1.1);
            // let cn1 = contains(&segments, &c1);
            // let cn2 = contains(&segments, &c2);
            // println!("{:?},{:?}={area} -> {:?}({cn1}),{:?}({cn2})", p1, p2, c1, c2);
            // cn1 && cn2
        })
        .unwrap()
        .0
}

fn contains(segments: &[S], p: &P) -> bool {
    if let Some(_) = segments.iter().find(|&s|s.contains(p)) {
        true
    } else {
        let sum: u32 = segments.iter()
            .filter(|&s| s.is_vertical)
            .map(|&s| {
                let v = s.intersect(p);
                // if v != 0 {println!("{p:?} -> {s:?}, {v}");}
                v
            })
            .sum::<u32>();
        (sum / 2) % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    use super::*;

    #[test]
    fn test_parse() {
        let pts = parse_data(SAMPLE);
        assert_eq!(pts[0], P(7, 1));
        assert_eq!(pts[1], P(11, 1));
    }

    #[test]
    fn test_area() {
        assert_eq!(P(2, 5).area(&P(11, 1)), 50);
    }

    #[test]
    fn test_segment() {
        let s_v = S::new(P(11,1), P(11,7));
        assert!(s_v.is_vertical);

        assert!(!s_v.contains(&P(11, 0)));
        assert!(s_v.contains(&P(11, 1)));
        assert!(s_v.contains(&P(11, 4)));
        assert!(s_v.contains(&P(11, 7)));
        assert!(!s_v.contains(&P(11, 8)));
        assert!(!s_v.contains(&P(10, 4)));

        let s_h = S::new(P(7,1), P(11,1));
        assert!(!s_h.is_vertical);

        assert!(!s_h.contains(&P(6, 1)));
        assert!(s_h.contains(&P(7, 1)));
        assert!(s_h.contains(&P(8, 1)));
        assert!(s_h.contains(&P(11, 1)));
        assert!(!s_h.contains(&P(12, 1)));
        assert!(!s_h.contains(&P(8, 2)));
    }

    #[test]
    fn test_intersect() {
        assert_eq!(S::new(P(2, 5), P(2, 2)).intersect(&P(9, 3)), 2);
        assert_eq!(S::new(P(2, 5), P(2, 3)).intersect(&P(9, 3)), 1);
        assert_eq!(S::new(P(2, 5), P(2, 4)).intersect(&P(9, 3)), 0);

        // let s_h = S::new(P(7,1), P(11,1));
        // let s_v = S::new(P(11,1), P(11,7));
        // assert!(s_v.intersect(&s_h));
        //
        // let s_h = S::new(P(7,1), P(10,1));
        // let s_v = S::new(P(11,1), P(11,7));
        // assert!(!s_v.intersect(&s_h));
        //
        // let s_h = S::new(P(12,2), P(7,2));
        // let s_v = S::new(P(11,1), P(11,7));
        // assert!(s_v.intersect(&s_h));
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 50);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 4748769124);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 24);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 0);
    }
}
