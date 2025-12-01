use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;
use nom::{IResult, Parser};

const INPUT: &str = include_str!("../../data/2023/input9.txt");

fn build_pyramid(ns: Vec<i32>) -> Vec<Vec<i32>> {
    let mut pyramid = vec![ns];

    let mut layer = 0;

    loop {
        let len = pyramid[layer].len();
        let mut next = vec![];
        let prev = &pyramid[layer];
        for i in 0..(len - 1) {
            next.push(prev[i + 1] - prev[i]);
        }

        let is_last = next.iter().all(|v| *v == 0);

        if (is_last) {
            next.push(0);
        }

        pyramid.push(next);

        if is_last {
            break;
        }

        layer += 1;
    }

    pyramid
}

fn guess1(ns: Vec<i32>) -> i32 {
    let mut pyramid = build_pyramid(ns);

    for i in (0..(pyramid.len() - 1)).rev() {
        let diff = pyramid[i + 1][pyramid[i + 1].len() - 1];
        let mut current = &mut pyramid[i];
        current.push(current[current.len() - 1] + diff);
    }

    *pyramid[0].last().unwrap()
}

fn guess2(ns: Vec<i32>) -> i32 {
    let mut pyramid = build_pyramid(ns);

    for i in (0..(pyramid.len() - 1)).rev() {
        let diff = pyramid[i + 1][0];
        let mut current = &mut pyramid[i];
        current.insert(0, current[0] - diff);
    }

    pyramid[0][0]
}

fn load(data: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(
        newline,
        separated_list1(space1, nom::character::complete::i32),
    ).parse(data)
}

fn solve1(data: &str) -> i32 {
    let (_, nss) = load(data).unwrap();

    nss.into_iter().map(|vs| guess1(vs)).sum()
}

fn solve2(data: &str) -> i32 {
    let (_, nss) = load(data).unwrap();

    nss.into_iter().map(|vs| guess2(vs)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn guess1_test() {
        assert_eq!(guess1(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(guess1(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(guess1(vec![10, 13, 16, 21, 30, 45]), 68);

        assert_eq!(
            guess1(vec![
                1, 2, 5, 13, 33, 89, 245, 643, 1565, 3535, 7495, 15128, 29479, 56181, 105913,
                199391, 377649, 723582, 1407901, 2788593, 5627669
            ]),
            11562151
        );

        assert_eq!(
            guess1(vec![
                14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6
            ]),
            -7
        );
    }

    #[test]
    fn guess2_test() {
        assert_eq!(guess2(vec![10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn solve1_test() {
        assert_eq!(solve1(EXAMPLE), 114);
    }

    #[test]
    fn quiz1_test() {
        assert_eq!(solve1(INPUT), 1904165718);
    }

    #[test]
    fn quiz2_test() {
        assert_eq!(solve2(INPUT), 964);
    }
}
