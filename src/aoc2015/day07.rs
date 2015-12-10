/* --- Day 7: Some Assembly Required ---

    This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates! Unfortunately, little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

    Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

    The included instructions booklet describes how to connect the parts together: x AND y -> z means to connect wires x and y to an AND gate, and then connect its output to wire z.

    For example:

    123 -> x means that the signal 123 is provided to wire x.
    x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
    p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
    NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.
    Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If, for some reason, you'd like to emulate the circuit instead, almost all programming languages (for example, C, JavaScript, or Python) provide operators for these gates.

    For example, here is a simple circuit:

    123 -> x
    456 -> y
    x AND y -> d
    x OR y -> e
    x LSHIFT 2 -> f
    y RSHIFT 2 -> g
    NOT x -> h
    NOT y -> i
    After it is run, these are the signals on the wires:

    d: 72
    e: 507
    f: 492
    g: 114
    h: 65412
    i: 65079
    x: 123
    y: 456
    In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?

    --- Part Two ---

    Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires (including wire a). What new signal is ultimately provided to wire a?

*/

use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Expr {
    Terminal(Terminal),
    Not(Terminal),
    And(Terminal, Terminal),
    Or(Terminal, Terminal),
    LShift(Terminal, Terminal),
    RShift(Terminal, Terminal),
}

#[derive(Debug,Clone)]
enum Terminal {
    Ident(String),
    Literal(u16),
}



pub fn solve(input: &String) -> Vec<Result<u16, String>> {
    let mut v: Vec<(&str, Expr)> = vec![];
    for line in input.lines() {
        let s: Vec<&str> = line.split(" -> ").collect();
        let expr = parse_expr(s[0].clone());
        if expr.is_err() { return vec![Err(expr.unwrap_err())]; }
        let write_to = s[1];
        v.push((write_to, expr.unwrap()));
    }
    let mut map: HashMap<&str, u16> = HashMap::new();
    
    while !v.is_empty() {
        let mut executed_line = 0;
        let mut executed = false;
        for item in v.iter().enumerate() {
            let i = item.0;
            let write_to = (item.1).0;
            let expr = (item.1).1.clone();
            match calc(expr, &map) {
                Some(value) => {
                    map.insert(write_to, value);
                    executed = true;
                    executed_line = i;
                    break;
                },
                None => continue,
            }
        }
        if !executed { println!("failed"); break; }
        v.remove(executed_line);
    }
    let answer = match map.get("a") {
        Some(a) => Ok(*a),
        None => Err(format!("Couldn't find 'a' in {:?}", map)),
    };
    vec![answer]
}

fn calc (expr: Expr, map: &HashMap<&str, u16>) -> Option<u16> {
    use self::Expr::*;
    use self::Terminal::*;
    
    match expr {
        Terminal(t) => match t {
            Ident(ref a) => {
                let s: &str = &a;
                if map.contains_key(s) { 
                    Some(*(map.get(s).expect("Failed on getting value for key")))
                } else { None }
            },
            Literal(b) => Some(b),
        },
        Not(e) => match calc(Terminal(e), map) {
            Some(r) => Some(!r),
            None => None,
        },
        And(lhs, rhs) => {
            let left = calc(Terminal(lhs), map);
            let right = calc(Terminal(rhs), map);
            if left.is_none() || right.is_none() {
                None
            } else {
                Some(left.unwrap() & right.unwrap())
            }
        },
        Or(lhs, rhs) => {
            let left = calc(Terminal(lhs), map);
            let right = calc(Terminal(rhs), map);
            if left.is_none() || right.is_none() {
                None
            } else {
                Some(left.unwrap() | right.unwrap())
            }
        },
        LShift(lhs, rhs) => {
            let left = calc(Terminal(lhs), map);
            let right = calc(Terminal(rhs), map);
            if left.is_none() || right.is_none() {
                None
            } else {
                Some(left.unwrap() << right.unwrap())
            }
        },
        RShift(lhs, rhs) => {
            let left = calc(Terminal(lhs), map);
            let right = calc(Terminal(rhs), map);
            if left.is_none() || right.is_none() {
                None
            } else {
                Some(left.unwrap() >> right.unwrap())
            }
        },
    }
}

fn parse_expr(expr: &str) -> Result<Expr, String> {
    
    let v: Vec<&str> = expr.split_whitespace().collect();
    if v.len() == 1 {
        Ok(Expr::Terminal(parse_terminal(v[0])))
    } else if v.len() == 2 {
        let a: Result<Terminal, String> = if v[0] != "NOT" {
            Err(format!("Expected NOT in: {}", expr))
        } else {
            Ok(parse_terminal(v[1]))
        };
        match a {
            Err(e) => Err(e),
            Ok(n) => Ok(Expr::Not(n))
        }
    } else if v.len() == 3 {
        let lhs = parse_terminal(v[0]);
        let op = v[1];
        let rhs = parse_terminal(v[2]);
        
        if op == "AND" {
            Ok(Expr::And(lhs, rhs))
        } else if op == "OR" {
            Ok(Expr::Or(lhs, rhs))
        } else if op == "LSHIFT" {
            Ok(Expr::LShift(lhs, rhs))
        } else if op == "RSHIFT" {
            Ok(Expr::RShift(lhs, rhs))
        } else {
            Err(format!("Unsupported binary operator: {}", op))
        }
    } else {
        Err(format!("Could not parse: {}", expr))
    }
}

fn parse_terminal(terminal: &str) -> Terminal {
    use self::Terminal::*;
    
    match terminal.parse::<u16>() {
        Err(_) => Ident(terminal.to_string()),
        Ok(a) => Literal(a),
    }
}