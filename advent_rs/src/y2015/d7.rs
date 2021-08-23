use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Expr {
    Val(u16),
    Var(String),
    And(String, String),
    Or(String, String),
    Not(String),
    LShift(String, u8),
    RShift(String, u8),
}

fn parse_line(line: &str) -> (&str, &str) {
    let vs: Vec<&str> = line.split(" -> ").collect();
    (vs[0], vs[1])
}

fn parse_expr(expr: &str) -> Expr {
    let rs: Vec<&str> = expr.split_whitespace().collect();
    match rs.len() {
        1 => {
            if let Ok(value) = rs[0].parse::<u16>() {
                Expr::Val(value)
            } else {
                Expr::Var(rs[0].to_string())
            }
        }
        2 => {
            if rs[0] == "NOT" {
                Expr::Not(rs[1].to_string())
            } else {
                panic!("unsupported expression: {}", expr)
            }
        }
        3 => match rs[1] {
            "AND" => Expr::And(rs[0].to_string(), rs[2].to_string()),
            "OR" => Expr::Or(rs[0].to_string(), rs[2].to_string()),
            "LSHIFT" => Expr::LShift(rs[0].to_string(), rs[2].parse::<u8>().unwrap()),
            "RSHIFT" => Expr::RShift(rs[0].to_string(), rs[2].parse::<u8>().unwrap()),
            _ => panic!("unsupported expression: {}", expr),
        },
        _ => panic!("unsupported expression: {}", expr),
    }
}

fn load_program_from_file(filename: &str) -> VecDeque<(String, Expr)> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    load_program(&contents)
}

fn load_program(source: &str) -> VecDeque<(String, Expr)> {
    let mut program: VecDeque<(String, Expr)> = VecDeque::new();

    for line in source.lines() {
        let (expr, key) = parse_line(line);
        let expr = parse_expr(expr);
        program.push_back((key.to_string(), expr));
    }

    program
}

fn execute_program(program: &mut VecDeque<(String, Expr)>) -> HashMap<String, u16> {
    let mut result: HashMap<String, u16> = HashMap::new();

    while !program.is_empty() {
        let (var, expr) = program.pop_front().unwrap();

        if let Some(value) = eval_expr(&expr, &result) {
            result.insert(var, value);
        } else {
            program.push_back((var, expr));
        }
    }

    result
}

fn eval_operand(operand: &str, envs: &HashMap<String, u16>) -> Option<u16> {
    if let Ok(value) = operand.parse::<u16>() {
        Some(value)
    } else {
        if let Some(v) = envs.get(operand) {
            Some(*v)
        } else {
            None
        }
    }
}

fn eval_expr(expr: &Expr, envs: &HashMap<String, u16>) -> Option<u16> {
    match expr {
        Expr::Val(v) => Some(*v),
        Expr::Var(var) => {
            let a = eval_operand(var, envs)?;
            Some(a)
        }
        Expr::And(op1, op2) => {
            let a = eval_operand(op1, envs)?;
            let b = eval_operand(op2, envs)?;
            Some(a & b)
        }
        Expr::Or(op1, op2) => {
            let a = eval_operand(op1, envs)?;
            let b = eval_operand(op2, envs)?;
            Some(a | b)
        }
        Expr::Not(op) => {
            let a = eval_operand(op, envs)?;
            Some(!a)
        }
        Expr::LShift(op, sh) => {
            let a = eval_operand(op, envs)?;
            Some(a << sh)
        }
        Expr::RShift(op, sh) => {
            let a = eval_operand(op, envs)?;
            Some(a >> sh)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_eval() {
        let mut program: VecDeque<(String, Expr)> = VecDeque::new();
        program.push_back(("x".to_string(), Expr::Val(123)));
        program.push_back(("y".to_string(), Expr::Val(456)));
        program.push_back(("d".to_string(), Expr::And("x".to_string(), "y".to_string())));
        program.push_back(("e".to_string(), Expr::Or("x".to_string(), "y".to_string())));
        program.push_back(("f".to_string(), Expr::LShift("x".to_string(), 2)));
        program.push_back(("g".to_string(), Expr::RShift("y".to_string(), 2)));
        program.push_back(("h".to_string(), Expr::Not("x".to_string())));
        program.push_back(("i".to_string(), Expr::Not("y".to_string())));

        let result: HashMap<String, u16> = execute_program(&mut program);

        assert_eq!(result.get("d"), Some(&72));
        assert_eq!(result.get("e"), Some(&507));
        assert_eq!(result.get("f"), Some(&492));
        assert_eq!(result.get("g"), Some(&114));
        assert_eq!(result.get("h"), Some(&65412));
        assert_eq!(result.get("i"), Some(&65079));
        assert_eq!(result.get("x"), Some(&123));
        assert_eq!(result.get("y"), Some(&456));
    }

    #[test]
    fn test_parse_and_eval() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        let mut program: VecDeque<(String, Expr)> = load_program(input);

        let result: HashMap<String, u16> = execute_program(&mut program);

        assert_eq!(result.get("d"), Some(&72));
        assert_eq!(result.get("e"), Some(&507));
        assert_eq!(result.get("f"), Some(&492));
        assert_eq!(result.get("g"), Some(&114));
        assert_eq!(result.get("h"), Some(&65412));
        assert_eq!(result.get("i"), Some(&65079));
        assert_eq!(result.get("x"), Some(&123));
        assert_eq!(result.get("y"), Some(&456));
    }

    #[test]
    fn run_quiz1() {
        let mut program = load_program_from_file("input7.txt");
        let result: HashMap<String, u16> = execute_program(&mut program);
        assert_eq!(result.get("a"), Some(&46065));
    }

    #[test]
    fn run_qui2() {
        // 1674 -> b
        let mut program = load_program_from_file("input7.txt");
        for (key, expr) in &mut program {
            if key == "b" {
                *expr = Expr::Val(46065);
            }
        }

        let result: HashMap<String, u16> = execute_program(&mut program);
        assert_eq!(result.get("a"), Some(&14134));
    }
}
