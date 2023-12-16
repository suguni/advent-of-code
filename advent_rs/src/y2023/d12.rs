use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::min;
use std::ptr::replace;

const INPUT: &str = include_str!("../../data/2023/input12.txt");

fn load_line(line: &str) -> (Vec<char>, Vec<u32>) {
    let (_, result): (_, (Vec<char>, Vec<u32>)) =
        separated_pair(marks_parser, space1, nums_parser)(line).unwrap();
    result
}

fn marks_parser(line: &str) -> IResult<&str, Vec<char>> {
    many1(alt((
        nom::character::complete::char('#'),
        nom::character::complete::char('.'),
        nom::character::complete::char('?'),
    )))(line)
}

fn nums_parser(line: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(
        nom::character::complete::char(','),
        nom::character::complete::u32,
    )(line)
}

fn find_positions(marks: &[char], count: usize) -> Vec<usize> {
    let mut pos = vec![];

    let mut already_sharp_matched = false;

    let end = marks
        .iter()
        .position(|c| *c == '#')
        .map(|p| min(p + count, marks.len()))
        .unwrap_or(marks.len());

    for (index, region) in marks[..end].windows(count).enumerate() {
        let sharp_matched = region
            .iter()
            .map(|c| if *c == '?' { '#' } else { *c })
            .all(|c| c == '#');

        let next_is_not_sharp = index + count == marks.len() || marks[index + count] != '#';

        if sharp_matched && next_is_not_sharp {
            pos.push(index);
        }
    }

    pos
}

fn construct_str(marks: &[char], start: usize, count: usize) -> Vec<char> {
    let mut result = vec![];

    marks[0..start]
        .iter()
        .map(|c| if *c == '?' { '.' } else { *c })
        .for_each(|c| result.push(c));

    marks[start..(start + count)]
        .iter()
        .map(|c| if *c == '?' { '#' } else { *c })
        .for_each(|c| result.push(c));

    marks[(start + count)..]
        .iter()
        .for_each(|c| result.push(*c));

    result
}

fn clean_up(marks: &mut Vec<char>, start: usize) {
    for c in marks.iter_mut() {
        if *c == '?' {
            *c = '.';
        }
    }
}

fn find_and_replace(start: usize, marks: &[char], count: usize) -> Vec<(usize, Vec<char>)> {
    let mut result = vec![];

    let ps = find_positions(&marks[start..], count);

    for pos in ps.iter().map(|p| *p + start).into_iter() {
        result.push((pos + count, construct_str(marks, pos, count)));
    }

    result
}

fn turn_to_next(marks: &mut Vec<char>, mut start: usize) -> usize {
    while start < marks.len() && marks[start - 1] == '#' {
        if marks[start] == '?' {
            marks[start] = '.';
        }
        start += 1;
    }
    start
}

fn find_all(marks: Vec<char>, counts: Vec<u32>) -> Vec<Vec<char>> {
    let mut jobs = vec![(0, marks, &counts[..])];
    let mut result = vec![];

    while let Some((start, marks, counts)) = jobs.pop() {
        let c = counts[0];
        for (mut next_start, mut replaced_marks) in
            find_and_replace(start, &marks, c as usize).into_iter()
        {
            next_start = turn_to_next(&mut replaced_marks, next_start);

            if next_start >= replaced_marks.len() && counts.len() > 1 {
                // drop
            } else if counts.len() == 1 {
                if !replaced_marks[next_start..].contains(&'#') {
                    clean_up(&mut replaced_marks, next_start);
                    result.push(replaced_marks);
                }
            } else {
                jobs.push((next_start, replaced_marks, &counts[1..]));
            }
        }
    }

    result
}

fn find_all_count(marks: Vec<char>, counts: Vec<u32>) -> usize {
    find_all(marks, counts).len()
}

fn solve1(data: &str) -> usize {
    data.lines()
        .map(|line| load_line(line))
        .flat_map(|(marks, counts)| find_all(marks, counts))
        .count()
}

fn solve2(data: &str) -> usize {
    data.lines()
        .map(|line| load_line(line))
        .map(|(marks, counts)| (repeat_marks(marks), counts.repeat(5)))
        .flat_map(|(marks, counts)| find_all(marks, counts))
        .count()
}

fn repeat_marks(marks: Vec<char>) -> Vec<char> {
    itertools::repeat_n(marks, 5)
        .collect::<Vec<Vec<char>>>()
        .join(&'?')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set;
    use std::collections::HashSet;

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 21);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE), 525152);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 7361);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT), 7361);
    }

    #[test]
    fn test_turn_to_next() {
        let mut marks = to_chars(".#?.###");
        let start = turn_to_next(&mut marks, 2);
        assert_eq!(start, 3);
        assert_eq!(marks, to_chars(".#..###"));

        let mut marks = to_chars("#??.###");
        let start = turn_to_next(&mut marks, 1);
        assert_eq!(start, 2);
        assert_eq!(marks, to_chars("#.?.###"));
    }

    #[test]
    fn test_find_and_replace() {
        let line = to_chars("???.###");
        let vs = find_and_replace(0, &line, 1);
        assert_eq!(
            vs,
            vec![
                (1, "#??.###".chars().collect()),
                (2, ".#?.###".chars().collect()),
                (3, "..#.###".chars().collect()),
            ]
        );

        let vs = find_and_replace(2, &to_chars("#??.###"), 1);
        assert_eq!(vs, vec![(3, "#.#.###".chars().collect()),]);

        let vs = find_and_replace(3, &to_chars(".#..###"), 1);
        assert_eq!(vs, vec![]);

        let line = to_chars("?###????????");
        let vs = find_and_replace(0, &line, 3);
        assert_eq!(vs, vec![(4, ".###????????".chars().collect())]);

        let line = to_chars(".###????????");
        let vs = find_and_replace(5, &line, 2);
        assert_eq!(
            vs,
            vec![
                (7, ".###.##?????".chars().collect()),
                (8, ".###..##????".chars().collect()),
                (9, ".###...##???".chars().collect()),
                (10, ".###....##??".chars().collect()),
                (11, ".###.....##?".chars().collect()),
                (12, ".###......##".chars().collect()),
            ]
        );
    }

    #[test]
    fn test_founds() {
        let founds = find_all(to_chars("?###????????"), vec![3, 2, 1])
            .into_iter()
            .map(|vc| vc.iter().collect())
            .collect::<HashSet<String>>();

        assert_eq!(
            founds,
            set![
                ".###.##.#...".to_string(),
                ".###.##..#..".to_string(),
                ".###.##...#.".to_string(),
                ".###.##....#".to_string(),
                ".###..##.#..".to_string(),
                ".###..##..#.".to_string(),
                ".###..##...#".to_string(),
                ".###...##.#.".to_string(),
                ".###...##..#".to_string(),
                ".###....##.#".to_string()
            ]
        );

        let founds = find_all(to_chars("???.###"), vec![1, 1, 3])
            .into_iter()
            .map(|vc| vc.iter().collect())
            .collect::<HashSet<String>>();

        assert_eq!(founds, set!["#.#.###".to_string()]);
    }

    #[test]
    fn test_founds_in_real() {
        let input = to_chars("???.??#????#?.");
        let candidates = find_and_replace(0, &input, 1);
        assert_eq!(
            candidates,
            vec![
                (1, to_chars("#??.??#????#?.")),
                (2, to_chars(".#?.??#????#?.")),
                (3, to_chars("..#.??#????#?.")),
                (5, to_chars("....#?#????#?.")),
                (7, to_chars("......#????#?.")),
            ]
        );

        let mut v = to_chars("#??.??#????#?.");
        let next = turn_to_next(&mut v, 1);
        assert_eq!(next, 2);
        assert_eq!(v, to_chars("#.?.??#????#?."));

        let input = to_chars("#.?.??#????#?.");
        let pos = find_positions(&input[2..], 3);
        assert_eq!(pos, vec![4 - 2, 5 - 2, 6 - 2]);

        let input = to_chars("#.?.??#????#?.");
        let candidates = find_and_replace(2, &input, 3);
        assert_eq!(
            candidates,
            vec![
                (7, to_chars("#...###????#?.")),
                (8, to_chars("#....###???#?.")),
                (9, to_chars("#.....###??#?.")),
            ]
        );

        let input = to_chars("#...###.???#?.");
        let candidates = find_and_replace(8, &input, 1);
        assert_eq!(
            candidates,
            vec![
                (9, to_chars("#...###.#??#?.")),
                (10, to_chars("#...###..#?#?.")),
                (12, to_chars("#...###....#?.")),
            ]
        );
    }

    #[test]
    fn test_find_real_2() {
        let input = to_chars("#.?.??#????#?.");
        let founds = find_all(input, vec![1, 3, 2])
            .into_iter()
            .map(|vc| vc.iter().collect())
            .collect::<HashSet<String>>();

        assert_eq!(
            founds,
            set![
                "#...###...##..".to_string(),
                "#....###..##..".to_string(),
                "#.....###.##..".to_string(),
                "#...###....##.".to_string(),
                "#....###...##.".to_string(),
                "#.....###..##.".to_string()
            ]
        );
    }

    #[test]
    fn test_find_positions() {
        assert_eq!(
            find_positions(&vec!['?', '?', '?', '.', '#', '#', '#'], 2),
            vec![0, 1]
        );
        assert_eq!(find_positions(&to_chars("?###????????"), 3), vec![1]);
    }

    #[test]
    fn test_load_line() {
        let (marks, nums) = load_line("???.### 1,1,3");
        assert_eq!(marks, vec!['?', '?', '?', '.', '#', '#', '#']);
        assert_eq!(nums, vec![1, 1, 3]);
    }

    fn to_chars(line: &str) -> Vec<char> {
        line.chars().collect::<Vec<char>>()
    }
}
