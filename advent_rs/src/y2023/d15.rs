use crate::y2021::d20::index;
use itertools::{fold, Itertools};
use std::collections::LinkedList;
use std::str::FromStr;

const INPUT: &str = include_str!("../../data/2023/input15.txt");

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| (acc + c as u32) * 17 % 256)
}

fn solve1(data: &str) -> u32 {
    data.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|step| hash(step))
        .sum()
}

#[derive(Debug)]
enum Op<'a> {
    Dash(&'a str),
    Eq(&'a str, u32),
}

fn lens(s: &str) -> Op {
    if s.ends_with('-') {
        Op::Dash(&s[..s.len() - 1])
    } else {
        let mut ss = s.split('=');
        Op::Eq(
            ss.next().unwrap(),
            u32::from_str(ss.next().unwrap()).unwrap(),
        )
    }
}

fn solve2(data: &str) -> u32 {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];

    for s in data.lines().next().unwrap().split(',').into_iter() {
        let op = lens(s);
        match op {
            Op::Dash(label) => {
                let slot = hash(label) as usize;
                if let Some(mut vs) = boxes.get_mut(slot) {
                    if let Some((idx, _)) = vs.iter().find_position(|(p, _)| *p == label) {
                        vs.remove(idx);
                    }
                }
            }
            Op::Eq(label, focal) => {
                let slot = hash(label) as usize;
                if let Some(mut vs) = boxes.get_mut(slot) {
                    if let Some((idx, (_, old_focal))) =
                        vs.iter_mut().find_position(|(p, _)| *p == label)
                    {
                        *old_focal = focal;
                    } else {
                        vs.push((label, focal));
                    }
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_no, vs)| {
            vs.iter()
                .enumerate()
                .map(|(slot_no, (_, f))| (box_no as u32 + 1) * (slot_no as u32 + 1) * *f)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);

        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("cm"), 0);
        assert_eq!(hash("qp"), 1);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 1320);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 516469);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE), 145);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT), 221627);
    }
}
