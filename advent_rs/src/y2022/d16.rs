use std::borrow::ToOwned;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeInclusive;

use colored::Colorize;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use regex::Regex;

use crate::{interpolate_color, read_file, set};

const FILE_NAME: &str = "data/2022/input16.txt";

#[derive(Debug, PartialEq)]
struct Valve {
    name: String,
    idx: usize,
    rate: u32,
    nexts: Vec<(usize, u32)>,
}

impl Valve {
    fn new(name: String, idx: usize, rate: u32, nexts: Vec<(usize, u32)>) -> Self {
        Self {
            name,
            idx,
            rate,
            nexts, // (next idx, edge weight)
        }
    }
}

fn load(input: &str) -> Vec<Valve> {
    let regex =
        Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let data = input
        .lines()
        .map(|line| {
            let cas = regex.captures(line).unwrap();
            let name = cas[1].to_owned();
            let rate = *&cas[2].parse::<u32>().unwrap();
            let nexts = cas[3].to_owned();
            (name, rate, nexts)
        })
        .collect::<Vec<(String, u32, String)>>();

    let names: HashMap<String, usize> = data
        .iter()
        .enumerate()
        .map(|(idx, (name, _, _))| (name.clone(), idx))
        .collect();

    data.into_iter()
        .map(|(name, rate, nexts)| {
            let nexts: Vec<(usize, u32)> = nexts
                .split(",")
                .map(|s| (*names.get(&s.trim().to_owned()).unwrap(), 1))
                .collect();
            let idx = *names.get(&name).unwrap();
            Valve {
                name,
                idx,
                rate,
                nexts,
            }
        })
        .collect()
}

fn to_dot(valves: &HashMap<usize, Valve>) -> String {
    let mut graph = "strict graph {\n".to_owned();
    let nodes = valves
        .iter()
        .map(|(idx, valve)| {
            let mut s = String::new();
            s.push_str(&format!(
                "    {0} [label=\"{0}[{1}] {2}\", fontsize=8]\n",
                valve.name, valve.idx, valve.rate
            ));

            for (next, cost) in valve.nexts.iter() {
                s.push_str(&format!(
                    "    {} -- {} [label={}, fontsize=8]\n",
                    valve.name,
                    valves.get(next).unwrap().name,
                    cost
                ));
            }
            s
        })
        .collect::<Vec<String>>()
        .join("\n");

    graph.push_str(&nodes);
    graph.push_str("\n    AA [shape=box]\n");
    graph.push_str("}");
    graph
}

fn find_next(valve: &Valve, next_id: usize) -> usize {
    valve
        .nexts
        .iter()
        .enumerate()
        .find(|(_, (i, c))| *i == next_id)
        .map(|(idx, _)| idx)
        .unwrap()
}

fn connect(valves: &mut HashMap<usize, Valve>, idx: usize) {
    let valve = valves.get(&idx).unwrap();
    let (left_idx, left_cost) = valve.nexts[0];
    let (right_idx, right_cost) = valve.nexts[1];
    let cost = left_cost + right_cost;

    valves.entry(left_idx).and_modify(|left| {
        left.nexts.remove(find_next(left, idx));
        left.nexts.push((right_idx, cost));
    });

    valves.entry(right_idx).and_modify(|right| {
        right.nexts.remove(find_next(right, idx));
        right.nexts.push((left_idx, cost));
    });

    valves.entry(idx).and_modify(|valve| {
        valve.nexts.clear();
    });
}

fn compact_valves(valves: &mut HashMap<usize, Valve>) {
    let start: String = "AA".to_owned();

    loop {
        let mut candidate = HashSet::new();
        let keys = valves.keys().map(|k| *k).collect::<Vec<usize>>();

        for idx in keys {
            let valve = valves.get(&idx).unwrap();

            if valve.name == start || valve.rate > 0 {
                continue;
            }

            // ???
            if valve.nexts.len() != 2 {
                continue;
            }

            connect(valves, idx);
            candidate.insert(idx);
        }

        if candidate.is_empty() {
            break;
        }

        for idx in candidate {
            valves.remove(&idx);
        }
    }
}

fn to_map(valves: Vec<Valve>) -> HashMap<usize, Valve> {
    valves.into_iter().map(|valve| (valve.idx, valve)).collect()
}

fn routes(valves: &HashMap<usize, Valve>) {
    let start = valves.iter().find(|(_, v)| v.name == "AA").unwrap().0;

    // remains minute, releasing pressure
    let mut routes: Vec<(Vec<usize>, HashSet<usize>, i32, u32)> =
        vec![(vec![*start], set![*start], 30, 0)];

    loop {
        println!("{:?}", routes.len());

        let mut add: Vec<(Vec<usize>, HashSet<usize>, i32, u32)> = vec![];
        let mut rem: Vec<usize> = vec![];

        for (idx, (route, opened, remains, pressures)) in routes.iter_mut().enumerate() {
            if *remains <= 0 {
                continue;
            }

            let from = route.last().unwrap();
            let valve = valves.get(from).unwrap();

            for (next, weight) in valve.nexts.iter() {
                let mut new_route = route.clone();
                let mut new_opened = opened.clone();
                new_route.push(*next);
                let mut new_remain = *remains - *weight as i32;
                let mut new_pressure = *pressures;

                if !new_opened.contains(next) {
                    new_opened.insert(*next);
                    new_remain -= 1;
                    if new_remain > 0 {
                        new_pressure += (new_remain as u32) * valves.get(next).unwrap().rate;
                    }
                }

                add.push((new_route, new_opened, new_remain, new_pressure));
            }
            rem.push(idx);
        }

        let r_count = rem.len();
        let a_count = add.len();

        if rem.is_empty() {
            break;
        }

        rem.sort_by(|a, b| b.cmp(a));

        for rm in rem {
            routes.remove(rm);
        }

        for nr in add {
            routes.push(nr)
        }

        let (_, _, r, p) = routes
            .iter()
            .max_by(|(_, _, _, p1), (_, _, _, p2)| p1.cmp(p2))
            .unwrap();

        let (_, _, r1, p1) = routes
            .iter()
            .min_by(|(_, _, _, p1), (_, _, _, p2)| p1.cmp(p2))
            .unwrap();

        println!(
            "remove: {}, add: {}, max pressure!!! {}, remaining: {}, min pressure {}, min remaining {}",
            r_count, a_count, p, r, p1, r1
        );
    }

    let (_, _, _, p) = routes
        .iter()
        .max_by(|(_, _, _, p1), (_, _, _, p2)| p1.cmp(p2))
        .unwrap();

    println!("VALUE!!! {}", p);
}

fn proc1(input: &str) -> String {
    let valves = load(input);
    let mut valves: HashMap<usize, Valve> = to_map(valves);
    compact_valves(&mut valves);
    routes(&valves);

    to_dot(&valves)
}

fn quiz1() -> String {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(input: &str) -> usize {
    todo!()
}

fn quiz2() -> usize {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    #[ignore]
    fn test_load() {
        assert_eq!(
            load(INPUT),
            vec![
                Valve::new("AA".to_owned(), 0, 0, vec![(3, 1), (8, 1), (1, 1)]),
                Valve::new("BB".to_owned(), 1, 13, vec![(2, 1), (0, 1)]),
                Valve::new("CC".to_owned(), 2, 2, vec![(3, 1), (1, 1)]),
                Valve::new("DD".to_owned(), 3, 20, vec![(2, 1), (0, 1), (4, 1)]),
                Valve::new("EE".to_owned(), 4, 3, vec![(5, 1), (3, 1)]),
                Valve::new("FF".to_owned(), 5, 0, vec![(4, 1), (6, 1)]),
                Valve::new("GG".to_owned(), 6, 0, vec![(5, 1), (7, 1)]),
                Valve::new("HH".to_owned(), 7, 22, vec![(6, 1)]),
                Valve::new("II".to_owned(), 8, 0, vec![(0, 1), (9, 1)]),
                Valve::new("JJ".to_owned(), 9, 21, vec![(8, 1)]),
            ]
        );
    }

    #[test]
    #[ignore]
    fn test_calculation() {
        assert_eq!(20 * 3 + 33 * 4 + 54 * 8 + 76 * 4 + 79 * 3 + 81 * 6, 1651);
    }

    #[test]
    #[ignore]
    fn test_to_dot() {
        let valves = load(INPUT);
        let mut valves = to_map(valves);
        compact_valves(&mut valves);
        assert_eq!(
            to_dot(&valves),
            "strict graph {
    AA [xlabel=0]
    AA -- { DD, II, BB }
    BB [xlabel=13]
    BB -- { CC, AA }
    CC [xlabel=2]
    CC -- { DD, BB }
    DD [xlabel=20]
    DD -- { CC, AA, EE }
    EE [xlabel=3]
    EE -- { FF, DD }
    FF [xlabel=0]
    FF -- { EE, GG }
    GG [xlabel=0]
    GG -- { FF, HH }
    HH [xlabel=22]
    HH -- { GG }
    II [xlabel=0]
    II -- { AA, JJ }
    JJ [xlabel=21]
    JJ -- { II }
    AA [shape=box]
}"
        )
    }

    #[test]
    #[ignore]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), "".to_owned());
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), "".to_owned()); // 2253
    }

    #[test]
    #[ignore]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 0);
    }

    #[test]
    #[ignore]
    fn test_quiz2() {
        assert_eq!(quiz2(), 0);
    }
}
