use std::collections::HashSet;
use itertools::Itertools;
use nom::Parser;

const QUIZ_INPUT: &str = include_str!("../../data/2024/input6.txt");

type Pos = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir { N, E, S, W }

impl Dir {
    fn turn(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
}

#[derive(Debug)]
struct LabMap {
    start: Pos,
    obstacles: Vec<Pos>,
    size: Pos,
}

impl LabMap {
    fn is_outside(&self, pos: Pos) -> bool {
        let (r, c) = pos;
        let (sr, sc) = self.size;
        r < 0 || c < 0 || r >= sr || c >= sc
    }

    fn is_obstacle(&self, pos: Pos) -> bool {
        self.obstacles.contains(&pos)
    }

    fn add_obstacle(&self, pos: Pos) -> LabMap {
        let mut obstacles = self.obstacles.clone();
        obstacles.push(pos);
        LabMap {
            start: self.start,
            obstacles,
            size: self.size,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

impl Guard {
    fn new(pos: Pos, dir: Dir) -> Self {
        Guard { pos, dir }
    }

    fn turn(&self) -> Guard {
        match self.dir {
            Dir::N => Guard::new(self.pos, Dir::E),
            Dir::E => Guard::new(self.pos, Dir::S),
            Dir::S => Guard::new(self.pos, Dir::W),
            Dir::W => Guard::new(self.pos, Dir::N),
        }
    }

    fn step(&self, step: i32) -> Guard {
        let (r, c) = self.pos;
        match self.dir {
            Dir::N => Guard::new((r - step, c), self.dir),
            Dir::E => Guard::new((r, c + step), self.dir),
            Dir::S => Guard::new((r + step, c), self.dir),
            Dir::W => Guard::new((r, c - step), self.dir),
        }
    }
}

fn parse_data(input: &str) -> LabMap {
    let mut start = (0, 0);
    let mut obstacles = vec![];

    let mut rows = 0;
    let mut cols = 0;

    input.lines().enumerate().for_each(|(row, line)| {
        let row = row as i32;
        line.chars().enumerate().for_each(|(col, ch)| {
            let col = col as i32;
            if ch == '^' {
                start = (row, col);
            } else if ch == '#' {
                obstacles.push((row, col));
            }
            cols = col;
        });
        rows = row;
    });

    LabMap {
        start,
        obstacles,
        size: (rows + 1, cols + 1),
    }
}

fn solve1(input: &str) -> usize {
    let lab_map = parse_data(input);
    let mut current = Guard::new(lab_map.start, Dir::N);

    let mut marked = vec!['.'; lab_map.size.0 as usize * lab_map.size.1 as usize];
    lab_map.obstacles.iter().for_each(|(r, c)| marked[*r as usize * lab_map.size.0 as usize + *c as usize] = '#');

    while let Some(next) = step_one(&current, &lab_map) {
        let (r, c) = next.pos;
        marked[r as usize * lab_map.size.0 as usize + c as usize] = 'X';
        current = next;
    }

    let (r, c) = lab_map.start;
    marked[r as usize * lab_map.size.0 as usize + c as usize] = '^';

    let map = marked.chunks(lab_map.size.0 as usize).map(|cs| cs.iter().join("")).join("\n");

    println!("{}", map);

    marked.iter().filter(|x| **x == 'X' || **x == '^').count()
}

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
}

enum Stop {
    OUTSIDE, VISITED,
}

fn step_one(current: &Guard, lab_map: &LabMap) -> Option<Guard> {
    let next = current.step(1);

    if lab_map.is_outside(next.pos) {
        None
    } else if lab_map.is_obstacle(next.pos) {
        Some(current.turn())
    } else {
        Some(next)
    }
}

fn step_more(current: &Guard, lab_map: &LabMap, visited: &Vec<Guard>) -> Result<Guard, Stop> {
    if let Some(next) = next_guard(current, &lab_map.obstacles) {
        if visited.contains(&next) {
            Err(Stop::VISITED)
        } else {
            Ok(next)
        }
    } else {
        Err(Stop::OUTSIDE)
    }
}

fn next_guard(current: &Guard, obstacles: &Vec<Pos>) -> Option<Guard> {
    match current.dir {
        Dir::N => obstacles.iter()
            .filter(|(r, c)| *c == current.pos.1 && *r < current.pos.0)
            .max_by(|(r1, _), (r2, _)| (*r1).cmp(r2))
            .map(|(r, c)| Guard::new((*r + 1, *c), current.dir.turn())),

        Dir::E => obstacles.iter()
            .filter(|(r, c)| *c > current.pos.1 && *r == current.pos.0)
            .min_by(|(_, c1), (_, c2)| (*c1).cmp(c2))
            .map(|(r, c)| Guard::new((*r, *c - 1), current.dir.turn())),

        Dir::S => obstacles.iter()
            .filter(|(r, c)| *c == current.pos.1 && *r > current.pos.0)
            .min_by(|(r1, _), (r2, _)| (*r1).cmp(r2))
            .map(|(r, c)| Guard::new((*r - 1, *c), current.dir.turn())),

        Dir::W => obstacles.iter()
            .filter(|(r, c)| *c < current.pos.1 && *r == current.pos.0)
            .max_by(|(_, c1), (_, c2)| (*c1).cmp(c2))
            .map(|(r, c)| Guard::new((*r, *c + 1), current.dir.turn())),
    }
}

fn has_loop(lab_map: &LabMap) -> bool {
    let mut current = Guard::new(lab_map.start, Dir::N);
    let mut visited = vec![];

    loop {
        match step_more(&current, &lab_map, &visited) {
            Ok(next) => {
                visited.push(next);
                current = next;
            }
            Err(Stop::OUTSIDE) => {
                return false;
            }
            Err(Stop::VISITED) => {
                return true;
            }
        }
    }
}

fn solve2(input: &str) -> usize {
    let lab_map = parse_data(input);
    let mut current = Guard::new(lab_map.start, Dir::N);

    let mut origin_path = HashSet::new();

    while let Some(next) = step_one(&current, &lab_map) {
        origin_path.insert(next.pos);
        current = next;
    }

    let mut count = 0;

    for pos in origin_path {
        let new_lab_map = lab_map.add_obstacle(pos);
        if has_loop(&new_lab_map) {
            count += 1;
        }
    }

    count
}

fn quiz2() -> usize {
    solve2(QUIZ_INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_parse_data() {
        let input = parse_data(SAMPLE);
        assert_eq!(input.start, (6, 4));
        assert_eq!(input.size, (10, 10));
        assert_eq!(
            input.obstacles,
            vec![
                (0, 4),
                (1, 9),
                (3, 2),
                (4, 7),
                (6, 1),
                (7, 8),
                (8, 0),
                (9, 6)
            ]
        );
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 41);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 4647);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 6);
    }

    #[test]
    fn test_step_size() {
        assert_eq!(
            next_guard(&Guard::new((0, 0), Dir::E), &vec![(0, 2), (0, 3)]),
            Some(Guard::new((0, 1), Dir::S)));

        assert_eq!(
            next_guard(&Guard::new((0, 4), Dir::W), &vec![(0, 2), (0, 5)]),
            Some(Guard::new((0, 3), Dir::N)));

        assert_eq!(
            next_guard(&Guard::new((0, 0), Dir::S), &vec![(2, 0), (3, 0)]),
            Some(Guard::new((1, 0), Dir::W)));

        assert_eq!(
            next_guard(&Guard::new((5, 0), Dir::N), &vec![(2, 0), (3, 0)]),
            Some(Guard::new((4, 0), Dir::E)));

        assert_eq!(
            next_guard(&Guard::new((5, 0), Dir::N), &vec![(0, 2), (0, 3)]),
            None);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 1723);
    }
}
