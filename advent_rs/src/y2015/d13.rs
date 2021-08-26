use regex::Regex;

fn parse_line(line: &str) -> (&str, &str, i32) {
    let re = Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$").unwrap();
    let cap = re.captures(line).unwrap();

    let from = cap.get(1).unwrap().as_str();
    let sign = cap.get(2).unwrap().as_str();
    let happiness = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let to = cap.get(4).unwrap().as_str();

    (from, to,
     if sign == "gain" { happiness } else { -happiness })
}

fn parse_input_data(text: &str) -> Vec<(&str, &str, i32)> {
    text.lines()
        .map(|line| parse_line(line))
        .collect()
}

fn find_happiness(g1: &str, g2: &str, happiness: &Vec<(&str, &str, i32)>) -> i32 {
    happiness
        .iter()
        .find_map(|(a, b, h)|
            if *a == g1 && *b == g2 { Some(*h) } else { None })
        .unwrap()
}

fn calc_happiness(arrange: &Vec<&str>, happiness: &Vec<(&str, &str, i32)>) -> i32 {
    let mut h = 0;

    for (i, guest) in arrange.iter().enumerate() {
        let next = arrange[(i + 1) % arrange.len()];
        h += find_happiness(guest, next, happiness);
    }

    for (i, guest) in arrange.iter().rev().enumerate() {
        let next = arrange[arrange.len() - (i + 1) % arrange.len() - 1];
        h += find_happiness(guest, next, happiness);
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::y2015::{collect_keys, permutations, read_file};

    #[test]
    fn quiz1() {
        let data = read_file("../data/2015/input13.txt");
        let happiness = parse_input_data(&data);
        let guests = collect_keys(&happiness);
        let perms = permutations(&guests);

        let max_change = perms
            .iter()
            .map(|p| calc_happiness(p, &happiness))
            .max()
            .unwrap();

        assert_eq!(max_change, 664); // 452
    }

    #[test]
    fn quiz2() {
        let me = "ME";

        let data = read_file("../data/2015/input13.txt");
        let mut happiness = parse_input_data(&data);
        let mut guests = collect_keys(&happiness);

        if !guests.insert(me) {
            panic!("try other name");
        }

        for guest in &guests {
            happiness.push((me, *guest, 0));
            happiness.push((*guest, me, 0));
        }

        let perms = permutations(&guests);

        let max_change = perms
            .iter()
            .map(|p| calc_happiness(p, &happiness))
            .max()
            .unwrap();

        assert_eq!(max_change, 640);
    }

    #[test]
    fn test_calc_happiness() {
        /*
        +41 +46
+55   David    -2
Carol       Alice
+60    Bob    +54
     -7  +83
         */

        /*
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
         */
        let happiness = vec![
            ("Alice", "Bob", 54),
            ("Alice", "Carol", 79),
            ("Alice", "David", -2),
            ("Bob", "Alice", 83),
            ("Bob", "Carol", -7),
            ("Bob", "David", -63),
            ("Carol", "Alice", -62),
            ("Carol", "Bob", 60),
            ("Carol", "David", 55),
            ("David", "Alice", 46),
            ("David", "Bob", -7),
            ("David", "Carol", 41),
        ];

        let arrange = vec!["David", "Alice", "Bob", "Carol"];
        assert_eq!(calc_happiness(&arrange, &happiness), 330);

    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("Alice would gain 54 happiness units by sitting next to Bob."),
                   ("Alice", "Bob", 54));

        assert_eq!(parse_line("Carol would lose 62 happiness units by sitting next to Alice."),
                   ("Carol", "Alice", -62));
    }
}