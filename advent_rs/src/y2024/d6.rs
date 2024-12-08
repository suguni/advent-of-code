use itertools::Itertools;
use nom::Parser;

const QUIZ_INPUT: &str = include_str!("../../data/2024/input6.txt");


type Pos = (i32, i32);

#[derive(Clone, Copy, Debug)]
enum Dir { N, E, S, W }

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
}

#[derive(Debug)]
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

fn run(current: &Guard, lab_map: &LabMap) -> Option<Guard> {
    let next = current.step(1);

    if lab_map.is_outside(next.pos) {
        None
    } else if lab_map.is_obstacle(next.pos) {
        Some(current.turn())
    } else {
        Some(next)
    }
}

fn solve1(input: &str) -> usize {
    let lab_map = parse_data(input);
    let mut current = Guard::new(lab_map.start, Dir::N);

    let mut marked = vec!['.'; lab_map.size.0 as usize * lab_map.size.1 as usize];
    lab_map.obstacles.iter().for_each(|(r, c)| marked[*r as usize * lab_map.size.0 as usize + *c as usize] = '#');

    let (r, c) = lab_map.start;
    marked[r as usize * lab_map.size.0 as usize + c as usize] = 'O';


    while let Some(g) = run(&current, &lab_map) {
        let (r, c) = g.pos;
        marked[r as usize * lab_map.size.0 as usize + c as usize] = 'X';
        current = g;
    }

    let map = marked.chunks(lab_map.size.0 as usize).map(|cs| cs.iter().join("")).join("\n");

    println!("{}", map);

    marked.iter().filter(|x| **x == 'X' || **x == 'O').count()
}

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
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
}
