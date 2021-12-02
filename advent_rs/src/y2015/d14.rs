use regex::Regex;

fn parse_line(line: &str) -> (i32, i32, i32) {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let caps = re.captures(line).unwrap();

    (
        caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
    )
}

fn calc_dist((speed, fly, rest): (i32, i32, i32), time: i32) -> i32 {
    let dist = (time / (fly + rest)) * speed * fly;
    let r = time % (fly + rest);
    let x = r - fly;
    dist + speed * (if x < 0 { r } else { fly })
}

fn leaders(rules: &Vec<(i32, i32, i32)>, time: i32) -> Vec<usize> {
    let (_, max_indices): (i32, Vec<usize>) = rules
        .iter()
        .map(|rule| calc_dist(*rule, time))
        .enumerate()
        .fold((i32::MIN, Vec::new()), |(max, mut acc), (i, dist)| {
            if dist == max {
                acc.push(i);
                (max, acc)
            } else if dist > max {
                (dist, vec![i])
            } else {
                (max, acc)
            }
        });

    max_indices
}

fn calc_points(rules: &Vec<(i32, i32, i32)>, time: i32) -> Vec<i32> {
    let mut points = vec![0; rules.len()];
    for t in 1..=time {
        let leaders = leaders(rules, t);
        for leader in leaders {
            points[leader] += 1;
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const DURATION: i32 = 2503;

    #[test]
    fn quiz2() {
        let rules: Vec<(i32, i32, i32)> = read_file("../data/2015/input14.txt")
            .lines()
            .map(|line| parse_line(line))
            .collect();

        let points = calc_points(&rules, DURATION);

        assert_eq!(points.iter().max().unwrap(), &1059);
    }

    #[test]
    fn test_leader() {
        let rules = vec![(10, 10, 10), (10, 20, 10), (10, 30, 10)];
        assert_eq!(leaders(&rules, 1), vec![0, 1, 2]);
        assert_eq!(leaders(&rules, 11), vec![1, 2]);
        assert_eq!(leaders(&rules, 21), vec![2]);
    }

    #[test]
    fn quiz1() {
        let distance = read_file("../data/2015/input14.txt")
            .lines()
            .map(|line| parse_line(line))
            .map(|d| calc_dist(d, DURATION))
            .max()
            .unwrap();
        assert_eq!(distance, 2655);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds."),
            (8, 8, 53)
        );
    }

    #[test]
    fn test_calc_dist_simple() {
        assert_eq!(calc_dist((14, 10, 127), 1), 14);
        assert_eq!(calc_dist((14, 10, 127), 10 + 127), 14 * 10);
    }

    #[test]
    fn test_calc_dist() {
        assert_eq!(calc_dist((14, 10, 127), 1000), 1120);
        assert_eq!(calc_dist((16, 11, 162), 1000), 1056);
    }
}
