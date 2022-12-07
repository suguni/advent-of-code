use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::read_file;

const FILE_NAME: &str = "data/2022/input7.txt";

fn capture_cd(line: &str) -> Option<String> {
    let regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    regex.captures(line).map(|cas| cas[1].to_owned())
}

fn capture_ls(line: &str) -> Option<()> {
    let regex = Regex::new(r"^\$ ls$").unwrap();
    regex.captures(line).map(|cas| ())
}

fn capture_dir(line: &str) -> Option<String> {
    let regex = Regex::new(r"^dir (.+)$").unwrap();
    regex.captures(line).map(|cas| cas[1].to_owned())
}

fn capture_file(line: &str) -> Option<(usize, String)> {
    let regex = Regex::new(r"^(\d+) (.+)$").unwrap();
    regex
        .captures(line)
        .map(|cas| (cas[1].parse::<usize>().unwrap(), cas[2].to_owned()))
}

fn proc(lines: &str) -> HashMap<PathBuf, usize> {
    let mut current = PathBuf::new();
    let mut fs: HashMap<PathBuf, Vec<(usize, String)>> = HashMap::new();
    fs.insert(PathBuf::from("/"), vec![]);

    for line in lines.lines() {
        if let Some(path) = capture_cd(line) {
            if path == "..".to_owned() {
                current.pop();
            } else {
                current.push(path);
            }
        } else if let Some(()) = capture_ls(line) {
            //
        } else if let Some(dir) = capture_dir(line) {
            let mut sub = current.clone();
            sub.push(dir);
            fs.insert(sub, vec![]);
        } else if let Some((size, name)) = capture_file(line) {
            for (fb, vs) in fs.iter_mut() {
                if current.starts_with(fb) {
                    vs.push((size, name.clone()));
                }
            }
        } else {
            unreachable!();
        }
    }

    fs.into_iter()
        .map(|(p, vs)| (p, vs.iter().map(|(s, _)| s).sum::<usize>()))
        .collect()
}

fn proc1(lines: &str) -> usize {
    proc(lines)
        .iter()
        .map(|(_, &s)| s)
        .filter(|&s| s < 100000)
        .sum::<usize>()
}

fn quiz1() -> usize {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn proc2(lines: &str) -> usize {
    let fs = proc(lines);
    let total = fs.get(Path::new("/")).unwrap();
    let unused = 70000000 - total;
    let delete = 30000000 - unused;
    fs.into_values().filter(|s| *s > delete).min().unwrap()
}

fn quiz2() -> usize {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 95437);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 1723892);
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 24933642);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 8474158);
    }
}
