use regex::Regex;
use std::{collections::HashSet, iter::FromIterator};

fn parse_line(line: &str) -> Vec<String> {
    let re = Regex::new(r"\w+").unwrap();
    let mut words = Vec::with_capacity(14);

    for cap in re.captures_iter(line) {
        words.push(cap[0].to_string());
    }

    words
}

pub fn quiz1(text: &str) -> usize {
    text.lines()
        .map(parse_line)
        .flat_map(|vs| vs[10..].iter().map(|v| v.len()).collect::<Vec<_>>())
        .filter(|c| *c == 2 || *c == 4 || *c == 3 || *c == 7)
        .count()
}
// count
// 0 : 6
// 1 : 2*
// 2 : 5
// 3 : 5
// 4 : 4*
// 5 : 5
// 6 : 6
// 7 : 3*
// 8 : 7*
// 9 : 6

// cardinality
// 2 : {1}
// 3 : {7}
// 4 : {4}
// 5 : {2, 3, 5}
// 6 : {0, 6, 9}
// 7 : {8}

fn collect_by_count(input: &[String], len: usize) -> Vec<HashSet<char>> {
    input
        .iter()
        .filter(|s| s.len() == len)
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .collect::<Vec<HashSet<char>>>()
}

fn deduction(input: Vec<String>) -> i32 {
    let n1 = collect_by_count(&input[..10], 2).pop().unwrap();
    let n7 = collect_by_count(&input[..10], 3).pop().unwrap();
    let n4 = collect_by_count(&input[..10], 4).pop().unwrap();
    let n8 = collect_by_count(&input[..10], 7).pop().unwrap();

    let mut c5s = collect_by_count(&input[..10], 5);
    let mut c6s = collect_by_count(&input[..10], 6);

    // 1 - {5} -> 공집합 인것 3
    // 4 - {5} -> 원소가 한개 5
    //                        2
    let i = c5s
        .iter()
        .position(|nx| n1.difference(nx).count() == 0)
        .unwrap();
    let n3 = c5s.remove(i);

    let i = c5s
        .iter()
        .position(|nx| n4.difference(nx).count() == 1)
        .unwrap();
    let n5 = c5s.remove(i);

    let n2 = c5s.pop().unwrap();

    // 1 - {6} -> 공집합이 아닌 것 6 (c)
    // 4 - {6} ->                  0 (d)
    //                             9
    let i = c6s
        .iter()
        .position(|nx| n1.difference(nx).next() != None)
        .unwrap();
    let n6 = c6s.remove(i);

    let i = c6s
        .iter()
        .position(|nx| n4.difference(nx).next() != None)
        .unwrap();
    let n0 = c6s.remove(i);

    let n9 = c6s.pop().unwrap();

    let nums = vec![n0, n1, n2, n3, n4, n5, n6, n7, n8, n9];

    input[10..]
        .iter()
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .map(|output| nums.iter().position(|nx| nx.eq(&output)).unwrap() as i32)
        .fold(0, |acc, v| acc * 10 + v)
}

pub fn quiz2(text: &str) -> i32 {
    text.lines()
        .map(parse_line)
        .map(|input| deduction(input))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const LINE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";

    #[test]
    fn test_parse() {
        let words = parse_line(LINE);
        assert_eq!(words.len(), 14);
    }

    #[test]
    fn run_y2021_d8_quiz1() {
        let text = read_file("data/2021/input8.txt");
        assert_eq!(quiz1(text.as_str().trim()), 264);
    }

    #[test]
    fn test_deduction() {
        let vs = parse_line(LINE);
        assert_eq!(deduction(vs), 8394);
    }

    #[test]
    fn run_y2021_d8_quiz2() {
        let text = read_file("data/2021/input8.txt");
        assert_eq!(quiz2(text.as_str().trim()), 0);
    }
}
