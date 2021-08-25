/*
--- Day 11: Corporate Policy ---

Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has devised a method of
coming up with a password based on the previous one. Corporate policy dictates that passwords
must be exactly eight lowercase letters (for security reasons), so he finds his new password
by incrementing his old password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase
the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter
to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed
some additional password requirements:

    Passwords must include one increasing straight of at least three letters,
    like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.

    Passwords may not contain the letters i, o, or l, as these letters can be mistaken
    for other characters and are therefore confusing.

    Passwords must contain at least two different, non-overlapping pairs of letters,
    like aa, bb, or zz.

For example:

    hijklmmn meets the first requirement (because it contains the straight hij)
    but fails the second requirement requirement (because it contains i and l).

    abbceffg meets the third requirement (because it repeats bb and ff)
    but fails the first requirement.

    abbcegjk fails the third requirement, because it only has one double letter (bb).

    The next password after abcdefgh is abcdffaa.

    The next password after ghijklmn is ghjaabcc, because you eventually skip all
    the passwords that start with ghi..., since i is not allowed.

Given Santa's current password (your puzzle input), what should his next password be?

Your puzzle input is vzbxkghb.
 */

fn rule3(input: &str) -> bool {
    // Passwords must contain at least two different, non-overlapping pairs of letters,
    // like aa, bb, or zz.

    let bytes: Vec<u8> = input.bytes().collect();
    let mut count = 0;

    if bytes.len() < 2 {
        return false;
    }

    let mut c = bytes[0];

    let mut i  = 1;

    while i < bytes.len() {
        if c == bytes[i] {
            count += 1;
            if count >= 2 {
                return true;
            }

            if i + 1 >= bytes.len() {
                return false;
            }

            c = bytes[i + 1];
            i += 2;
        } else {
            c = bytes[i];
            i += 1;
        }
    }

    false
}

fn rule2(input: &str) -> bool {
    !input.is_empty() && !input.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

fn rule1(input: &str) -> bool {
    // Passwords must include one increasing straight of at least three letters,
    // like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.

    let bytes: Vec<u8> = input.bytes().collect();

    if bytes.len() < 3 {
        return false;
    }

    let mut inc = 0;

    for i in 0..bytes.len() - 1 {
        if bytes[i] + 1 == bytes[i + 1] {
            inc += 1;
            if inc >= 2 {
                return true;
            }
        } else {
            inc = 0;
        }
    }

    false
}

fn increment(input: &str) -> String {
    let mut bytes: Vec<u8> = input.bytes().collect();
    let mut overflow = false;

    for i in (0..bytes.len()).rev() {
        bytes[i] += 1;
        if bytes[i] > b'z' {
            bytes[i] = b'a';
            if i == 0 {
                overflow = true;
                break;
            }
        } else {
            break;
        }
    }

    if overflow {
        bytes.insert(0, b'a');
    }

    String::from_utf8(bytes).unwrap()
}

fn next_password(pwd: &str) -> String {
    let mut next = String::from(pwd);

    loop {
        next = increment(&next);
        if rule1(&next) && rule2(&next) && rule3(&next) {
            break;
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiz1() {
        let pwd = "vzbxkghb";
        assert_eq!(next_password(pwd), "vzbxxyzz");
    }

    #[test]
    fn quiz2() {
        let pwd = "vzbxkghb";
        assert_eq!(next_password(&next_password(pwd)), "vzcaabcc");
    }

    #[test]
    fn test_increment() {
        assert_eq!(increment("a"), "b".to_string());
        assert_eq!(increment("aa"), "ab".to_string());
        assert_eq!(increment("z"), "aa".to_string());
        assert_eq!(increment("az"), "ba".to_string());
    }

    #[test]
    fn test_rule1() {
        assert_eq!(rule1("abc"), true);
        assert_eq!(rule1("abd"), false);
        assert_eq!(rule1("acd"), false);
    }

    #[test]
    fn test_rule2() {
        assert_eq!(rule2("abc"), true);
        assert_eq!(rule2("abia"), false);
        assert_eq!(rule2("als"), false);
        assert_eq!(rule2("ado"), false);
        assert_eq!(rule2("xdwbcd"), true);
    }

    #[test]
    fn test_rule3() {
        assert_eq!(rule3("abc"), false);
        assert_eq!(rule3("aa"), false);
        assert_eq!(rule3("xyaaoibb"), true);
    }
}