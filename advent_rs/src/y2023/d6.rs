use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::complete;
use nom::multi::separated_list1;
use nom::sequence::{pair, tuple};
use nom::IResult;
use std::iter::zip;

const INPUT: &str = "Time:        53     71     78     80
Distance:   275   1181   1215   1524";

fn solve1(data: &str) -> u64 {
    let (_, vs) = load1(data).unwrap();

    vs.iter()
        .map(|&(time, distance)| count_record(time, distance))
        .product()
}

fn count_record(time: u64, distance: u64) -> u64 {
    let (count, end) = if time % 2 == 0 {
        (1, time / 2 - 1)
    } else {
        (0, time / 2)
    };

    (1..=end)
        .rev()
        .take_while(|i| i * (time - i) > distance)
        .count() as u64
        * 2
        + count
}

fn load1(data: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (data, (_, _, times)) =
        tuple((tag("Time:"), space1, separated_list1(space1, complete::u64)))(data)?;

    let (data, _) = newline(data)?;

    let (data, (_, _, distances)) = tuple((
        tag("Distance:"),
        space1,
        separated_list1(space1, complete::u64),
    ))(data)?;

    let pairs: Vec<(u64, u64)> = zip(times.into_iter(), distances.into_iter()).collect();

    Ok((data, pairs))
}

fn load2(data: &str) -> IResult<&str, (u64, u64)> {
    let (data, (_, _, times)) =
        tuple((tag("Time:"), space1, separated_list1(space1, digit1)))(data)?;

    let (data, _) = newline(data)?;

    let (data, (_, _, distances)) =
        tuple((tag("Distance:"), space1, separated_list1(space1, digit1)))(data)?;

    let time = times.concat().parse::<u64>().unwrap();
    let distances = distances.concat().parse::<u64>().unwrap();

    Ok((data, (time, distances)))
}

fn solve2(data: &str) -> u64 {
    let (_, (time, distance)) = load2(data).unwrap();
    count_record(time, distance)
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    use super::*;

    #[test]
    fn test_load1() {
        let (_, vs) = load1(EXAMPLE).unwrap();
        assert_eq!(vs, vec![(7, 9), (15, 40), (30, 200)]);
    }

    #[test]
    fn test_count_record() {
        assert_eq!(count_record(7, 9), 4);
        assert_eq!(count_record(15, 40), 8);
        assert_eq!(count_record(30, 200), 9);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 288);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 449820);
    }

    #[test]
    fn test_load2() {
        let (_, vs) = load2(EXAMPLE).unwrap();
        assert_eq!(vs, (71530, 940200));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE), 71503);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT), 42250895);
    }
}
