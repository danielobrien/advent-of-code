use std::collections::{HashMap, HashSet};

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut medicine = "";
    let mut replacements = vec![];
    for line in input.lines() {
        if line.contains(" => ") {
            let split = line.split(" => ").collect::<Vec<&str>>();
            replacements.push((split[0], split[1]));
        } else if !line.is_empty() {
           medicine = line;
        }
    }
    let p1 = solve_p1(&replacements, medicine);
    let p2 = solve_p2(&replacements, medicine);
    vec![Ok(p1), Ok(p2)]
}

fn solve_p2(replacements: &Vec<(&str, &str)>, medicine: &str) -> usize {
    let mut rev_repl = HashMap::with_capacity(replacements.len());
    let mut repls = vec![];
    for (orig, repl) in replacements.iter().cloned() {
        rev_repl.insert(repl, orig);
        repls.push(repl);
    }
    repls.sort_by(|a, b| b.len().cmp(&a.len())); //sort from longest to shortest
    let mut current = medicine.to_string();
    let mut count = 0;
    'outer: loop {
        for repl in &repls {
            if let Some(r) = rev_repl.get(repl) {
                let inst = current.matches(repl).count();
                count += inst;
                current = current.replace(repl, r);
            }
            if current == "e".to_string() { break 'outer; }
        }
    }
    count
}

fn solve_p1(replacements: &Vec<(&str, &str)>, medicine: &str) -> usize {
    let mut molecules = HashSet::new();
    for replacement in replacements {
            let orig = replacement.0;
            let subst = replacement.1;
        for (i, _) in medicine.match_indices(orig) {
            let mut new = medicine.to_string();    
            for _ in 0..orig.len() {
                new.remove(i);
            }
            for c in subst.chars().rev() {
                new.insert(i, c);
            }
            molecules.insert(new);
        }
        
    }
    molecules.len()
}
