use std::collections::HashSet;

use crate::read_file;

fn sum_priorities(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let len = line.chars().count();
            let (v1, v2): (Vec<_>, Vec<_>) =
                line.chars().enumerate().partition(|(i, _)| *i < len / 2);
            let s1: HashSet<_> = v1.iter().map(|(_, c)| c).collect();
            let s2: HashSet<_> = v2.iter().map(|(_, c)| c).collect();
            let p = **s1.intersection(&s2).next().unwrap();
            priority(p)
        })
        .sum()
}

fn priority(p: char) -> i32 {
    if p.is_uppercase() {
        (p as u8 - 'A' as u8) as i32 + 27
    } else {
        (p as u8 - 'a' as u8) as i32 + 1
    }
}

fn quiz1() -> i32 {
    let input = read_file("data/2022/input3.txt");
    sum_priorities(&input)
}

fn sum_priorities2(input: &str) -> i32 {
    input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut acc, (index, item)| {
            if index % 3 == 0 {
                acc.push(Vec::new());
            }
            let i = acc.len() - 1;
            acc[i].push(item.chars().collect::<HashSet<char>>());
            acc
        })
        .iter()
        .map(|ls| {
            let is1 = ls[0]
                .intersection(&ls[1])
                .map(|c| *c)
                .collect::<HashSet<_>>();
            let p = *is1.intersection(&ls[2]).next().unwrap();
            priority(p)
        })
        .sum()
}

fn quiz2() -> i32 {
    let input = read_file("data/2022/input3.txt");
    sum_priorities2(&input)
}

#[test]
fn test_sum_priorities() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(sum_priorities(input), 157);
}

#[test]
fn test_quiz1() {
    assert_eq!(quiz1(), 7850);
}

#[test]
fn test_sum_priorities2() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(sum_priorities2(input), 70);
}

#[test]
fn test_quiz2() {
    assert_eq!(quiz2(), 2581);
}
