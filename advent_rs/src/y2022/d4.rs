use std::convert::TryInto;

use crate::read_file;

fn process1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut xs = line.split(',').map(|rng| {
                let it = rng
                    .split('-')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                it[0]..=it[1]
            });
            (xs.next().unwrap(), xs.next().unwrap())
        })
        .filter(|(a, b)| {
            (a.start() <= b.start() && b.end() <= a.end())
                || (b.start() <= a.start() && a.end() <= b.end())
        })
        .count()
}

fn quiz1() -> usize {
    let input = read_file("data/2022/input4.txt");
    process1(&input)
}

fn process2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut xs = line.split(',').map(|rng| {
                let it = rng
                    .split('-')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                it[0]..=it[1]
            });
            (xs.next().unwrap(), xs.next().unwrap())
        })
        .filter(|(a, b)| {
            (a.start() <= b.start() && b.start() <= a.end())
                || (b.start() <= a.start() && a.start() <= b.end())
        })
        .count()
}

fn quiz2() -> usize {
    let input = read_file("data/2022/input4.txt");
    process2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_process1() {
        assert_eq!(process1(INPUT), 2);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 464);
    }

    #[test]
    fn test_process2() {
        assert_eq!(process2(INPUT), 4);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 770);
    }
}
