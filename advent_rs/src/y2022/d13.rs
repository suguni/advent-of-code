use colored::Colorize;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{interpolate_color, read_file};

const FILE_NAME: &str = "data/2022/input13.txt";


fn load(input: &str) {
    todo!();
}

fn proc1(input: &str) {
    todo!()
}


fn quiz1() {
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
    #[ignore]
    fn test_load() {
        let grid = load(INPUT);
        assert_eq!(grid, ());
    }

    #[test]
    #[ignore]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), ());
    }

    #[test]
    #[ignore]
    fn test_quiz1() {
        assert_eq!(quiz1(), ());
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
