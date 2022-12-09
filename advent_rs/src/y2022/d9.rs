use std::collections::{HashMap, HashSet};

use crate::read_file;

const FILE_NAME: &str = "data/2022/input9.txt";

type Pos = (i32, i32);

fn load(input: &str) -> Vec<(Pos, i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, count) = line.split_once(" ").unwrap();
            let count = count.parse::<i32>().unwrap();
            (dir_pos(dir.chars().next().unwrap()), count)
        })
        .collect::<Vec<_>>()
}

fn dir_pos(dir: char) -> Pos {
    match dir {
        'U' => (0, 1),
        'R' => (1, 0),
        'L' => (-1, 0),
        'D' => (0, -1),
        _ => panic!(),
    }
}

fn follow_move(dif: Pos, mov: Pos) -> Pos {
    match dif {
        (0, 0) => (0, 0),
        (1, 0) => match mov {
            (1, 0) | (1, 1) | (1, -1) => mov,
            _ => (0, 0),
        },
        (1, 1) => match mov {
            (1, 0) | (0, 1) => (1, 1),
            (-1, 1) => (0, 1),
            (1, -1) => (1, 0),
            (1, 1) => (1, 1),
            _ => (0, 0),
        },
        (0, 1) => match mov {
            (0, 1) | (1, 1) | (-1, 1) => mov,
            _ => (0, 0),
        },
        (-1, 1) => match mov {
            (-1, 0) | (0, 1) => (-1, 1),
            (1, 1) => (0, 1),
            (-1, -1) => (-1, 0),
            (-1, 1) => (-1, 1),
            _ => (0, 0),
        },
        (-1, 0) => match mov {
            (-1, 0) | (-1, 1) | (-1, -1) => mov,
            _ => (0, 0),
        },
        (-1, -1) => match mov {
            (-1, 0) | (0, -1) => (-1, -1),
            (-1, 1) => (-1, 0),
            (1, -1) => (0, -1),
            (-1, -1) => (-1, -1),
            _ => (0, 0),
        },
        (0, -1) => match mov {
            (0, -1) | (-1, -1) | (1, -1) => mov,
            _ => (0, 0),
        },
        (1, -1) => match mov {
            (1, 0) | (0, -1) => (1, -1),
            (-1, -1) => (0, -1),
            (1, 1) => (1, 0),
            (1, -1) => (1, -1),
            _ => (0, 0),
        },
        _ => panic!(),
    }
}

fn trace(path: Vec<(Pos, i32)>, snake: &mut Vec<Pos>) -> usize {
    let length = snake.len();

    let mut ts = HashSet::new();
    ts.insert((0, 0));

    println!("{:?}", &snake);

    for (dir, count) in path {
        for n in 0..count {
            let (mut mx, mut my) = dir;
            for i in 1..snake.len() {
                let (dx, dy) = (snake[i - 1].0 - snake[i].0, snake[i - 1].1 - snake[i].1);

                snake[i - 1].0 += mx;
                snake[i - 1].1 += my;

                (mx, my) = follow_move((dx, dy), (mx, my));

                if mx == 0 && my == 0 {
                    break;
                } else if i == length - 1 {
                    snake[i].0 += mx;
                    snake[i].1 += my;
                    ts.insert((snake[i].0, snake[i].1));
                }
            }
            println!("{:?} {:?}, {}/{}", &snake, dir, n, count);
        }
    }

    ts.iter().count()
}

fn proc1(input: &str) -> usize {
    let path = load(input);
    let mut snake: Vec<(i32, i32)> = vec![(0, 0); 2];
    trace(path, &mut snake)
}

fn quiz1() -> usize {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(input: &str) -> usize {
    let path = load(input);
    let mut snake: Vec<(i32, i32)> = vec![(0, 0); 10];
    trace(path, &mut snake)
}

fn quiz2() -> usize {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 13);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 6243);
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 1);
        assert_eq!(proc2(INPUT2), 36);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 2630);
    }
}
