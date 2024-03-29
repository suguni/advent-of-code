#![allow(dead_code)]

use regex::Regex;

fn parse_line(line: &str) -> (String, String, i32) {
    let re = Regex::new(r"^(.+) to (.+) = (\d+)").unwrap();
    let cas = re.captures(line).unwrap();
    let from = String::from(&cas[1]);
    let to = String::from(&cas[2]);
    let dist = (&cas[3]).parse::<i32>().unwrap();
    (from, to, dist)
}

fn calc_dist(items: &Vec<(String, String, i32)>, path: &Vec<String>) -> i32 {
    let mut total_dist = 0;

    for i in 0..path.len() - 1 {
        let l1 = &path[i];
        let l2 = &path[i + 1];
        let mut found = false;

        for (n1, n2, dist) in items {
            if (n1 == l1 && n2 == l2) || (n1 == l2 && n2 == l1) {
                total_dist += dist;
                found = true;
                break;
            }
        }

        if !found {
            panic!("cannot find location");
        }
    }

    total_dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn quiz1() {
        let items: Vec<(String, String, i32)> = read_file("../data/2015/input9.txt")
            .lines()
            .map(|line| parse_line(line))
            .collect();

        let locations = collect_keys(&items);

        let max_dist = permutations(&locations)
            .iter()
            .map(|path| calc_dist(&items, &path))
            .min()
            .unwrap();

        assert_eq!(207, max_dist);
    }

    #[test]
    fn quiz2() {
        let items: Vec<(String, String, i32)> = read_file("../data/2015/input9.txt")
            .lines()
            .map(|line| parse_line(line))
            .collect();

        let locations = collect_keys(&items);

        let max_dist = permutations(&locations)
            .iter()
            .map(|path| calc_dist(&items, &path))
            .max()
            .unwrap();

        assert_eq!(804, max_dist);
    }

    #[test]
    fn test_calc_dist() {
        let path = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let items = vec![
            ("a".to_string(), "b".to_string(), 10),
            ("b".to_string(), "c".to_string(), 20),
        ];
        assert_eq!(30, calc_dist(&items, &path));

        let items = vec![
            ("b".to_string(), "a".to_string(), 10),
            ("c".to_string(), "b".to_string(), 20),
        ];
        assert_eq!(30, calc_dist(&items, &path));
    }

    #[test]
    fn test_parse_line() {
        let line = "Faerun to Norrath = 129";
        let expected = ("Faerun".to_string(), "Norrath".to_string(), 129);
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_collect_locations() {
        let items = vec![
            ("Faerun".to_string(), "Norrath".to_string(), 129),
            ("Faerun".to_string(), "Tristram".to_string(), 58),
        ];

        let mut expected = HashSet::new();
        expected.insert("Faerun".to_string());
        expected.insert("Tristram".to_string());
        expected.insert("Norrath".to_string());

        assert_eq!(expected, collect_keys(&items));
    }

    #[test]
    fn test_comb_string() {
        let mut locations = HashSet::new();
        locations.insert("a".to_string());
        locations.insert("b".to_string());
        locations.insert("c".to_string());

        let paths = permutations(&locations);
        let mut expected = HashSet::new();
        expected.insert(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        expected.insert(vec!["a".to_string(), "c".to_string(), "b".to_string()]);
        expected.insert(vec!["b".to_string(), "a".to_string(), "c".to_string()]);
        expected.insert(vec!["b".to_string(), "c".to_string(), "a".to_string()]);
        expected.insert(vec!["c".to_string(), "a".to_string(), "b".to_string()]);
        expected.insert(vec!["c".to_string(), "b".to_string(), "a".to_string()]);

        assert_eq!(paths, expected);
    }

    #[test]
    fn test_comb_number() {
        let mut locations = HashSet::new();
        locations.insert(1);
        locations.insert(2);

        let paths = permutations(&locations);
        let mut expected = HashSet::new();
        expected.insert(vec![1, 2]);
        expected.insert(vec![2, 1]);

        assert_eq!(paths, expected);
    }
}
