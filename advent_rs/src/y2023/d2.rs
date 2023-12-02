#![allow(dead_code)]

use crate::read_file;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{digit0, digit1, multispace0, newline, space0, space1};
use nom::combinator::{map, map_res, recognize};
use nom::error::ErrorKind;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, separated_pair};
use nom::{FindSubstring, IResult, Slice};
use std::cmp::max;
use std::error::Error;
use std::iter::{Enumerate, FilterMap, Iterator};
use std::num::ParseIntError;
use std::ops::Index;
use std::str::{Chars, FromStr};

const QUIZ_INPUT: &str = include_str!("../../data/2023/f2.txt");

fn quiz1() -> u32 {
    solve1(QUIZ_INPUT, (12, 13, 14))
}

fn quiz2() -> u32 {
    solve2(QUIZ_INPUT)
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    cubes: Vec<(u32, u32, u32)>,
}

fn load(data: &str) -> Vec<Game> {
    let (_, gs) = game_list_parser(data).unwrap();
    gs
}

fn solve1(data: &str, (br, bg, bb): (u32, u32, u32)) -> u32 {
    load(data)
        .iter()
        .filter_map(|game| {
            if (game
                .cubes
                .iter()
                .all(|&(ar, ag, ab)| ar <= br && ag <= bg && ab <= bb))
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn solve2(data: &str) -> u32 {
    load(data)
        .iter()
        .map(|game| {
            let (r, g, b) =
                game.cubes
                    .iter()
                    .fold((0_u32, 0_u32, 0_u32), |(ar, ag, ab), &(r, g, b)| {
                        (max::<u32>(ar, r), max::<u32>(ag, g), max::<u32>(ab, b))
                    });
            r * g * b
        })
        .sum()
}

fn game_id_parser(line: &str) -> IResult<&str, u32> {
    map_res(
        separated_pair(tag("Game"), space1, digit1),
        |(_, id): (_, &str)| id.parse::<u32>(),
    )(line)
}

fn color_parser(line: &str) -> IResult<&str, (u32, u32, u32)> {
    map_res(
        separated_pair(digit1, space1, alt((tag("red"), tag("green"), tag("blue")))),
        |(count, color): (&str, &str)| -> Result<(u32, u32, u32), ParseIntError> {
            let count = count.parse::<u32>()?;
            Ok(match color {
                "red" => (count, 0, 0),
                "green" => (0, count, 0),
                "blue" => (0, 0, count),
                _ => (0, 0, 0),
            })
        },
    )(line)
}

fn cube_parser(line: &str) -> IResult<&str, (u32, u32, u32)> {
    map_res(
        separated_list0(preceded(tag(","), space0), color_parser),
        |res: Vec<(u32, u32, u32)>| -> Result<(u32, u32, u32), ErrorKind> {
            Ok(res
                .iter()
                .fold((0, 0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1, acc.2 + v.2)))
        },
    )(line)
}

fn cube_list_parser(line: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    separated_list0(preceded(tag(";"), space0), cube_parser)(line)
}

fn game_parser(line: &str) -> IResult<&str, Game> {
    map_res(
        separated_pair(game_id_parser, preceded(tag(":"), space0), cube_list_parser),
        |(id, cubes)| -> Result<Game, ErrorKind> { Ok(Game { id, cubes }) },
    )(line)
}

fn game_list_parser(line: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, game_parser)(line)
}

#[cfg(test)]
mod tests {

    use super::*;

    const DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn game_id_parse_test() {
        let line = DATA.lines().next().unwrap();
        let (_, id) = game_id_parser(line).unwrap();
        assert_eq!(id, 1);
    }

    #[test]
    fn color_parse_test() {
        let (_, (r, g, b)) = color_parser("3 blue").unwrap();
        assert_eq!(b, 3);

        let (_, (r, g, b)) = color_parser("3 green").unwrap();
        assert_eq!(g, 3);

        let (_, (r, g, b)) = color_parser("3 red").unwrap();
        assert_eq!(r, 3);

        let r = color_parser("3 cyan");
        assert!(r.is_err());
    }

    #[test]
    fn cube_parse_test() {
        let (_, cube) = cube_parser("1 blue, 2 green").unwrap();
        assert_eq!(cube, (0, 2, 1));

        let (_, cube) = cube_parser("8 green, 6 blue, 20 red").unwrap();
        assert_eq!(cube, (20, 8, 6));
    }

    #[test]
    fn cube_list_parse_test() {
        let (_, cubes) =
            cube_list_parser("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap();
        assert_eq!(cubes, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]);
    }

    #[test]
    fn game_parse_test() {
        let (_, game) =
            game_parser("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
                .unwrap();
        assert_eq!(
            game,
            Game {
                id: 3,
                cubes: vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]
            }
        );
    }

    #[test]
    fn solve1_test() {
        assert_eq!(solve1(DATA, (12, 13, 14)), 8);
    }

    #[test]
    fn quiz1_test() {
        assert_eq!(quiz1(), 2061);
    }

    #[test]
    fn solve2_test() {
        assert_eq!(solve2(DATA), 2286);
    }

    #[test]
    fn quiz2_test() {
        assert_eq!(quiz2(), 72596);
    }
}
