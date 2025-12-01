use nom::character::complete::{alphanumeric1, anychar, newline, space1};
use nom::combinator::map;
use nom::multi::{count, separated_list1};
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../data/2023/input7.txt");

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

fn solve1(data: &str) -> u32 {
    let (_, vs) = load(data).unwrap();
    let mut vs = vs
        .into_iter()
        .map(|(hands, bids)| (type_of(hands), to_nums(hands), bids))
        .collect();
    total_winnings(vs)
}

fn solve2(data: &str) -> u32 {
    let (_, vs) = load(data).unwrap();
    let mut vs = vs
        .into_iter()
        .map(|(hands, bids)| (type_of_with_joker(hands), to_nums_with_joker(hands), bids))
        .collect();
    total_winnings(vs)
}

fn total_winnings(mut vs: Vec<(Type, Vec<u32>, u32)>) -> u32 {
    vs.sort_by(|(t1, v1, _), (t2, v2, _)| {
        let o = t1.cmp(t2);
        if o == Ordering::Equal {
            v1.cmp(v2)
        } else {
            o
        }
    });

    let max = vs.len();

    vs.iter()
        .enumerate()
        .map(|(rank, (_, _, bids))| ((rank + 1) as u32) * *bids)
        .sum()
}

fn type_of(hands: &str) -> Type {
    let mut letters = HashMap::new();
    for c in hands.chars() {
        letters.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut counts: Vec<u32> = letters.values().map(|v| *v).collect();
    counts.sort_by(|a, b| b.cmp(a));

    type_of_counts(&counts)
}

fn to_nums(hands: &str) -> Vec<u32> {
    hands
        .chars()
        .map(|c| {
            if c.is_numeric() {
                c.to_digit(10).unwrap()
            } else {
                match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => 0,
                }
            }
        })
        .collect()
}

fn load(data: &str) -> IResult<&str, Vec<(&str, u32)>> {
    separated_list1(
        newline,
        separated_pair(alphanumeric1, space1, nom::character::complete::u32),
    ).parse(data)
}

fn type_of_with_joker(hands: &str) -> Type {
    let mut letters = HashMap::new();
    let mut joker_count = 0_u32;

    for c in hands.chars() {
        if c == 'J' {
            joker_count += 1;
        } else {
            letters.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    let counts = if letters.is_empty() {
        vec![joker_count]
    } else {
        let mut counts: Vec<u32> = letters.values().map(|v| *v).collect();
        counts.sort_by(|a, b| b.cmp(a));
        counts[0] += joker_count;
        counts
    };

    type_of_counts(&counts)
}

fn type_of_counts(counts: &Vec<u32>) -> Type {
    if counts[0] == 1 {
        Type::High
    } else if counts[0] == 2 {
        if counts[1] == 1 {
            Type::One
        } else {
            Type::Two
        }
    } else if counts[0] == 3 {
        if counts[1] == 1 {
            Type::Three
        } else {
            Type::Full
        }
    } else if counts[0] == 4 {
        Type::Four
    } else {
        Type::Five
    }
}

fn to_nums_with_joker(hands: &str) -> Vec<u32> {
    hands
        .chars()
        .map(|c| {
            if c.is_numeric() {
                c.to_digit(10).unwrap()
            } else {
                match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    _ => 0,
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_typeof() {
        assert_eq!(type_of("23456"), Type::High);
        assert_eq!(type_of("32T3K"), Type::One);
        assert_eq!(type_of("T55J5"), Type::Three);
        assert_eq!(type_of("KK677"), Type::Two);
        assert_eq!(type_of("KTJJT"), Type::Two);
        assert_eq!(type_of("QJQJJ"), Type::Full);
        assert_eq!(type_of("JJOJJ"), Type::Four);
        assert_eq!(type_of("JJJJJ"), Type::Five);
    }

    #[test]
    fn test_type_of_with_joker() {
        assert_eq!(type_of_with_joker("32T3K"), Type::One);
        assert_eq!(type_of_with_joker("KK677"), Type::Two);

        assert_eq!(type_of_with_joker("T55J5"), Type::Four);
        assert_eq!(type_of_with_joker("KTJJT"), Type::Four);
        assert_eq!(type_of_with_joker("QQQJA"), Type::Four);

        assert_eq!(type_of_with_joker("23456"), Type::High);
        assert_eq!(type_of_with_joker("JJOJJ"), Type::Five);
        assert_eq!(type_of_with_joker("JJJJJ"), Type::Five);
    }

    #[test]
    fn test_load() {
        let (_, res) = load(
            "32T3K 765
T55J5 684",
        )
        .unwrap();

        assert_eq!(res, vec![("32T3K", 765), ("T55J5", 684),]);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 6440);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 253638586);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE), 5905);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT), 253253225);
    }
}
