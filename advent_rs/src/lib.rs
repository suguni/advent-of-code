mod y2015;
pub mod y2021;

use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("read error");
    contents
}

pub fn split_text<T>(text: &str, sep: char) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    text.split(sep).map(|s| s.parse().unwrap()).collect()
}

pub fn write_file(filename: &str, text: &str) {
    let mut file = File::create(filename).expect("cannot create file");
    write!(file, "{}", text).expect("write error");
}

pub fn permutations<T: Eq + Clone + std::hash::Hash>(set: &HashSet<T>) -> HashSet<Vec<T>> {
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

pub fn collect_keys<T: Hash + Clone + std::cmp::Eq>(items: &Vec<(T, T, i32)>) -> HashSet<T> {
    let mut result = HashSet::new();
    for (n1, n2, _) in items {
        result.insert(n1.clone());
        result.insert(n2.clone());
    }
    result
}

#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}
