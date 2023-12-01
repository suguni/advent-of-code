#![allow(dead_code)]

use crate::read_file;
use nom::{FindSubstring, Slice};
use std::iter::Iterator;
use std::ops::Index;

const QUIZ_INPUT: &str = include_str!("../../data/2023/f1.txt");

fn quiz1() -> u32 {
    cal_values(&QUIZ_INPUT, extract1)
}

fn quiz2() -> u32 {
    cal_values(&QUIZ_INPUT, extract2)
}

fn cal_values(data: &str, tokenizer: fn(&str) -> Vec<u32>) -> u32 {
    data.lines()
        .map(|line| {
            let ns = tokenizer(line);
            let v1 = ns.first().unwrap();
            let v2 = ns.last().unwrap();
            v1 * 10 + v2
        })
        .sum()
}

fn extract1(line: &str) -> Vec<u32> {
    let chars = line.chars().collect::<Vec<char>>();

    let v1 = line
        .chars()
        .find(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .unwrap();

    let v2 = line
        .chars()
        .rfind(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .unwrap();

    vec![v1, v2]
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn extract2(line: &str) -> Vec<u32> {
    let digits: Vec<Vec<char>> = DIGITS.iter().map(|&s| s.chars().collect()).collect();

    let mut result = vec![];

    let chars = line.chars().collect::<Vec<char>>();

    let mut pos = 0_usize;
    loop {
        if chars[pos].is_numeric() {
            result.push(chars[pos].to_digit(10).unwrap());
            break;
        } else {
            let found = start_with_digits(chars[pos..].iter().collect());
            if let Some(idx) = found {
                result.push((idx + 1) as u32);
                break;
            } else {
                if pos == chars.len() - 1 {
                    break;
                } else {
                    pos += 1;
                }
            }
        }
    }

    let mut pos = chars.len() - 1;
    loop {
        if chars[pos].is_numeric() {
            result.push(chars[pos].to_digit(10).unwrap());
            break;
        } else {
            let found = start_with_digits(chars[pos..].iter().collect());
            if let Some(idx) = found {
                result.push((idx + 1) as u32);
                break;
            } else {
                if pos == 0 {
                    break;
                } else {
                    pos -= 1;
                }
            }
        }
    }

    result
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
        assert_eq!(cal_values(data, extract1), 142);
    }

    #[test]
    fn test_extractor() {
        assert_eq!(extract2("twone"), vec![2, 1]);
        assert_eq!(extract2("two1nine"), vec![2, 9]);
        assert_eq!(extract2("sixrctqxdpkxpfdkglvthreenine47rzs"), vec![6, 7]);
        assert_eq!(extract2("7c"), vec![7, 7]);
        assert_eq!(
            extract2("eighthjbqsbz6ndpkdlnpmpxqvpmsrbvksnnleightnzxmjg"),
            vec![8, 8]
        );
        assert_eq!(
            extract2("mcqcmxxzcmpzrz4ntgnsgqbqjmkzpqvxtvsixrzzr3seven"),
            vec![4, 7]
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
        assert_eq!(cal_values(data, extract2), 281);
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
