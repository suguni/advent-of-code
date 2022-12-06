use std::collections::HashSet;

use regex::Regex;

use crate::read_file;

fn proc(input: &str, size: usize) -> usize {
    let cs = input.chars().collect::<Vec<char>>();
    let (idx, _) = cs
        .windows(size)
        .enumerate()
        .find(|&(_, xs)| xs.iter().collect::<HashSet<_>>().len() == size)
        .unwrap();
    idx + size
}
fn proc1(input: &str) -> usize {
    proc(input, 4)
}

fn quiz1() -> usize {
    let input = read_file("data/2022/input6.txt");
    proc1(&input)
}

fn proc2(input: &str) -> usize {
    proc(input, 14)
}

fn quiz2() -> usize {
    let input = read_file("data/2022/input6.txt");
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 7);
        assert_eq!(proc1(INPUT1), 5);
        assert_eq!(proc1(INPUT2), 6);
        assert_eq!(proc1(INPUT3), 10);
        assert_eq!(proc1(INPUT4), 11);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 1262);
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 19);
        assert_eq!(proc2(INPUT1), 23);
        assert_eq!(proc2(INPUT2), 23);
        assert_eq!(proc2(INPUT3), 29);
        assert_eq!(proc2(INPUT4), 26);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 3444);
    }
}
