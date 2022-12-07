use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, digit1, newline, not_line_ending, space1};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use regex::Regex;

use crate::read_file;

const FILE_NAME: &str = "data/2022/input7.txt";

#[derive(Debug)]
enum Op<'a> {
    Ls(Vec<File<'a>>),
    Cd(&'a str),
}

#[derive(Debug)]
enum File<'a> {
    Dir(&'a str),
    File(&'a str, usize),
}

fn file(input: &str) -> IResult<&str, File> {
    let (input, (size, name)) = separated_pair(digit1, space1, not_line_ending)(input)?;
    Ok((input, File::File(name, size.parse::<usize>().unwrap())))
}

fn dir(input: &str) -> IResult<&str, File> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dir) = not_line_ending(input)?;
    Ok((input, File::Dir(dir)))
}

fn ls(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, fs) = separated_list1(newline, alt((dir, file)))(input)?;
    Ok((input, Op::Ls(fs)))
}

fn cd(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    Ok((input, Op::Cd(dir)))
}

fn commands(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(newline, alt((ls, cd)))(input)
}

fn proc(input: &str) -> HashMap<PathBuf, usize> {
    let (_, vvs) = commands(input).unwrap();

    let mut current = PathBuf::new();
    let mut fs: HashMap<PathBuf, usize> = HashMap::new();
    fs.insert(PathBuf::from("/"), 0);

    for op in vvs {
        match op {
            Op::Ls(vs) => {
                for file in vs {
                    match file {
                        File::Dir(dir) => {
                            let mut sub = current.clone();
                            sub.push(dir);
                            fs.insert(sub, 0);
                        }
                        File::File(name, size) => {
                            for (fb, vs) in fs.iter_mut() {
                                if current.starts_with(fb) {
                                    *vs += size;
                                }
                            }
                        }
                    }
                }
            }
            Op::Cd(path) => {
                if path == "..".to_owned() {
                    current.pop();
                } else if path == "/".to_owned() {
                    current = PathBuf::from("/");
                } else {
                    current.push(path);
                }
            }
        }
    }
    fs
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
