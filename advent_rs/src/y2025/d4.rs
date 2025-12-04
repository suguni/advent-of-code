use std::ops::RangeInclusive;

const QUIZ_INPUT: &str = include_str!("../../data/2025/input4.txt");

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
}

fn quiz2() -> usize {
    solve2(QUIZ_INPUT)
}

fn solve1(data: &str) -> usize {
    let wall = load(data);
    let mut count = 0;
    for r in 0..wall.rows {
        for c in 0..wall.cols {
            let s = wall.get(r, c);
            let adj = adjacency(&wall, r, c);
            if s == '@' {
                if adj < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve2(data: &str) -> usize {
    let mut wall = load(data);
    while !reduce(&mut wall) {}
    wall.removed_count()
}

struct Wall {
    data: Vec<char>,
    cols: usize,
    rows: usize,
}

impl Wall {
    fn get(&self, r: usize, c: usize) -> char {
        let p = r * self.cols + c;
        self.data[p]
    }

    fn mark_x(&mut self, pos: &Vec<(usize, usize)>) {
        for (r, c) in pos {
            self.data[r * self.rows + c] = 'X';
        }
    }

    fn removed_count(&self) -> usize {
        self.data.iter().filter(|&c| *c == 'X').count()
    }
}

fn load(input: &str) -> Wall {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let data = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    Wall { data, cols, rows }
}

fn adjacency(wall: &Wall, r: usize, c: usize) -> u32 {
    let mut result = 0;
    for r1 in range(r, wall.rows) {
        for c1 in range(c, wall.cols) {
            if !(r1 == r && c1 == c) {
                if wall.get(r1, c1) == '@' {
                    result += 1;
                }
            }
        }
    }
    result
}

fn range(x: usize, max: usize) -> RangeInclusive<usize> {
    if x == 0 {
        x..=x + 1
    } else if x == max - 1 {
        x - 1..=x
    } else {
        x - 1..=x + 1
    }
}

fn reduce(wall: &mut Wall) -> bool {
    let mut pos = vec![];

    for r in 0..wall.rows {
        for c in 0..wall.cols {
            let s = wall.get(r, c);
            let adj = adjacency(&wall, r, c);
            if s == '@' && adj < 4 {
                pos.push((r, c));
            }
        }
    }

    wall.mark_x(&pos);
    pos.is_empty()
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 13);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 1449);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 43);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 8746);
    }
}
