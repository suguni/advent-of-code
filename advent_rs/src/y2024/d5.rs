use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::i32 as ci32;
use nom::character::complete::{char, multispace1, newline};
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::collections::HashMap;

const QUIZ_INPUT: &str = include_str!("../../data/2024/input5.txt");

fn parse_rules(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(newline, separated_pair(ci32, char('|'), ci32))(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, separated_list1(tag(","), ci32))(input)
}

fn parse_data(input: &str) -> IResult<&str, (Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    separated_pair(parse_rules, multispace1, parse_updates)(input)
}

fn rules_to_map(rules: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    rules.iter().fold(HashMap::new(), |mut acc, (a, b)| {
        let vs = acc.entry(*a).or_insert(vec![]);
        vs.push(*b);
        // vs.sort();
        acc
    })
}

fn check(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..(update.len() - 1) {
        if let Some(rs) = rules.get(&update[i]) {
            if !update[(i + 1)..].iter().all(|&x| rs.contains(&x)) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn solve1(input: &str) -> i32 {
    let (_, (rules, updates)) = parse_data(input).unwrap();
    let rules = rules_to_map(rules);
    updates.iter().filter(|&us| check(us, &rules)).map(|us| {
        // dbg!(us);
        us[us.len() / 2]
    }).sum()
}

fn quiz1() -> i32 {
    solve1(QUIZ_INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse() {
        let input = "75|13
53|13

75,47,61,53,29";

        assert_eq!(
            parse_data(input).unwrap(),
            (
                "",
                (vec![(75, 13), (53, 13)], vec![vec![75, 47, 61, 53, 29]])
            )
        );
    }

    #[test]
    fn test_rules_to_map() {
        let result = rules_to_map(vec![
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
        ]);

        let expected = HashMap::from([(97, vec![13, 61, 47]), (75, vec![29, 53]), (61, vec![13])]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_check() {
        let (_, (rules, _)) = parse_data(SAMPLE).unwrap();
        let rules = rules_to_map(rules);
        assert!(check(&vec![75, 47, 61, 53, 29], &rules));
        assert_eq!(check(&vec![61, 13, 29], &rules), false);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 143);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 4609);
    }
}
