#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::io::Write;

use regex::Regex;

/*
처음에는 정규식으로 케이스들을 치환한 후 카운트 하였으나 계속 실패
서브레딧 참고하여 단순히 케이스별로 카운트만 하였더니 정확하게 계산됨.
replace 할 때 중복되는 케이스들이 존재하기 때문인것으로 판단됨.
\ 가 여러번 들어갈 경우 replace 를 하나씩하면 문제가 됨
\\x123 이런 경우 \\->\ 로 변경 된 결과를 다시 hex -> char 변경하면 결국 한문자만 남음
하지만 이 경우는 \x123 이 최종 결과이어야 함.
 */

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

fn unescape(text: &str) -> String {
    let mut unescaped = strip_quote(text);
    unescaped = replace_backslash(&unescaped);
    unescaped = replace_hex_to_char(&unescaped);
    replace_quote(&unescaped)
}

fn replace_hex_to_char(text: &str) -> String {
    let re = Regex::new(r"\\x[0-9a-f][0-9a-f]").unwrap();
    let mut result = String::new();
    let mut start = 0;

    for m in re.find_iter(text) {
        if let Ok(_c) = u8::from_str_radix(&text[m.start() + 2..m.end()], 16) {
            result.push_str(&text[start..m.start()]);
            result.push('_'); // char::from_u32(c as u32).unwrap());
            start = m.end();
        }
    }

    result.push_str(&text[start..]);
    result
}

fn strip_quote(text: &str) -> String {
    replace_all(text, r#"(?m)^"|"$"#, "")
}

fn replace_backslash(text: &str) -> String {
    replace_all(text, r#"\\\\"#, r#"\"#)
}

fn replace_quote(text: &str) -> String {
    replace_all(text, r#"\\""#, r#"""#)
}

fn remove_whitespace(text: &str) -> String {
    replace_all(text, r"\s+", "")
}

fn replace_all(text: &str, pat: &str, rep: &str) -> String {
    let re = Regex::new(pat).unwrap();
    re.replace_all(text, rep).to_string()
}

fn count_char(text: &str) -> u32 {
    let mut count = 0;
    for line in text.lines() {
        let l = remove_whitespace(line);
        if line != l {
            println!("{} {}", line.len(), line);
            println!("{} {}", l.len(), l);
        }
        count += line.len() as u32;
    }
    count
}

fn unescaped_mem_count(line: &str) -> (usize, usize) {
    let mut mem: usize = 0;
    let mut i = 1;

    let chars: Vec<char> = line.chars().collect();

    while i < chars.len() - 1 {
        mem += 1;
        match chars[i] {
            '\\' => {
                match chars[i + 1] {
                    'x' => i += 4,
                    '\\' | '"' => i += 2,
                    _ => panic!("Invalid format")
                }

            },
            _ => i += 1,
        }
    }

    (chars.len(), mem)
}

fn unescaped_mem_counts(text: &str) -> (usize, usize) {
    let mut code: usize = 0;
    let mut mem: usize = 0;

    for line in text.lines() {
        let (c, m) = unescaped_mem_count(line);
        mem += m;
        code += c;
    }

    (code, mem)
}

fn escaped_mem_count(line: &str) -> (usize, usize) {
    let mut escaped: usize = 0;

    for c in line.chars() {
        match c {
            '"' | '\\' => escaped += 2,
            _ => escaped += 1
        }
    }

    (line.len(), escaped + 2)
}

fn escaped_mem_counts(text: &str) -> (usize, usize) {
    let mut code: usize = 0;
    let mut escaped: usize = 0;

    for line in text.lines() {
        let (c, e) = escaped_mem_count(line);
        code += c;
        escaped += e;
    }

    (code, escaped)
}

/*
 *  The only escape sequences used are \\ (which represents a single backslash),
 *  \" (which represents a lone double-quote character),
 *  and \x plus two hexadecimal characters
 *  (which represents a single character with that ASCII code).
 */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string_slice() {
        let x = "abcde".to_string();
        assert_eq!(x[1..3], "bc".to_string());
    }

    #[test]
    fn test_regex() {
        let regex = Regex::new(r"\\{2}").unwrap();
        let text = r"hello\\world\!";
        assert_eq!(regex.replace_all(text, "__"), r"hello__world\!");

        let re2 = Regex::new(r"\\x\d\d").unwrap();
        let text = r"hello\x27world";
        assert_eq!(re2.replace_all(text, "'"), "hello'world");
    }

    #[test]
    fn test_strip_quote() {
        let text = r#""hello\"world""#;
        assert_eq!(strip_quote(text), r#"hello\"world"#);
    }

    /*#[test]
    fn test_hex_to_char() {
        let text = r"hello\xa2world";
        assert_eq!(replace_hex_to_char(text), "hello¢world".to_string());

        let text = r"\x27hello\x27world\x27";
        assert_eq!(replace_hex_to_char(text), "'hello'world'".to_string());

        let text = r"hello world";
        assert_eq!(replace_hex_to_char(text), "hello world".to_string());
    }

    #[test]
    fn test_unescaped() {
        let text = r#""lregjexqaqgwloydxdsc\\o\"dnjfmjcu""#;
        let expected = r#"lregjexqaqgwloydxdsc\o"dnjfmjcu"#;
        assert_eq!(unescape(text), expected.to_string());

        let text = r#""lnxluajtk\x8desue\\k\x7abhwokfhh""#;
        let expected = "lnxluajtk\u{8d}esue\\kzbhwokfhh";
        assert_eq!(unescape(text), expected.to_string());
    }*/

    #[test]
    fn test_count_chars() {
        let text = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;
        assert_eq!(count_char(text), 23);
        assert_eq!(count_char(&unescape(text)), 11);
    }

    #[test]
    fn test_unescaped_mem_counts() {
        let text = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;

        let (code, mem) = unescaped_mem_counts(text);
        assert_eq!(code, 23);
        assert_eq!(mem, 11);
    }

    #[test]
    fn quiz1() {
        let text = read_file("../data/2015/input8.txt");
        let (code, mem) = unescaped_mem_counts(&text);
        assert_eq!(code - mem, 1342); // 1345x, 1277x, 1342X
    }

    /*
    #[test]
    fn quiz1() {
        let text = read_file("../data/2015/input8.txt");
        let unescaped = unescape(&text);
        write_file("../data/2015/input8-processed.txt", &unescaped);

        let s1 = count_char(&text);
        let s2 = count_char(&unescaped);

        // assert_eq!(s1, 6202);
        // assert_eq!(s2, 4857);

        assert_eq!(s1 - s2, 1342); // 1345x, 1277x, 1342X
    }
     */

    #[test]
    fn test_escaped_mem_count() {
        assert_eq!(escaped_mem_count(r#""""#), (2, 6));
        assert_eq!(escaped_mem_count(r#""abc""#), (5, 9));
        assert_eq!(escaped_mem_count(r#""aaa\"aaa""#), (10, 16));
        assert_eq!(escaped_mem_count(r#""\x27""#), (6, 11));
    }

    #[test]
    fn test_escaped_mem_counts() {
        let text = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;

        let (code, escaped) = escaped_mem_counts(text);
        assert_eq!(code, 23);
        assert_eq!(escaped, 42);
    }

    #[test]
    fn quiz2() {
        let text = read_file("../data/2015/input8.txt");
        let (code, escaped) = escaped_mem_counts(&text);
        assert_eq!(escaped - code, 2074);
    }
}
