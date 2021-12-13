use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Node {
    Start,
    Big(String),
    Small(String),
    End,
}

fn gen_node(name: &str) -> Node {
    if name == "start" {
        Node::Start
    } else if name == "end" {
        Node::End
    } else {
        if name.chars().all(|c| c.is_uppercase()) {
            Node::Big(name.to_string())
        } else {
            Node::Small(name.to_string())
        }
    }
}

fn load_data(text: &str) -> HashMap<Node, HashSet<Node>> {
    text.lines().fold(HashMap::new(), |mut map, line| {
        let mut s = line.split('-');
        let left = gen_node(s.next().unwrap());
        let right = gen_node(s.next().unwrap());

        let v = map.entry(left.clone()).or_insert(HashSet::new());
        v.insert(right.clone());

        let v = map.entry(right).or_insert(HashSet::new());
        v.insert(left);

        map
    })
}

fn tranverse_map(
    links: &mut HashMap<Node, HashSet<Node>>,
    can_visit_small: fn(visited: &HashMap<Node, usize>, node: &Node) -> bool,
) -> Vec<Vec<Node>> {
    let mut completed: Vec<Vec<Node>> = vec![];

    let mut paths: Vec<(Vec<Node>, HashMap<Node, usize>)> =
        vec![(vec![Node::Start], HashMap::new())];

    while let Some((path, visited)) = paths.pop() {
        let last = path.last().unwrap();

        for next in links[&last].iter() {
            let mut next_path = path.clone();
            let mut next_visited = visited.clone();

            match next {
                Node::End => {
                    next_path.push(next.clone());
                    completed.push(next_path);
                }
                Node::Start => {}
                Node::Big(_) => {
                    next_path.push(next.clone());
                    paths.push((next_path, next_visited));
                }
                Node::Small(_) => {
                    if can_visit_small(&next_visited, &next) {
                        next_path.push(next.clone());
                        let e = next_visited.entry(next.clone()).or_insert(0);
                        *e += 1;
                        paths.push((next_path, next_visited));
                    }
                }
            }
        }
    }

    completed
}

fn can_visit_quiz1(visited: &HashMap<Node, usize>, node: &Node) -> bool {
    visited.get(node).is_none()
}

pub fn quiz1(text: &str) -> usize {
    let mut links = load_data(text);
    let paths = tranverse_map(&mut links, can_visit_quiz1);
    paths.len()
}

fn can_visit_quiz2(visited: &HashMap<Node, usize>, node: &Node) -> bool {
    if let Some(count) = visited.get(node) {
        if *count > 1 {
            return false;
        }
    } else {
        return true;
    }

    visited.values().all(|v| *v <= 1)
}

pub fn quiz2(text: &str) -> usize {
    let mut links = load_data(text);
    let paths = tranverse_map(&mut links, can_visit_quiz2);
    paths.len()
}

#[allow(unused)]
fn print_debug(completed: &Vec<Vec<Node>>, paths: &Vec<(Vec<Node>, HashMap<Node, usize>)>) {
    let cs = completed
        .iter()
        .map(|path| path_to_string(path))
        .collect::<Vec<String>>();

    let ps = paths
        .iter()
        .map(|(path, visited)| {
            format!("{} || {}", path_to_string(path), visited_to_string(visited))
        })
        .collect::<Vec<String>>();

    println!("{:?} ___ {:?}", cs, ps);
}

#[allow(unused)]
fn visited_to_string(visited: &HashMap<Node, usize>) -> String {
    visited
        .iter()
        .fold(String::new(), |mut acc, (node, count)| {
            acc.push_str(format!("{}{}", node_string(node), count).as_str());
            acc
        })
}

#[allow(unused)]
fn path_to_string(path: &Vec<Node>) -> String {
    path.iter()
        .map(|n| node_string(n))
        .collect::<Vec<&str>>()
        .join(",")
}

#[allow(unused)]
fn node_string(node: &Node) -> &str {
    match node {
        Node::Start => "start",
        Node::End => "end",
        Node::Big(id) => id.as_str(),
        Node::Small(id) => id.as_str(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const DATA1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const DATA2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const DATA3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_to_string() {
        assert_eq!(
            path_to_string(&vec![
                Node::Start,
                Node::Big("A".to_string()),
                Node::Small("b".to_string()),
                Node::End
            ]),
            "start,A,b,end"
        );
    }

    #[test]
    fn test_gen_node() {
        assert_eq!(gen_node("start"), Node::Start);
        assert_eq!(gen_node("A"), Node::Big("A".to_string()));
        assert_eq!(gen_node("dc"), Node::Small("dc".to_string()));
        assert_eq!(gen_node("end"), Node::End);
    }

    #[test]
    fn test_load_data() {
        let links = load_data(DATA1.trim());

        // start A b c d end
        assert_eq!(links.len(), 6);
        assert_eq!(
            links[&Node::Start],
            set![Node::Big("A".to_string()), Node::Small("b".to_string())]
        );
        assert_eq!(
            links[&Node::Small("b".to_string())],
            set![
                Node::Start,
                Node::Big("A".to_string()),
                Node::Small("d".to_string()),
                Node::End
            ]
        );
    }

    #[test]
    fn test_traverse() {
        let mut links = load_data(DATA1.trim());

        let paths = tranverse_map(&mut links, can_visit_quiz1);

        let paths = paths
            .iter()
            .map(|path| path_to_string(&path))
            .collect::<HashSet<String>>();

        let expected = "start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,end
start,A,c,A,b,A,end
start,A,c,A,b,end
start,A,c,A,end
start,A,end
start,b,A,c,A,end
start,b,A,end
start,b,end"
            .lines()
            .map(|line| line.to_string())
            .collect::<HashSet<String>>();
        assert_eq!(paths, expected);
    }

    #[test]
    fn test_traverse2() {
        let mut links = load_data(DATA1.trim());

        let paths = tranverse_map(&mut links, can_visit_quiz2);

        let paths = paths
            .iter()
            .map(|path| path_to_string(&path))
            .collect::<HashSet<String>>();

        let expected = "start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end"
            .lines()
            .map(|line| line.to_string())
            .collect::<HashSet<String>>();

        assert_eq!(paths, expected);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(DATA1.trim()), 10);
        assert_eq!(quiz1(DATA2.trim()), 19);
        assert_eq!(quiz1(DATA3.trim()), 226);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(DATA1.trim()), 36);
        assert_eq!(quiz2(DATA2.trim()), 103);
        assert_eq!(quiz2(DATA3.trim()), 3509);
    }

    #[test]
    fn run_2021_d12_quiz1() {
        let text = read_file("data/2021/input12.txt");
        assert_eq!(quiz1(text.as_str().trim()), 4167);
    }

    #[test]
    fn run_2021_d12_quiz2() {
        let text = read_file("data/2021/input12.txt");
        assert_eq!(quiz2(text.as_str().trim()), 98441);
    }
}
