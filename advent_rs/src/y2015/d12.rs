/*
--- Day 12: JSAbacusFramework.io ---

Santa's Accounting-Elves need help balancing the books after a recent order.
Unfortunately, their accounting software uses a peculiar storage format.
That's where you come in.

They have a JSON document which contains a variety of things:
arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings.
Your first job is to simply find all of the numbers throughout the document and add them together.

For example:

    [1,2,3] and {"a":2,"b":4} both have a sum of 6.
    [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
    {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
    [] and {} both have a sum of 0.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?
 */

/*
--- Part Two ---

Uh oh - the Accounting-Elves have realized that they double-counted everything red.

Ignore any object (and all of its children) which has any property with
the value "red". Do this only for objects ({...}), not arrays ([...]).

    [1,2,3] still has a sum of 6.
    [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.
    {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.
    [1,"red",5] has a sum of 6, because "red" in an array has no effect.


 */

use regex::Regex;

fn find_numbers(text: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let re = Regex::new(r"-?\d+").unwrap();
    for m in re.find_iter(text) {
        result.push(m.as_str().parse::<i32>().unwrap());
    }
    result
}

fn find_open_curl_brace(text: &str, start: usize) -> Option<usize> {
    let mut meet_ccb = 0;
    let mut meet_csb = 0;

    for i in (0..start+1).rev() {
        match (text.as_bytes())[i] {
            b'}' => meet_ccb += 1,
            b']' => meet_csb += 1,
            b'[' => {
                if meet_ccb == 0 && meet_csb == 0 {
                    return None
                }
                meet_csb -= 1;
            },
            b'{' => if meet_ccb == 0 {
                return Some(i);
            } else {
                meet_ccb -= 1;
            },
            _ => {}
        }
    }

    None
}

fn find_close_curl_brace(text: &str, start: usize) -> Option<usize> {
    let mut meet_ocb = 0;

    for i in start..text.len() {
        match (text.as_bytes())[i] {
            b'{' => meet_ocb += 1,
            b'}' => if meet_ocb == 0 {
                return Some(i);
            } else {
                meet_ocb -= 1;
            },
            _ => {}
        }
    }

    None
}

fn remove_red_object(text: &str) -> String {
    let re = Regex::new(r#""red""#).unwrap();
    let mut result = String::from(text);
    let mut start = 0;

    loop {
        if start == result.len() {
            break;
        }

        if let Some(m) = re.find(&result[start..]) {
            let ms = m.start() + start;
            let me = m.end() + start;

            if let Some(ss) = find_open_curl_brace(&result, ms) {
                if let Some(es) = find_close_curl_brace(&result, me) {
                    result.replace_range(ss..es + 1, "");
                    start = ss;
                } else {
                    panic!("Invalid format {}", result);
                }
            } else {
                start = me;
            }
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::y2015::*;

    #[test]
    fn quiz1() {
        let json = read_file("../data/2015/input12.txt");
        assert_eq!(find_numbers(&json).iter().sum::<i32>(), 191164);
    }

    #[test]
    fn quiz2() {
        let json = read_file("../data/2015/input12.txt");
        let removed = remove_red_object(&json);
        // write_file("../data/2015/input12-2-output.txt", &removed);
        assert_eq!(find_numbers(&removed).iter().sum::<i32>(), 87842);
    }

    #[test]
    fn test_find_numbers() {
        assert_eq!(find_numbers(r#"{"a":[-1,1]}"#), vec![-1, 1]);
    }

    #[test]
    fn test_re_number() {
        let re = Regex::new(r"-?\d+").unwrap();

        assert_number_matches(&re,
                              r#"{"a":[-1,1]}"#,
                              vec!["-1", "1"]);

        assert_number_matches(&re,
                              r#"{"a":{"b":4},"c":-1} "#,
                              vec!["4", "-1"]);
    }

    fn assert_number_matches(re: &Regex, text: &str, expected: Vec<&str>) {
        let mut matches = re.find_iter(text);
        for t in expected {
            assert_eq!(matches.next().unwrap().as_str(), t);
        }

    }

    #[test]
    fn test_remove_red_object() {
        assert_eq!(remove_red_object("[1,2,3]"),
                   "[1,2,3]".to_string());

        assert_eq!(remove_red_object(r#"[1,{"c":"red","b":2},3]"#),
                   "[1,,3]".to_string());

        assert_eq!(remove_red_object(r#"{"d":"red","e":[1,2,3,4],"f":5}"#),
                   "".to_string());

        assert_eq!(remove_red_object(r#"[1,"red",5]"#),
                   r#"[1,"red",5]"#.to_string());

        assert_eq!(remove_red_object(r#"[1,{"c":"red","b":2},3,{"c":"red","b":2}]"#),
                   r#"[1,,3,]"#.to_string());

        assert_eq!(remove_red_object(r#"[1,{"c":["red"],"b":2},3]"#),
                   r#"[1,{"c":["red"],"b":2},3]"#.to_string());
    }
}
