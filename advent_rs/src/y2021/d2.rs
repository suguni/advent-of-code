use crate::read_file;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct Pos {
    pub h: i32,
    pub d: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_command(line: &str) -> Command {
    let cas = Regex::new(r"^(\w+)\s(\d+)$")
        .unwrap()
        .captures(&line)
        .unwrap();

    let op = String::from(&cas[1]);
    let x = String::from(&cas[2]).parse::<i32>().unwrap();

    match op.as_str() {
        "forward" => Command::Forward(x),
        "down" => Command::Down(x),
        "up" => Command::Up(x),
        _ => panic!(),
    }
}

fn load_data(text: &str) -> Vec<Command> {
    text.lines().map(|line| parse_command(line)).collect()
}

fn calc_pos(commands: Vec<Command>) -> Pos {
    let mut pos = Pos { h: 0, d: 0 };
    for command in commands {
        match command {
            Command::Forward(x) => {
                pos.h += x;
            }
            Command::Up(x) => {
                pos.d -= x;
            }
            Command::Down(x) => {
                pos.d += x;
            }
        }
    }
    pos
}

fn calc_pos2(commands: Vec<Command>) -> Pos {
    let mut pos = Pos { h: 0, d: 0 };
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(x) => {
                pos.h += x;
                pos.d += aim * x;
            }
            Command::Up(x) => {
                aim -= x;
            }
            Command::Down(x) => {
                aim += x;
            }
        }
    }
    pos
}
pub fn quiz1() -> i32 {
    let text = read_file("data/2021/input2.txt");
    let cmds = load_data(text.as_str());
    let pos = calc_pos(cmds);
    pos.h * pos.d
}

pub fn quiz2() -> i32 {
    let text = read_file("data/2021/input2.txt");
    let cmds = load_data(text.as_str());
    let pos = calc_pos2(cmds);
    pos.h * pos.d
}

#[cfg(test)]
mod tests {

    use super::Command::*;
    use super::*;

    #[test]
    fn test_parse_command() {
        let f1 = "forward 5";
        assert_eq!(parse_command(f1), Command::Forward(5));
    }

    #[test]
    fn test_load_data() {
        let text = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
"
        .trim();
        assert_eq!(
            load_data(text),
            vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]
        );
    }

    #[test]
    fn test_calc() {
        let cmds = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let pos = calc_pos(cmds);
        assert_eq!(pos, Pos { h: 15, d: 10 })
    }

    // #[test]
    // fn run_quiz() {
    //     assert_eq!(quiz2(), 1);
    // }
}
