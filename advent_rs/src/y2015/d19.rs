#![allow(non_snake_case)]

use super::*;
use regex::Regex;

fn format_molecule(molecule: &str) -> String {
    molecule.replace("Rn", "(")
        .replace("Y", ",")
        .replace("Ar", ")")
}

fn load_data(contents: &str) -> (Vec<(&str, &str)>, &str) {
    let mut lines = contents.lines();

    let re = Regex::new(r"(\w+) => (\w+)").unwrap();

    let replacements: Vec<(&str, &str)> = lines.by_ref()
        .take_while(|line| !line.trim().is_empty())
        .fold(Vec::new(), |mut map: Vec<(&str, &str)>, line| {
            let caps = re.captures(line).unwrap();
            let key = caps.get(1).unwrap().as_str();
            let val = caps.get(2).unwrap().as_str();
            map.push((key, val));
            map
        });

    (replacements, lines.next().unwrap())
}

fn load_data_q2(contents: &str) -> (Vec<(String, Vec<String>)>, Vec<String>) {
    let (replacements, molecules) = load_data(contents);

    let replacements: Vec<(String, Vec<String>)> = replacements
        .iter()
        .map(|(key, vals)| {
            (key.to_string(),
             tokenize_molecules(vals))
        })
        .collect();

    let molecules = tokenize_molecules(molecules);

    (replacements, molecules)
}

fn tokenize_molecules(molecules: &str) -> Vec<String> {
    molecules
        .chars()
        .fold(Vec::new(), |mut a, b| {
            if char::is_ascii_uppercase(&b) {
                a.push(b.to_string());
            } else {
                a.last_mut().unwrap().push(b);
            }
            a
        })
}

fn collapse_simple_inside_RnAr(molecules: &mut Vec<String>,
                               replacements: &Vec<(String, Vec<String>)>) -> bool {
    let mut i = molecules.len() - 1;
    while i > 1 {
        if molecules[i] == "Ar" || molecules[i] == "Y" {
            let element = &molecules[i - 2..=i - 1];
            if let Some((k, _)) = replacements
                .iter()
                .find(|(_, elems)| elems == element) {
                molecules.remove(i - 2);
                molecules[i - 2] = k.clone();
                return true;
            }
        }
        i -= 1;
    }
    false
}

fn collapse_simple(molecules: &mut Vec<String>,
                   replacements: &Vec<(String, Vec<String>)>) -> bool {
    let mut i = molecules.len() - 1;
    while i > 0 {
        let element = &molecules[i - 1..=i];
        if let Some((k, _)) = replacements
            .iter()
            .find(|(_, elems)| elems == element) {
            molecules.remove(i - 1);
            molecules[i - 1] = k.clone();
            return true;
        }
        i -= 1;
    }
    false
}

fn find_elements(pos: usize,
                 molecules: &mut Vec<String>,
                 replacements: &Vec<(String, Vec<String>)>) -> Option<(usize, usize)> {
    for (i, (_, elements)) in replacements.iter().enumerate() {
        let len = elements.len();

        if pos >= (len - 1) {
            let start = pos - (len - 1);
            if molecules[start..=pos] == *elements {
                return Some((start, i));
            }
        }
    }
    None
}

fn collapse_RnAr(molecules: &mut Vec<String>,
                 replacements: &Vec<(String, Vec<String>)>) -> bool {
    for i in 0..molecules.len() {
        if molecules[i] == "Ar" {
            if let Some((start, idx)) = find_elements(i, molecules, replacements) {
                let (key, elements) = &replacements[idx];
                for _ in start..start + elements.len() - 1 {
                    molecules.remove(start);
                }
                molecules[start] = key.clone();
                return true;
            }
        }
    }
    false
}

fn replace((key, val): (&str, &str), molecule: &str, reverse: bool) -> HashSet<String> {
    let (key, val) = if reverse { (val, key) } else { (key, val) };
    let key_len = key.len();
    let mut start = 0;

    let mut result = HashSet::new();
    while start < molecule.len() {
        if let Some(idx) = molecule[start..].find(key) {
            let mut str = String::new();
            str.push_str(&molecule[..(start + idx)]);
            str.push_str(val);
            str.push_str(&molecule[(start + idx + key_len)..]);
            result.insert(str);
            start += idx + key_len;
        } else {
            break;
        }
    }

    result
}

fn replace_all(replacements: &Vec<(&str, &str)>, molecule: &str) -> HashSet<String> {
    replacements
        .iter()
        .flat_map(|r|
            replace(*r, molecule, false))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapse_molecules_deep_simple() {
        let contents = read_file("../data/2015/input19.txt");
        let (replacements, _) = load_data_q2(&contents);
        let mut molecules = tokenize_molecules("SiRnTiBPBPMgAr");

        while collapse_simple_inside_RnAr(&mut molecules, &replacements) {
            // println!("{}", molecules.join(""));
            // "SiRnTiBPMgAr"
            // "SiRnTiBFAr"
            // "SiRnTiMgAr"
            // "SiRnMgAr"
        }

        assert_eq!(molecules.join(""), "SiRnMgAr");
    }

    #[test]
    fn quiz2() {
        let contents = read_file("../data/2015/input19.txt");
        let (replacements, mut molecules) = load_data_q2(&contents);

        println!("START {}", format_molecule(&molecules.join("")));

        let mut step = 0;
        loop {
            let old = step;

            while collapse_RnAr(&mut molecules, &replacements) {
                step += 1;
                // println!("{} {}", step, format_molecule(&molecules.join("")));
            }

            while collapse_simple_inside_RnAr(&mut molecules, &replacements) {
                step += 1;
                // println!("{} {}", step, format_molecule(&molecules.join("")));
            }

            if collapse_simple(&mut molecules, &replacements) {
                step += 1;
                // println!("{} {}", step, format_molecule(&molecules.join("")));
            }

            if old == step {
                break;
            }
        }

        println!("Final {} {}", step, format_molecule(&molecules.join("")));
        assert_eq!(step, 195);
    }

    #[test]
    fn test_collapse_molecules_RnAr() {
        let mut molecules = tokenize_molecules(
            "CaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCa");

        let replacements = vec![
            ("P".to_string(), tokenize_molecules("SiRnFAr")),
        ];

        let mut old = molecules.clone();

        loop {
            collapse_RnAr(&mut molecules, &replacements);

            if old == molecules {
                break;
            } else {
                old = molecules.clone();
            }
        }

        assert_eq!(molecules.join(""),
                   "CaSiThCaPRnPTiTiBFArCaCaSiRnSiThCaCa")
    }

    #[test]
    fn quiz1() {
        let contents = read_file("../data/2015/input19.txt");
        let (replacements, molecule) = load_data(&contents);
        assert_eq!(replace_all(&replacements, molecule).len(), 509);
    }

    #[test]
    fn test_replace_all() {
        let rs = replace_all(&vec![
            ("H", "HO"),
            ("H", "OH"),
            ("O", "HH"),
        ], "HOH");
        assert_eq!(rs.len(), 4);
    }

    #[test]
    fn test_replace_not_found() {
        assert_eq!(replace(("H", "HO"), "ASD", false), HashSet::new());
    }

    #[test]
    fn test_replace() {
        let mut r = HashSet::new();
        r.insert("HOOH".to_string());
        r.insert("HOHO".to_string());
        assert_eq!(replace(("H", "HO"), "HOH", false), r);

        let mut r = HashSet::new();
        r.insert("OHOH".to_string());
        r.insert("HOOH".to_string());
        assert_eq!(replace(("H", "OH"), "HOH", false), r);
    }

    #[test]
    fn test_load() {
        let contents = r"H => HO
H => OH
O => HH

CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSi

";
        let (replacements, molecule) = load_data(contents);
        assert_eq!(replacements, vec![
            ("H", "HO"),
            ("H", "OH"),
            ("O", "HH"),
        ]);

        assert_eq!(molecule, "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSi");
    }
}