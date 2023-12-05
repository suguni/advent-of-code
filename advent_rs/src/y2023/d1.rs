#![allow(dead_code)]

use crate::read_file;
use nom::{FindSubstring, Slice};
use std::iter::{Enumerate, FilterMap, Iterator};
use std::ops::Index;
use std::str::Chars;

const QUIZ_INPUT: &str = include_str!("../../data/2023/input1.txt");

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn quiz1() -> u32 {
    cal_values(&QUIZ_INPUT, to_ns1)
}

fn quiz2() -> u32 {
    cal_values(&QUIZ_INPUT, to_ns2)
}

fn cal_values<F: Fn(&str) -> Box<dyn Iterator<Item = u32> + '_>>(data: &str, ns_iter: F) -> u32 {
    data.lines()
        .map(move |line| {
            let mut ns = ns_iter(line);
            let (v1, v2) = extract(ns);
            v1 * 10 + v2
        })
        .sum()
}

fn extract(mut ns: Box<dyn Iterator<Item = u32> + '_>) -> (u32, u32) {
    let v1 = ns.next().unwrap();
    let v2 = ns.last().or(Some(v1)).unwrap();
    (v1, v2)
}

fn to_ns1(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    Box::new(line.chars().filter_map(|c| c.to_digit(10)))
}

fn to_ns2(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    let mut ns = line.chars().enumerate().filter_map(move |(i, c)| {
        c.to_digit(10).or_else(|| {
            DIGITS
                .iter()
                .enumerate()
                .find(|&(idx, &s)| line[i..].starts_with(s))
                .map(|(idx, _)| (idx + 1) as u32)
        })
    });

    Box::new(ns)
}

fn start_with_digits(chars: String) -> Option<usize> {
    DIGITS
        .iter()
        .enumerate()
        .find(|&(idx, &s)| chars.starts_with(s))
        .map(|(idx, _)| idx)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(cal_values(data, to_ns1), 142);
    }

    #[test]
    fn test_to_num2_extractor() {
        assert_eq!(extract(to_ns2("twone")), (2, 1));
        assert_eq!(extract(to_ns2("two1nine")), (2, 9));
        assert_eq!(extract(to_ns2("sixrctqxdpkxpfdkglvthreenine47rzs")), (6, 7));
        assert_eq!(extract(to_ns2("7c")), (7, 7));
        assert_eq!(
            extract(to_ns2("eighthjbqsbz6ndpkdlnpmpxqvpmsrbvksnnleightnzxmjg")),
            (8, 8)
        );
        assert_eq!(
            extract(to_ns2("mcqcmxxzcmpzrz4ntgnsgqbqjmkzpqvxtvsixrzzr3seven")),
            (4, 7)
        );
    }

    #[test]
    fn test2() {
        let data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(cal_values(data, to_ns2), 281);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 54877);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 54100);
    }
}
