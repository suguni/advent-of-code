use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

const QUIZ_INPUT: &str = include_str!("../../data/2025/input7.txt");

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
}

fn quiz2() -> usize {
    solve2(QUIZ_INPUT)
}

fn parse_data(input: &str) -> (usize, Vec<Vec<usize>>) {
    let mut lines = input.lines();

    let first = lines.next().unwrap();
    let (start, _) = first.chars().find_position(|c| *c == 'S').unwrap();

    let mut splitters = vec![];
    while let Some(line) = lines.next() {
        let ss = line
            .chars()
            .enumerate()
            .filter(|(i, c)| *c == '^')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        if !ss.is_empty() {
            splitters.push(ss);
        }
    }

    (start, splitters)
}

fn solve1(input: &str) -> usize {
    let (start, splitters) = parse_data(input);

    let mut beams = vec![start];
    let mut split_count = 0;

    for splitter in splitters {
        let (count, next_beams, _) = split(&beams, &splitter);
        beams = next_beams;
        split_count += count;
    }

    split_count
}

fn solve2(input: &str) -> usize {
    let (start, splitters) = parse_data(input);
    let mut beams = vec![start];
    let mut flow = HashMap::<usize, usize>::new();
    flow.insert(start, 1);

    for splitter in splitters {
        let (_, next_beams, path) = split(&beams, &splitter);

        flow = merge_path(flow, path);
        println!("{:?}", flow);
        beams = next_beams;
    }

    println!("{:?}", flow);
    flow.values().sum()
}

fn merge_path(flow: HashMap<usize, usize>, path: Vec<(usize, usize)>) -> HashMap<usize, usize> {
    // flow : 위치로 흐름 수 (from, count)
    // path : from -> to, from 은 flow 에 존재해야 한다.
    // to 로 가는 경로가 여러개 있을 수 있으므로 flow 에서 to 로 들어가는 모든 from 의 count 를 더하여 새로운 flow 를 만든다.

    let mut path_map = HashMap::new();
    for (from, to) in path {
        let v = path_map.entry(from).or_insert(Vec::new());
        v.push(to);
    }

    let mut new_flow = HashMap::<usize, usize>::new();

    for (from, count) in flow {
        let path = path_map.get(&from).unwrap();
        for to in path {
            match new_flow.entry(*to) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += count;
                }
                Entry::Vacant(entry) => {
                    entry.insert(count);
                }
            }
        }
    }

    new_flow
}

fn split(beams: &[usize], splitters: &[usize]) -> (usize, Vec<usize>, Vec<(usize, usize)>) {
    let mut split = vec![];
    let mut path = vec![];
    let mut split_count = 0;

    for b in beams {
        if splitters.contains(b) {
            split.push(*b - 1);
            split.push(*b + 1);
            split_count += 1;
            path.push((*b, *b - 1));
            path.push((*b, *b + 1));
        } else {
            split.push(*b);
            path.push((*b, *b));
        }
    }

    (split_count, split.into_iter().dedup().collect(), path)
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    use super::*;

    #[test]
    fn test_parse() {
        let (start, splitters) = parse_data(SAMPLE);
        assert_eq!(start, 7);
        assert_eq!(
            splitters,
            vec![
                vec![7],
                vec![6, 8],
                vec![5, 7, 9],
                vec![4, 6, 10],
                vec![3, 5, 9, 11],
                vec![2, 6, 12],
                vec![1, 3, 5, 7, 9, 13],
            ]
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            split(&vec![7], &vec![7]),
            (1, vec![6, 8], vec![(7, 6), (7, 8)])
        );
        assert_eq!(
            split(&vec![6, 8], &vec![6, 8]),
            (2, vec![5, 7, 9], vec![(6, 5), (6, 7), (8, 7), (8, 9)])
        );
        assert_eq!(
            split(&vec![4, 6, 8, 10], &vec![4, 6, 10]),
            (
                3,
                vec![3, 5, 7, 8, 9, 11],
                vec![(4, 3), (4, 5), (6, 5), (6, 7), (8, 8), (10, 9), (10, 11)]
            )
        );
        assert_eq!(
            split(
                &vec![1, 3, 4, 5, 7, 8, 10, 11, 13],
                &vec![1, 3, 5, 7, 9, 13],
            ),
            (
                5,
                vec![0, 2, 4, 6, 8, 10, 11, 12, 14],
                vec![
                    (1, 0),
                    (1, 2),
                    (3, 2),
                    (3, 4),
                    (4, 4),
                    (5, 4),
                    (5, 6),
                    (7, 6),
                    (7, 8),
                    (8, 8),
                    (10, 10),
                    (11, 11),
                    (13, 12),
                    (13, 14)
                ]
            )
        );
    }

    fn to_map(ts: Vec<(usize, usize)>) -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        for (k, v) in ts {
            map.insert(k, v);
        }
        map
    }

    #[test]
    fn test_merge_path() {
        assert_eq!(
            merge_path(to_map(vec![(7, 1)]), vec![(7, 6), (7, 8)]),
            to_map(vec![(6, 1), (8, 1)])
        );

        // assert_eq!(
        //     merge_path(
        //         vec![vec![7, 6], vec![7, 8]],
        //         vec![(6, 5), (6, 7), (8, 7), (8, 9)]
        //     ),
        //     vec![vec![7, 6, 5], vec![7, 6, 7], vec![7, 8, 7], vec![7, 8, 9]]
        // );
        //
        // assert_eq!(
        //     merge_path(vec![vec![7], vec![8]], vec![(7, 6), (7, 8), (8, 8)]),
        //     vec![vec![7, 6], vec![7, 8], vec![8, 8]]
        // );
        //
        // assert_eq!(
        //     merge_path(vec![vec![7, 7], vec![6, 7]], vec![(7, 6), (7, 8)]),
        //     vec![vec![7, 7, 6], vec![7, 7, 8], vec![6, 7, 6], vec![6, 7, 8]]
        // );
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 21);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 1550);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 40);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 9897897326778);
    }
}
