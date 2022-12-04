use std::collections::{HashMap, HashSet};

use crate::read_file;

fn letter_scores() -> HashMap<char, usize> {
    ('a'..='z')
        .chain(('A'..='Z'))
        .enumerate()
        .map(|(idx, letter)| (letter, idx + 1))
        .collect()
}

fn sum_priorities(input: &str) -> i32 {
    let letter_scores = letter_scores();

    input
        .lines()
        .map(|line| {
            let len = line.len();
            let v1 = &line[..len / 2];
            let v2 = &line[len / 2..];
            let found = v1.chars().find(|c| v2.contains(*c)).unwrap();
            letter_scores.get(&found).unwrap()
        })
        .sum::<usize>() as i32
}

fn quiz1() -> i32 {
    let input = read_file("data/2022/input3.txt");
    sum_priorities(&input)
}

fn sum_priorities2(input: &str) -> i32 {
    let letter_scores = letter_scores();
    input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut acc, (index, item)| {
            if index % 3 == 0 {
                acc.push(Vec::new());
            }
            let i = acc.len() - 1;
            acc[i].push(item);
            acc
        })
        .iter()
        .map(|ls| {
            let found = ls[0]
                .chars()
                .find(|c| ls[1].contains(*c) && ls[2].contains(*c))
                .unwrap();
            letter_scores.get(&found).unwrap()
        })
        .sum::<usize>() as i32
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
