use nom::branch::permutation;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::{digit1, newline, space0, space1};
use nom::combinator::map_res;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::{IResult, Parser};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

const DATA: &str = include_str!("../../data/2023/input4.txt");

fn quiz1() -> u32 {
    solve1(DATA)
}

fn quiz2() -> u32 {
    solve2(DATA)
}

fn solve1(data: &str) -> u32 {
    let cards = load(data);
    cards
        .iter()
        .map(|card| {
            let c = card.winning_count();
            if c == 0 {
                0
            } else {
                2_u32.pow(c as u32 - 1)
            }
        })
        .sum()
}

fn solve2(data: &str) -> u32 {
    let cards = load(data);

    let mut copyies = vec![0_u32; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let count = card.winning_count();
        if count > 0 {
            let copy = copyies[idx];
            for i in 1..=count {
                copyies[idx + i] += copy + 1;
            }
        }
    }

    copyies.iter().sum::<u32>() + cards.len() as u32
}

fn load(data: &str) -> Vec<Card> {
    let (_, cards) = card_list_parser(data).unwrap();
    cards
}

#[derive(Debug, Eq, PartialEq)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    having: Vec<u32>,
}

impl Card {
    fn winning_count(&self) -> usize {
        let winning_set: HashSet<u32> = HashSet::from_iter(self.winning.iter().cloned());
        let having_set: HashSet<u32> = HashSet::from_iter(self.having.iter().cloned());
        winning_set.intersection(&having_set).count()
    }
}

fn card_list_parser(line: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, card_parser).parse(line)
}

fn card_parser(line: &str) -> IResult<&str, Card> {
    let (line, _) = preceded(tag("Card"), space1).parse(line)?;
    let (line, id) = terminated(nom::character::complete::u32, tag(":")).parse(line)?;
    let (line, _) = space1(line)?;
    let (line, winning) = list_num_parser(line)?;
    let (line, _) = delimited(space1, tag("|"), space1).parse(line)?;
    let (line, having) = list_num_parser(line)?;

    Ok((
        line,
        Card {
            id,
            winning,
            having,
        },
    ))
}

fn list_num_parser(line: &str) -> IResult<&str, Vec<u32>> {
    map_res(
        separated_list0(space1, digit1),
        |vs: Vec<&str>| -> Result<Vec<u32>, _> {
            vs.iter().map(|&v| u32::from_str(v)).into_iter().collect()
        },
    ).parse(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 13);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 19855);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 30);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 10378710);
    }

    #[test]
    fn test_parse_card() {
        let (_, card) = card_parser("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();

        assert_eq!(
            card,
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                having: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }
}
