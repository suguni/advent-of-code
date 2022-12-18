use crate::read_file;

const ROCKS: [&str; 5] = [
    "####",
    ".#.
###
.#.",
    "..#
..#
###",
    "#
#
#
#",
    "##
##",
];

const SIZE: usize = 7;

const FILE_NAME: &str = "data/2022/input17.txt";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Dir {
    L,
    R,
    B,
}

impl Dir {
    fn new_from(c: char) -> Self {
        match c {
            '>' => Dir::R,
            '<' => Dir::L,
            'V' => Dir::B,
            _ => panic!(),
        }
    }

    fn next(&self, (x, y): Coord) -> Coord {
        match self {
            Dir::R => (x + 1, y),
            Dir::L => (x - 1, y),
            Dir::B => (x, y - 1),
        }
    }
}

type Bound = (i32, i32, i32, i32);
type Coord = (i32, i32);

struct Chamber {
    // rocks: Vec<Rock>,
    grid: Vec<bool>,
    rocks_bound: Bound,
}

fn sep_bound((l1, t1, r1, b1): Bound, (l2, t2, r2, b2): Bound) -> bool {
    l1 > r2 || r1 < l2 || t1 < b2 || b1 > t2
}

impl Chamber {
    fn new() -> Self {
        Self {
            // rocks: vec![],
            grid: vec![],
            rocks_bound: (0, 0, 0, 0),
        }
    }

    fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    fn collision_test(&self, rock: &Rock, (next_x, next_y): Coord) -> bool {
        let rl = next_x;
        let rt = next_y + rock.height as i32;
        let rr = next_x + rock.width as i32;
        let rb = next_y;

        if rl < 0 || rr as usize > SIZE || rb < 0 {
            return true;
        }

        if self.grid.is_empty() || sep_bound((rl, rt, rr, rb), self.rocks_bound) {
            return false;
        }

        let (cl, ct, cr, cb) = self.rocks_bound;

        for y in rb..rt.min(ct) {
            for x in rl.max(cl)..rr.min(cr) {
                let rock_pos = (y - rb) as usize * rock.width + (x - rl) as usize;
                let chamber_pos = (y * SIZE as i32 + x) as usize;
                if self.grid[chamber_pos] && rock.grid[rock_pos] {
                    return true;
                }
            }
        }

        false
    }

    fn move_rock(&self, rock: &Rock, (x, y): Coord, dir: Dir) -> Coord {
        let next_pos = dir.next((x, y));
        if self.collision_test(rock, next_pos) {
            return (x, y);
        }
        next_pos
    }

    fn fall_rock(&self, rock: &Rock, (x, y): Coord) -> Coord {
        let next_pos = Dir::B.next((x, y));
        if self.collision_test(rock, next_pos) {
            return (x, y);
        }
        next_pos
    }

    fn add_rock(&mut self, rock: Rock, (rock_x, rock_y): Coord) {
        let rl = rock_x;
        let rt = rock_y + rock.height as i32;
        let rr = rock_x + rock.width as i32;
        let rb = rock_y;

        if self.is_empty() {
            self.grid.append(&mut vec![false; SIZE * rock.height]);
            self.rocks_bound = (rl, rt, rr, rb);
        } else {
            let (_, bound_top, _, _) = self.rocks_bound;
            let diff = rt - bound_top;
            if diff > 0 {
                self.grid.append(&mut vec![false; SIZE * diff as usize]);
            }
            let (cl, ct, cr, cb) = self.rocks_bound;
            self.rocks_bound = (rl.min(cl), rt.max(ct), rr.max(cr), rb.min(cb));
        }

        for y in rb..rt {
            for x in rl..rr {
                let rock_pos = (y - rb) * rock.width as i32 + (x - rl);
                let chamber_pos = y * SIZE as i32 + x;
                if !self.grid[chamber_pos as usize] {
                    self.grid[chamber_pos as usize] = rock.grid[rock_pos as usize];
                }
            }
        }
    }
}

#[derive(Clone)]
struct Rock {
    grid: Vec<bool>,
    width: usize,
    height: usize,
}

impl Rock {
    fn new(pattern: &str) -> Self {
        let width = pattern.lines().next().unwrap().chars().count();
        let height = pattern.lines().count();
        let grid = pattern
            .lines()
            .rev()
            .flat_map(|line| line.chars().map(|c| c == '#'))
            .collect();
        Rock {
            grid,
            width,
            height,
        }
    }
}

struct Simulator {
    chamber: Chamber,

    rocks: Vec<Rock>,
    falling_rock: Option<Rock>,
    falling_rock_pos: Coord,
    next_rock: usize,

    move_patterns: Vec<Dir>,
    next_move: usize,
}

impl Simulator {
    fn new(pattern: &str, rocks: Vec<Rock>) -> Simulator {
        let move_patterns = pattern.chars().map(|c| Dir::new_from(c)).collect();

        let chamber = Chamber::new();

        Simulator {
            chamber,
            rocks,

            falling_rock: None,
            falling_rock_pos: (0, 0),
            next_rock: 0,

            move_patterns,
            next_move: 0,
        }
    }

    fn play(&mut self, count: usize) -> usize {
        let mut fell_count = 0;
        loop {
            if self.tick() {
                fell_count += 1;
                if fell_count % 10_000_000 == 0 {
                    println!("fell count: {fell_count}");
                }
                if fell_count >= count {
                    break;
                }
            }
        }
        self.chamber.rocks_bound.1 as usize
    }

    fn draw(&self, msg: &str) {
        println!("Rock ... {}", msg);
        let mut canvas = self
            .chamber
            .grid
            .iter()
            .map(|c| if *c { '#' } else { '.' })
            .collect::<Vec<char>>();

        if let Some(rock) = self.falling_rock.as_ref() {
            let (_, ct, _, _) = self.chamber.rocks_bound;
            let (rl, rb) = self.falling_rock_pos;
            let rock_top = rb + rock.height as i32;

            if rock_top > ct {
                canvas.append(&mut vec!['.'; (rock_top - ct) as usize * SIZE]);
            }

            for y in rb..rock_top {
                for x in rl..(rl + rock.width as i32) {
                    let rock_pos = (y - rb) * rock.width as i32 + (x - rl);
                    let chamber_pos = y * SIZE as i32 + x;

                    if rock.grid[rock_pos as usize] {
                        canvas[chamber_pos as usize] = '@';
                    }
                }
            }
        }

        // 42, 35
        let height = canvas.len() / 7;

        let mut paper = String::new();
        for h in (0..height).rev() {
            paper.push('|');
            paper.push_str(&canvas[7 * h..7 * (h + 1)].iter().collect::<String>());
            paper.push('|');
            paper.push('\n');
        }
        paper.push_str("+-------+");
        println!("{paper}");
        println!();
    }

    fn next_dir(&mut self) -> Dir {
        let dir = &self.move_patterns[self.next_move];
        self.next_move += 1;
        self.next_move %= self.move_patterns.len();
        *dir
    }

    fn tick(&mut self) -> bool {
        if self.falling_rock.is_none() {
            self.spawn_rock();
            // self.draw("spawned");
        }

        let dir = self.next_dir();
        let (moved, dir) = self.move_rock(dir);

        let msg = if moved {
            format!("moved to {:?}", dir)
        } else {
            format!("stay {:?}", dir)
        };
        // self.draw(&msg);

        let (moved, dir) = self.move_rock(Dir::B);
        // self.draw("fell");

        if !moved {
            if let Some(rock) = self.falling_rock.take() {
                self.chamber.add_rock(rock, self.falling_rock_pos);
                return true;
            }
        }
        false
    }

    fn move_rock(&mut self, dir: Dir) -> (bool, Dir) {
        if let Some(rock) = self.falling_rock.as_ref() {
            let new_pos = self.chamber.move_rock(rock, self.falling_rock_pos, dir);
            if self.falling_rock_pos != new_pos {
                self.falling_rock_pos = new_pos;
                (true, dir)
            } else {
                (false, dir)
            }
        } else {
            panic!()
        }
    }

    fn spawn_rock(&mut self) {
        let rock = self.rocks[self.next_rock].clone();
        self.falling_rock.replace(rock);
        self.falling_rock_pos = (
            2,
            if self.chamber.is_empty() {
                3
            } else {
                self.chamber.rocks_bound.1 + 3
            },
        );
        self.next_rock += 1;
        self.next_rock %= self.rocks.len();
    }
}

fn quiz1() -> usize {
    let input = read_file(FILE_NAME);
    let mut simulator = Simulator::new(&input, rocks());
    simulator.play(2022)
}

fn rocks() -> Vec<Rock> {
    ROCKS.map(|pattern| Rock::new(pattern)).to_vec()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_proc1() {
        let mut simulator = Simulator::new(INPUT, rocks());
        assert_eq!(simulator.play(2022), 3068);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 3067);
    }

    #[test]
    #[ignore]
    fn test_proc2() {
        let mut simulator = Simulator::new(INPUT, rocks());
        assert_eq!(simulator.play(1000000000000), 1514285714288);
    }
}
