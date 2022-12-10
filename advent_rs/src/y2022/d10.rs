use nom::{
    branch::alt,
    bytes::complete::{self, tag},
    character::complete::newline,
    multi::separated_list1,
    IResult,
};

use crate::read_file;

const FILE_NAME: &str = "data/2022/input10.txt";

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycle(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::Noop))
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, x) = nom::character::complete::i32(input)?;
    Ok((input, Instruction::Addx(x)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, ins) = separated_list1(newline, alt((noop, addx)))(input)?;
    Ok((input, ins))
}

fn load(input: &str) -> Vec<Instruction> {
    if let Ok((_, ins)) = instructions(input) {
        return ins;
    }
    panic!()
}

struct Register {
    pc: usize,
    x: i32,
}

impl Register {
    fn tick(&mut self, pc: usize) {
        self.pc += pc;
    }

    fn addx(&mut self, x: i32) {
        self.x += x;
    }
}

fn proc1(input: &str) -> i32 {
    let instructions = load(input);
    let target_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut strength = vec![0; 6];
    let mut ti = 0;

    let mut reg = Register { pc: 0, x: 1 };

    for ins in instructions {
        let cycle = ins.cycle();
        if target_cycles[ti] <= reg.pc + cycle {
            strength[ti] = target_cycles[ti] as i32 * reg.x;
            ti += 1;

            if ti == 6 {
                break;
            }
        }

        reg.tick(cycle);

        match ins {
            Instruction::Noop => {}
            Instruction::Addx(x) => {
                reg.addx(x);
            }
        }
    }

    strength.iter().sum()
}

fn quiz1() -> i32 {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn in_sprite(cycle: usize, sprite: i32) -> bool {
    let c = cycle as i32;
    sprite - 1 <= c && c <= sprite + 1
}

fn proc2(input: &str) -> String {
    const ROW: usize = 40;
    const COL: usize = 6;

    let instructions = load(input);

    let mut crt = vec!['.'; 240];
    let mut sprite = 1;
    let mut ti = 0;
    let mut acc_ins_cycle = instructions[ti].cycle();

    for cycle in 0..crt.len() {
        if cycle >= acc_ins_cycle {
            let ins = &instructions[ti];
            if let Instruction::Addx(x) = ins {
                sprite += x;
            }
        }

        if in_sprite(cycle % ROW, sprite) {
            crt[cycle] = '#';
        }

        if cycle >= acc_ins_cycle {
            ti += 1;
            acc_ins_cycle += instructions[ti].cycle();
        }
    }

    to_screen(crt, ROW, COL)
}

fn to_screen(mut crt: Vec<char>, row: usize, col: usize) -> String {
    let mut ret = String::new();

    for _ in 0..col {
        let (line, sub_crt) = crt.split_at(row);
        let head = line.iter().collect::<String>();
        ret.push_str(&head);
        ret.push('\n');
        crt = sub_crt.to_vec();
    }

    ret.trim().to_owned()
}

fn quiz2() -> String {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::Instruction::*;
    use super::*;

    const INPUT0: &str = "noop
addx 3
addx -5";

    #[test]
    fn test_load() {
        assert_eq!(load(INPUT0), vec![Noop, Addx(3), Addx(-5)]);
    }

    const OUTPUT_CRT1: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), OUTPUT_CRT1);
    }

    #[test]
    fn test_quiz2() {
        let crt = quiz2();
        assert_eq!(
            crt,
            "###...##...##..####.#..#.#....#..#.####.
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#.#....###..##...#....####.###..
#..#.####.#....#....#.#..#....#..#.#....
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#..##..####.#..#.####.#..#.#...."
                .to_owned()
        );
    }

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 13140);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 16060);
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
