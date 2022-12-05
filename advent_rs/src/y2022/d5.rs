use regex::Regex;

use crate::read_file;

#[derive(Debug, Eq, PartialEq)]
struct Op {
    count: usize,
    from: usize,
    to: usize,
}

impl Op {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Op { count, from, to }
    }
}

type Stacks = Vec<Vec<String>>;
type Ops = Vec<Op>;

fn parse_input(input: &str) -> (Stacks, Ops) {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    (parse_stack_block(blocks[0]), parse_ops_block(blocks[1]))
}

fn parse_stack_block(block: &str) -> Vec<Vec<String>> {
    let lines = block.lines().collect::<Vec<_>>();
    let stack_max_height = lines.len() - 1;
    let stack_count = lines.last().unwrap().split_whitespace().count();

    let mut stacks: Vec<Vec<String>> = vec![vec!["".to_owned(); stack_max_height]; stack_count];

    for (idx, line) in lines.iter().enumerate() {
        if idx == stack_max_height {
            break;
        }
        parse_stack_line(line, stack_max_height - idx - 1, &mut stacks);
    }

    for stack in stacks.iter_mut() {
        while let Some(elm) = stack.last() {
            if elm.is_empty() {
                stack.pop();
            } else {
                break;
            }
        }
    }

    stacks
}

fn parse_stack_line(line: &str, row: usize, stacks: &mut Vec<Vec<String>>) {
    for (idx, c) in line.chars().enumerate() {
        let col = idx / 4;
        if c.is_alphabetic() {
            stacks[col][row].push(c);
        }
    }
}

fn parse_ops_block(block: &str) -> Vec<Op> {
    let regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    block
        .lines()
        .map(|line| {
            let cas = regex.captures(line).unwrap();
            let count = *&cas[1].parse::<usize>().unwrap();
            let from = *&cas[2].parse::<usize>().unwrap();
            let to = *&cas[3].parse::<usize>().unwrap();
            Op::new(count, from, to)
        })
        .collect()
}

fn proc1(input: &str) -> String {
    let (mut stacks, ops) = parse_input(input);

    for op in ops.iter() {
        for _ in 0..op.count {
            let item = stacks[op.from - 1].pop().unwrap();
            stacks[op.to - 1].push(item);
        }
    }

    stacks.iter().fold(String::new(), |mut acc, vs| {
        acc.push_str(vs.last().unwrap());
        acc
    })
}

fn quiz1() -> String {
    let input = read_file("data/2022/input5.txt");
    proc1(&input)
}

fn proc2(input: &str) -> String {
    let (mut stacks, ops) = parse_input(input);

    for op in ops.iter() {
        let index = stacks[op.to - 1].len();
        for _ in 0..op.count {
            let item = stacks[op.from - 1].pop().unwrap();
            stacks[op.to - 1].insert(index, item);
        }
    }

    stacks.iter().fold(String::new(), |mut acc, vs| {
        acc.push_str(vs.last().unwrap());
        acc
    })
}

fn quiz2() -> String {
    let input = read_file("data/2022/input5.txt");
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "____[D]____
[N] [C]____
[Z] [M] [P]
_1   2   3_

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse() {
        let (stacks, ops) = parse_input(&INPUT.replace("_", " "));
        assert_eq!(
            stacks,
            vec![
                vec!["Z".to_owned(), "N".to_owned()],
                vec!["M".to_owned(), "C".to_owned(), "D".to_owned()],
                vec!["P".to_owned()],
            ],
        );
        assert_eq!(
            ops,
            vec![
                Op::new(1, 2, 1),
                Op::new(3, 1, 3),
                Op::new(2, 2, 1),
                Op::new(1, 1, 2),
            ],
        );
    }

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(&INPUT.replace("_", " ")), "CMZ".to_owned());
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), "SVFDLGLWV".to_owned());
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(&INPUT.replace("_", " ")), "MCD".to_owned());
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), "DCVTCVPCL".to_owned());
    }
}
