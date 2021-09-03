#![allow(dead_code)]

mod d7;
mod d8;
mod d9;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::HashSet;
use std::hash::Hash;

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("read error");
    contents
}

fn write_file(filename: &str, text: &str) {
    let mut file = File::create(filename).expect("cannot create file");
    write!(file, "{}", text).expect("write error");
}

fn permutations<T: Eq + Clone + std::hash::Hash>(set: &HashSet<T>) -> HashSet<Vec<T>> {
    let mut result = HashSet::new();

    if set.is_empty() {
        result.insert(Vec::new());
        return result;
    }

    for elem in set {
        let mut subset = set.clone();
        subset.remove(elem);

        let sub_paths = permutations(&subset);

        for mut sub_path in sub_paths {
            sub_path.insert(0, elem.clone());
            result.insert(sub_path);
        }
    }

    result
}

fn collect_keys<T: Hash + Clone + std::cmp::Eq>(items: &Vec<(T, T, i32)>) -> HashSet<T> {
    let mut result = HashSet::new();
    for (n1, n2, _) in items {
        result.insert(n1.clone());
        result.insert(n2.clone());
    }
    result
}
