use std::collections::{HashMap, HashSet};

use nom::branch::alt;
use nom::character::complete::{self, newline, space1};
use nom::multi::{count, separated_list0, separated_list1};
use nom::sequence::separated_pair;
use nom::Parser;
use nom::{bytes::complete::tag, sequence::preceded, IResult};

use crate::read_file;

const FILE_NAME: &str = "data/2022/input11.txt";

#[derive(Debug, PartialEq)]
enum Op {
    Plus,
    Multiply,
}

impl Op {
    fn eval(&self, v: u32, rh: u32) -> u64 {
        match self {
            Op::Plus => v as u64 + rh as u64,
            Op::Multiply => v as u64 * rh as u64,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Rh {
    Num(u32),
    Old,
}

fn monkey(input: &str) -> IResult<&str, u32> {
    let (input, idx) = preceded(tag("Monkey "), complete::u32).parse(input)?;
    let (input, _) = tag(":")(input)?;
    Ok((input, idx))
}

fn items(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = space1(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = separated_list0(tag(", "), complete::u32).parse(input)?;
    Ok((input, items))
}

fn operation(input: &str) -> IResult<&str, (Op, Rh)> {
    let (input, _) = space1(input)?;
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, (op, n)) = separated_pair(
        alt((tag("+").map(|_| Op::Plus), tag("*").map(|_| Op::Multiply))),
        space1,
        alt((
            complete::u32.map(|n| Rh::Num(n)),
            tag("old").map(|_| Rh::Old),
        )),
    ).parse(input)?;
    Ok((input, (op, n)))
}

fn divisible(input: &str) -> IResult<&str, u32> {
    let (input, _) = space1(input)?;
    let (input, n) = preceded(tag("Test: divisible by "), complete::u32).parse(input)?;
    Ok((input, n))
}

fn throw(input: &str) -> IResult<&str, u32> {
    let (input, _) = space1(input)?;
    let (input, n) = preceded(
        alt((
            tag("If true: throw to monkey "),
            tag("If false: throw to monkey "),
        )),
        complete::u32,
    ).parse(input)?;
    Ok((input, n))
}

#[derive(Debug, PartialEq)]
struct Monkey {
    idx: u32,
    items: Vec<u32>,
    op: Op,
    op_val: Rh,
    divisor: u32,
    case_t: u32,
    case_f: u32,
}
impl Monkey {
    fn new(
        idx: u32,
        items: Vec<u32>,
        op: Op,
        op_val: Rh,
        divisor: u32,
        case_t: u32,
        case_f: u32,
    ) -> Self {
        Self {
            idx,
            items,
            op,
            op_val,
            divisor,
            case_t,
            case_f,
        }
    }
}

fn block(input: &str) -> IResult<&str, Monkey> {
    let (input, idx) = monkey(input)?;
    let (input, _) = newline(input)?;
    let (input, items) = items(input)?;
    let (input, _) = newline(input)?;
    let (input, (op, op_val)) = operation(input)?;
    let (input, _) = newline(input)?;
    let (input, divisor) = divisible(input)?;
    let (input, _) = newline(input)?;
    let (input, case_t) = throw(input)?;
    let (input, _) = newline(input)?;
    let (input, case_f) = throw(input)?;
    Ok((
        input,
        Monkey::new(idx, items, op, op_val, divisor, case_t, case_f),
    ))
}

fn load(input: &str) -> Vec<Monkey> {
    let (_, monkeys) = separated_list1(count(newline, 2), block).parse(input).unwrap();
    monkeys
}

fn proc1(input: &str) -> usize {
    let mut monkeys = load(input);
    let len = monkeys.len();
    let mut inspect = vec![0; len];

    for round in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut items = &mut monkey.items;
            let mut p: HashMap<u32, Vec<u32>> = HashMap::new();
            items.reverse();
            while let Some(w) = items.pop() {
                inspect[i] += 1;

                let op_val = match monkey.op_val {
                    Rh::Num(n) => n,
                    Rh::Old => w,
                };

                let worry = (monkey.op.eval(w, op_val) / 3) as u32;
                let i = if worry % monkey.divisor == 0 {
                    monkey.case_t
                } else {
                    monkey.case_f
                };

                let vs = p.entry(i).or_default();
                vs.push(worry);
            }

            for (k, mut v) in p {
                monkeys[k as usize].items.append(&mut v);
            }
        }
    }

    inspect.sort();
    println!("{inspect:?}");
    inspect[len - 1] * inspect[len - 2]
}

fn quiz1() -> usize {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(input: &str) -> usize {
    let mut monkeys = load(input);
    let len = monkeys.len();
    let mut inspect = vec![0; len];

    // 흠................
    let dv: u64 = monkeys.iter().map(|monkey| monkey.divisor).product::<u32>() as u64;

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut items = &mut monkey.items;
            let mut p: HashMap<u32, Vec<u32>> = HashMap::new();
            items.reverse();
            while let Some(w) = items.pop() {
                inspect[i] += 1;

                let op_val = match monkey.op_val {
                    Rh::Num(n) => n,
                    Rh::Old => w,
                };

                let worry = (monkey.op.eval(w, op_val) % dv) as u32;
                let i = if worry % monkey.divisor == 0 {
                    monkey.case_t
                } else {
                    monkey.case_f
                };

                let vs = p.entry(i).or_default();
                vs.push(worry);
            }

            for (k, mut v) in p {
                monkeys[k as usize].items.append(&mut v);
            }
        }

        let r = round + 1;
        if r >= 1000 && r % 1000 == 0 {
            println!("{r}: {inspect:?}");
        }
    }

    inspect.sort();
    inspect[len - 1] * inspect[len - 2]
}

fn quiz2() -> usize {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_load() {
        assert_eq!(
            load(INPUT),
            vec![
                Monkey::new(0, vec![79, 98], Op::Multiply, Rh::Num(19), 23, 2, 3),
                Monkey::new(1, vec![54, 65, 75, 74], Op::Plus, Rh::Num(6), 19, 2, 0),
                Monkey::new(2, vec![79, 60, 97], Op::Multiply, Rh::Old, 13, 1, 3),
                Monkey::new(3, vec![74], Op::Plus, Rh::Num(3), 17, 0, 1),
            ]
        )
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 2713310158);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 35270398814);
    }

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 10605);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 316888);
    }
}
