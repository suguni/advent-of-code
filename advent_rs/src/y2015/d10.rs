/*
--- Day 10: Elves Look, Elves Say ---

Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading
 aloud the previous sequence and using that reading as the next sequence.
 For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).

Look-and-say sequences are generated iteratively, using the previous value as input for
 the next step. For each step, take the previous value, and replace each run of digits (like 111)
  with the number of digits (3) followed by the digit itself (1).

For example:

    1 becomes 11 (1 copy of digit 1).
    11 becomes 21 (2 copies of digit 1).
    21 becomes 1211 (one 2 followed by one 1).
    1211 becomes 111221 (one 1, one 2, and two 1s).
    111221 becomes 312211 (three 1s, two 2s, and one 1).

Starting with the digits in your puzzle input, apply this process 40 times.
What is the length of the result?

Your puzzle input is 1321131112.
 */

fn say(seq: &str) -> String {
    if seq.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    let mut chars = seq.chars();
    let mut top_count = 1;
    let mut top_char = chars.next().unwrap();

    for c in chars {
        if top_char != c {
            result.push_str(format!("{}", top_count).as_str());
            result.push(top_char);
            top_char = c;
            top_count = 1;
        } else {
            top_count += 1;
        }
    }

    result.push_str(format!("{}", top_count).as_str());
    result.push(top_char);

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_easy_case() {
        assert_eq!(say("1"), "11".to_string());
        assert_eq!(say("2"), "12".to_string());
        assert_eq!(say("11"), "21".to_string());
        assert_eq!(say("12"), "1112".to_string());
        assert_eq!(say("23"), "1213".to_string());
        assert_eq!(say("21"), "1211".to_string());
        assert_eq!(say("1211"), "111221".to_string());
        assert_eq!(say("111221"), "312211".to_string());
    }

    #[test]
    fn quiz1() {
        let mut input = String::from("1321131112");
        for _ in 0..40 {
            input = say(&input);
        }

        // println!("{}", input);
        assert_eq!(input.len(), 492982);
    }

    #[test]
    fn quiz2() {
        let mut input = String::from("1321131112");
        for _ in 0..50 {
            input = say(&input);
        }

        // println!("{}", input);
        assert_eq!(input.len(), 6989950);
    }
}
