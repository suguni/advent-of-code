use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, PResult, Parser};
use num::range_step;

const QUIZ_INPUT: &str = include_str!("../../data/2025/input2.txt");

fn parse_range(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(
        nom::character::complete::u64,
        char('-'),
        nom::character::complete::u64,
    )
    .parse(input)
}
fn parse_data(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(tag(","), parse_range).parse(input)
}

fn quiz1() -> u64 {
    solve1(QUIZ_INPUT)
}

fn solve1(data: &str) -> u64 {
    let mut sum = 0_u64;
    let (_, vs) = parse_data(data).unwrap();
    for (s, e) in vs {
        for v in s..=e {
            let s = format!("{v}");
            if is_invalid1(&s) {
                sum += v;
            }
        }
    }
    sum
}

fn is_invalid1(num: &str) -> bool {
    let len = num.len() / 2;
    &num[0..len] == &num[len..]
}

fn is_repeated(s: &str, l: usize) -> bool {
    for i in 1..s.len() / l {
        let p = l * i;
        let q = p + l;
        if &s[0..l] != &s[p..q] {
            return false;
        }
    }
    true
}

fn is_repeated_seq(num: &str) -> bool {
    for i in 0..num.len() / 2 {
        if num.len() % (i + 1) != 0 {
            continue;
        }
        if is_repeated(num, i + 1) {
            return true;
        }
    }
    false
}

fn invalid_nums(s: u64, e: u64) -> Vec<u64> {
    let mut result = vec![];
    for v in s..=e {
        let s = format!("{v}");
        if is_repeated_seq(&s) {
            result.push(v);
        }
    }
    result
}

fn solve2(data: &str) -> u64 {
    let mut sum = 0_u64;
    let (_, vs) = parse_data(data).unwrap();

    for (s, e) in vs {
        sum += invalid_nums(s, e).iter().sum::<u64>();
    }
    sum
}

fn quiz2() -> u64 {
    solve2(QUIZ_INPUT)
}


#[cfg(test)]
mod tests {
    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_is_invalid1() {
        assert_eq!(is_invalid1("11"), true);
        assert_eq!(is_invalid1("12"), false);
        assert_eq!(is_invalid1("22"), true);
        assert_eq!(is_invalid1("1010"), true);
        assert_eq!(is_invalid1("1188511885"), true);
        assert_eq!(is_invalid1("1188511886"), false);
        assert_eq!(is_invalid1("110"), false);
    }

    #[test]
    fn test_repeated() {
        assert_eq!(is_repeated("11", 1), true);
        assert_eq!(is_repeated("111", 1), true);
        assert_eq!(is_repeated("112", 1), false);
        assert_eq!(is_repeated("2121212121", 2), true);
        assert_eq!(is_repeated("2121212121", 1), false);
        assert_eq!(is_repeated("38593859", 1), false);
        assert_eq!(is_repeated("38593859", 2), false);
        assert_eq!(is_repeated("38593859", 4), true);
        assert_eq!(is_repeated("1188511886", 5), false);
        assert_eq!(is_repeated("2121212118", 1), false);
        assert_eq!(is_repeated("2121212118", 2), false);
        assert_eq!(is_repeated("2121212118", 5), false);
    }

    #[test]
    fn test_is_invalid2() {
        assert_eq!(is_repeated_seq("11"), true);
        assert_eq!(is_repeated_seq("12"), false);
        assert_eq!(is_repeated_seq("22"), true);
        assert_eq!(is_repeated_seq("1010"), true);
        assert_eq!(is_repeated_seq("1188511885"), true);
        assert_eq!(is_repeated_seq("1188511886"), false);
        assert_eq!(is_repeated_seq("110"), false);
        assert_eq!(is_repeated_seq("2121212118"), false);
        assert_eq!(is_repeated_seq("2121212121"), true);
        assert_eq!(is_repeated_seq("38593859"), true);
    }

    #[test]
    fn test_invalid_nums() {
        assert_eq!(invalid_nums(11, 22), vec![11, 22]);
        assert_eq!(invalid_nums(95, 115), vec![99, 111]);
        assert_eq!(invalid_nums(998, 1012), vec![999, 1010]);
        assert_eq!(invalid_nums(1188511880, 1188511890), vec![1188511885]);
        assert_eq!(invalid_nums(222220, 222224), vec![222222]);
        assert_eq!(invalid_nums(1698522, 1698528), vec![]);
        assert_eq!(invalid_nums(446443, 446449), vec![446446]);
        assert_eq!(invalid_nums(38593856, 38593862), vec![38593859]);
        assert_eq!(invalid_nums(565653, 565659), vec![565656]);
        assert_eq!(invalid_nums(824824821, 824824827), vec![824824824]);
        assert_eq!(invalid_nums(2121212118, 2121212124), vec![2121212121]);
    }

    #[test]
    fn test_nom() {
        let (input, d) = digit1::<_, (_, ErrorKind)>("11-22").unwrap();
        assert_eq!(d, "11");
        assert_eq!(input, "-22");

        let (input, d) = char::<_, (_, ErrorKind)>('-').parse(input).unwrap();
        assert_eq!(d, '-');
        assert_eq!(input, "22");

        let (input, d) = digit1::<_, (_, ErrorKind)>(input).unwrap();
        assert_eq!(d, "22");
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_range() {
        let (_, range) = parse_range("11-22").unwrap();
        assert_eq!(range, (11, 22));
    }

    #[test]
    fn test_parse_data() {
        let (_, vs) = parse_data("11-22,95-115").unwrap();
        assert_eq!(vs, vec![(11, 22), (95, 115)]);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 1227775554);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 15873079081);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 4174379265);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 22617871034);
    }
}
