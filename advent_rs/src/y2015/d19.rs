use super::*;
use regex::Regex;

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

/*fn tokenize_molecules(molecules: &str) -> Vec<&str> {
    molecules
        .chars()
        .scan('_', |a, b| {
            if char::is_ascii_uppercase() {

            } else {

            }
        })
}
*/
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

fn replace_rev_all(replacements: &Vec<(&str, &str)>, molecule: &str) -> HashSet<String> {
    replacements
        .iter()
        .flat_map(|r|
            replace(*r, molecule, true))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn quiz2_1() {
    //     let contents = read_file("../data/2015/input19.txt");
    //     let (replacements, target) = load_data(&contents);
    //
    //     let mut molecules: HashSet<String> = HashSet::new();
    //     molecules.insert(target.to_string());
    //
    //     let mut step = 0;
    //
    //     let mut found = false;
    //
    //     loop {
    //         step += 1;
    //
    //         molecules = molecules
    //             .iter()
    //             .flat_map(|m| replace_rev_all(&replacements, m))
    //             .collect::<HashSet<String>>();
    //
    //         let (min, max) = molecules.iter()
    //             .map(|s| s.len())
    //             .fold((usize::MAX, usize::MIN),
    //                   |(min, max), l| {
    //                       (
    //                           if l < min { l } else { min },
    //                           if l > max { l } else { max }
    //                       )
    //                   });
    //
    //         println!("{} {}/{} {}", step, min, max, molecules.len());
    //
    //         if molecules.contains(&"e".to_string()) {
    //             println!("============= FOUND!!! =============");
    //             found = true;
    //             break;
    //         } else if molecules.len() == 0 {
    //             break;
    //         }
    //     }
    //
    //     dbg!(step, found);
    //
    //     assert_eq!(step, 0);
    // }

    /*#[test]
    fn quiz2() {
        let contents = read_file("../data/2015/input19.txt");
        let (replacements, target) = load_data(&contents);

        let mut molecules: HashSet<String> = HashSet::new();
        molecules.insert(target.to_string());

        let mut step = 0;
        let mut found = false;

        loop {
            step += 1;

            molecules = molecules
                .iter()
                .flat_map(|m| replace_rev_all(&replacements, m))
                .collect::<HashSet<String>>();

            let (min, max) = molecules.iter()
                .map(|s| s.len())
                .fold((usize::MAX, usize::MIN),
                      |(min, max), l| {
                          (
                              if l < min { l } else { min },
                              if l > max { l } else { max }
                          )
                      });

            molecules = molecules
                .into_iter()
                .filter(|m| m.len() == min)
                .collect::<HashSet<String>>();

            println!("{} {}/{} {}", step, min, max, molecules.len());

            if molecules.contains("e") {
                println!("============= FOUND!!! =============");
                found = true;
                break;
            } else if molecules.is_empty() {
                break;
            }
        }

        dbg!(step, found);
        assert_eq!(step, 0);
    }*/

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