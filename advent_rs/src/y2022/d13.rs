use colored::Colorize;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{interpolate_color, read_file};

const FILE_NAME: &str = "data/2022/input13.txt";

#[derive(Debug, PartialEq)]
enum PackData {
    V(u32),
    L(Vec<PackData>),
}

use PackData::*;

type Packet = Vec<PackData>;

fn parse_line(line: &str) -> Packet {
    let mut packet = vec![];
    let cs = line.chars().collect::<Vec<char>>();
    for &c in cs[1..(cs.len()-1)].iter() {
        match c {
            '[' => {}
            ',' => {}
            ']' => {}
            '0'..='9' => {}
            _ => panic!()
        }
    }
    let cs: Vec<char> = line.chars().collect();
}

fn load_block(block: &str) -> (Packet, Packet) {
    let mut lines = block.lines();
    (parse_line(lines.next().unwrap()),
     parse_line(lines.next().unwrap()))
}

fn load(input: &str) -> Vec<(Packet, Packet)> {
    input.split("\n\n")
        .map(|block| load_block(block))
        .collect()
}

fn proc1(input: &str) -> u32 {
    todo!()
}


fn quiz1() -> u32 {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(input: &str) {
    todo!()
}

fn quiz2() {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("[1,1,3,1,1]"), vec![V(1), V(1), V(3), V(1), V(1)]);
    }

    #[test]
    #[ignore]
    fn test_load() {
        let packet = load(INPUT);
        assert_eq!(packet, vec![]);
    }

    #[test]
    #[ignore]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 13);
    }

    #[test]
    #[ignore]
    fn test_quiz1() {
        assert_eq!(quiz1(), 0);
    }

    #[test]
    #[ignore]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), ());
    }

    #[test]
    #[ignore]
    fn test_quiz2() {
        assert_eq!(quiz2(), ());
    }
}
