
#[derive(Debug)]
struct Sue {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Sue {
    fn new() -> Sue {
        Sue {
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
}

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut sues = vec![];
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let mut sue = Sue::new();
        for i in 0..split.len() {
            if i % 2 != 0 {
                continue;
            }
            // println!("{}: {}", i, split[i]);
            let mut s = split[i + 1].to_string();
            if s.ends_with(",") || s.ends_with(":") {
                s.pop();
            }
            let val = Some(s.parse::<usize>().unwrap());
            match split[i] {
                "children:" => sue.children = val,
                "cats:" => sue.cats = val,
                "samoyeds:" => sue.samoyeds = val,
                "pomeranians:" => sue.pomeranians = val,
                "akitas:" => sue.akitas = val,
                "vizslas:" => sue.vizslas = val,
                "goldfish:" => sue.goldfish = val,
                "trees:" => sue.trees = val,
                "cars:" => sue.cars = val,
                "perfumes:" => sue.perfumes = val,
                _ => continue,
            }
        }
        sues.push(sue);
    }
    let mut p1 = Err(format!("No Sue found"));
    let mut p2 = Err(format!("No Sue found"));
    for (i, sue) in sues.iter().enumerate() {
        if is_none_or(sue.children, 3) && is_none_or(sue.samoyeds, 2) &&
           is_none_or(sue.akitas, 0) && is_none_or(sue.vizslas, 0) &&
           is_none_or(sue.cars, 2) && is_none_or(sue.perfumes, 1) {
            if p1.is_err() && is_none_or(sue.cats, 7) && is_none_or(sue.trees, 3) &&
               is_none_or(sue.pomeranians, 3) && is_none_or(sue.goldfish, 5) {
                p1 = Ok(i + 1);
            }
            if p2.is_err() && (sue.cats.is_none() || sue.cats.unwrap() > 7) &&
               (sue.trees.is_none() || sue.trees.unwrap() > 3) &&
               (sue.pomeranians.is_none() || sue.pomeranians.unwrap() < 3) &&
               (sue.goldfish.is_none() || sue.goldfish.unwrap() < 5) {
                p2 = Ok(i + 1);
            }
            if p1.is_ok() && p2.is_ok() {
                break;
            }
        }
    }
    vec![p1, p2]
}

fn is_none_or<T: PartialEq>(item: Option<T>, value: T) -> bool {
    match item {
        None => true,
        Some(ref a) if *a == value => true,
        _ => false,
    }
}
