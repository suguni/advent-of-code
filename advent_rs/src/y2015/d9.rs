#![allow(dead_code)]

use std::collections::HashSet;
use regex::Regex;

fn all_paths<T: Eq + Clone + std::hash::Hash>(locs: &HashSet<T>) -> HashSet<Vec<T>> {
    let mut result = HashSet::new();

    if locs.is_empty() {
        result.insert(Vec::new());
        return result;
    }

    for elem in locs {
        let mut sub_locs = locs.clone();
        sub_locs.remove(elem);

        let sub_paths = all_paths(&sub_locs);

        for mut sub_path in sub_paths {
            sub_path.insert(0, elem.clone());
            result.insert(sub_path);
        }
    }

    result
}

fn parse_line(line: &str) -> (String, String, i32) {
    let re = Regex::new(r"^(.+) to (.+) = (\d+)").unwrap();
    let cas = re.captures(line).unwrap();
    let from = String::from(&cas[1]);
    let to = String::from(&cas[2]);
    let dist = (&cas[3]).parse::<i32>().unwrap();
    (from, to, dist)
}

fn collect_locations(items: &Vec<(String, String, i32)>) -> HashSet<String> {
    let mut result = HashSet::new();
    for (n1, n2, _) in items {
        result.insert(n1.clone());
        result.insert(n2.clone());
    }
    result
}

fn calc_dist(items: &Vec<(String, String, i32)>, path: &Vec<String>) -> i32{

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
    use crate::y2015::read_file;

    #[test]
    fn quiz1() {
        let items: Vec<(String, String, i32)> = read_file("../data/2015/input9.txt")
            .lines()
            .map(|line| parse_line(line))
            .collect();

        let locations = collect_locations(&items);

        let max_dist = all_paths(&locations)
            .iter()
            .map(|path| {
                let dist = calc_dist(&items, &path);
                // println!("{} : {:?}", dist, path);
                dist
            })
            .min()
            .unwrap();

        assert_eq!(207, max_dist);
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

        assert_eq!(expected, collect_locations(&items));
    }

    #[test]
    fn test_comb_string() {
        let mut locations = HashSet::new();
        locations.insert("a".to_string());
        locations.insert("b".to_string());
        locations.insert("c".to_string());

        let paths = all_paths(&locations);
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

        let paths = all_paths(&locations);
        let mut expected = HashSet::new();
        expected.insert(vec![1, 2]);
        expected.insert(vec![2, 1]);

        assert_eq!(paths, expected);
    }
}