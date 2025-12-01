use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair};
use nom::{IResult, Parser};

const INPUT: &str = include_str!("../../data/2023/input5.txt");

fn quiz1() -> u64 {
    solve1(INPUT)
}

fn solve1(data: &str) -> u64 {
    let (_, almanac) = almanac_parser(data).unwrap();

    almanac
        .seeds
        .iter()
        .map(|seed| {
            almanac
                .maps_list
                .iter()
                .fold(*seed, |acc, maps| maps.corresponds(acc))
        })
        .min()
        .unwrap()
}

fn quiz2() -> u64 {
    solve2(INPUT)
}

fn solve2(data: &str) -> u64 {
    let (_, almanac) = almanac_parser(data).unwrap();

    let mut seeds = vec![];
    for i in 0..almanac.seeds.len() / 2 {
        seeds.append(
            &mut (almanac.seeds[i * 2]..almanac.seeds[i * 2] + almanac.seeds[i * 2 + 1])
                .collect::<Vec<u64>>(),
        );
    }

    let mut result = u64::MAX;

    for seed in seeds {
        let mapped = almanac
            .maps_list
            .iter()
            .fold(seed, |acc, maps| maps.corresponds(acc));

        if mapped < result {
            result = mapped;
        }
    }

    result
}

#[derive(Debug, Eq, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps_list: Vec<Maps>,
}

#[derive(Debug, Eq, PartialEq)]
struct Maps {
    src_name: String,
    dest_name: String,
    ranges: Vec<Range>,
    // range_map: HashMap<u64, u64>,
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

impl Maps {
    fn corresponds(&self, input: u64) -> u64 {
        self.ranges
            .iter()
            .find(|range| range.mapped(input))
            .map(|range| range.correspond(input))
            .unwrap_or_else(|| input)
    }
}

impl Range {
    fn mapped(&self, input: u64) -> bool {
        input >= self.src_start && input < self.src_start + self.length
    }

    fn correspond(&self, input: u64) -> u64 {
        self.dest_start + input - self.src_start
    }
}

fn seed_parser(line: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        pair(tag("seeds:"), space1),
        separated_list1(space1, nom::character::complete::u64),
    ).parse(line)
}

fn maps_head_parser(line: &str) -> IResult<&str, (String, String)> {
    map(
        ((alpha1, tag("-to-"), alpha1, space1, tag("map:"))),
        |(src, _, dest, _, _): (&str, _, &str, _, _)| (src.to_string(), dest.to_string()),
    ).parse(line)
}

fn maps_list_parser(data: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
    separated_list1(
        newline,
        map(
            ((
                nom::character::complete::u64,
                space1,
                nom::character::complete::u64,
                space1,
                nom::character::complete::u64,
            )),
            |(dest, _, src, _, len): (u64, _, u64, _, u64)| (dest, src, len),
        ),
    ).parse(data)
}

fn maps_parser(data: &str) -> IResult<&str, Maps> {
    map(
        separated_pair(maps_head_parser, newline, maps_list_parser),
        |((src_name, dest_name), ranges)| {
            let ranges = ranges
                .iter()
                .map(|&(dest_start, src_start, length)| Range {
                    dest_start,
                    src_start,
                    length,
                })
                .collect::<Vec<Range>>();

            // let range_map = build_map(&ranges);

            Maps {
                src_name,
                dest_name,
                ranges,
                // range_map,
            }
        },
    ).parse(data)
}

// fn build_map(ranges: &Vec<Range>) -> HashMap<u64, u64> {
//     let mut result = HashMap::new();
//     for range in ranges {
//         for i in 0..range.length {
//             result.insert(range.src_start + i, range.dest_start + i);
//         }
//     }
//     result
// }

fn almanac_parser(data: &str) -> IResult<&str, Almanac> {
    map(
        ((
            seed_parser,
            newline,
            newline,
            separated_list1(((newline, newline)), maps_parser),
        )),
        |(seeds, _, _, maps_list): (Vec<u64>, _, _, Vec<Maps>)| Almanac { seeds, maps_list },
    ).parse(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let r1 = Range {
            dest_start: 50,
            src_start: 98,
            length: 2,
        };
        let r2 = Range {
            dest_start: 52,
            src_start: 50,
            length: 48,
        };

        assert_eq!(r1.mapped(79), false);
        assert_eq!(r2.mapped(79), true);

        assert_eq!(r1.mapped(97), false);
        assert_eq!(r1.mapped(98), true);
        assert_eq!(r1.mapped(99), true);

        assert_eq!(r2.correspond(79), 81);

        assert_eq!(r1.correspond(98), 50);
        assert_eq!(r1.correspond(99), 51);
    }

    #[test]
    fn test_seed_parse() {
        let (_, seeds) = seed_parser("seeds: 79 14 55 13").unwrap();

        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_maps_head_parse() {
        let (line, (src, dest)) = maps_head_parser("seed-to-soil map:").unwrap();

        assert_eq!(line, "");
        assert_eq!((src, dest), ("seed".to_string(), "soil".to_string()));
    }

    #[test]
    fn test_maps_list_parse() {
        let (_, vs) = maps_list_parser(
            "50 98 2
52 50 48",
        )
        .unwrap();

        assert_eq!(vs, vec![(50, 98, 2), (52, 50, 48)]);
    }

    #[test]
    fn test_map_parse() {
        let (_, maps) = maps_parser(
            "seed-to-soil map:
50 98 2
52 50 48",
        )
        .unwrap();

        assert_eq!(
            maps,
            Maps {
                src_name: "seed".to_string(),
                dest_name: "soil".to_string(),
                ranges: vec![
                    Range {
                        dest_start: 50,
                        src_start: 98,
                        length: 2
                    },
                    Range {
                        dest_start: 52,
                        src_start: 50,
                        length: 48
                    },
                ],
                // range_map: HashMap::new(),
            }
        )
    }

    #[test]
    fn test_almanac_parse() {
        let (_, almanac) = almanac_parser(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15",
        )
        .unwrap();

        let expected = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps_list: vec![
                Maps {
                    src_name: "seed".to_string(),
                    dest_name: "soil".to_string(),
                    ranges: vec![
                        Range {
                            dest_start: 50,
                            src_start: 98,
                            length: 2,
                        },
                        Range {
                            dest_start: 52,
                            src_start: 50,
                            length: 48,
                        },
                    ],
                    // range_map: HashMap::new(),
                },
                Maps {
                    src_name: "soil".to_string(),
                    dest_name: "fertilizer".to_string(),
                    ranges: vec![
                        Range {
                            dest_start: 0,
                            src_start: 15,
                            length: 37,
                        },
                        Range {
                            dest_start: 37,
                            src_start: 52,
                            length: 2,
                        },
                        Range {
                            dest_start: 39,
                            src_start: 0,
                            length: 15,
                        },
                    ],
                    // range_map: HashMap::new(),
                },
            ],
        };

        assert_eq!(almanac, expected)
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 35);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 484023871);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE), 46);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 46294175);
    }

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
