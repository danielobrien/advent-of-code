
#[derive(Debug, PartialEq, Eq)]
enum Op {
    None,
    On,
    Off,
    Toggle,
}

pub fn solve(input: &String) -> Vec<Result<u32, String>> {
    use self::Op::*;
    
    let mut lights: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    for line in input.lines() {
        let mut op = None;
        if line.starts_with("turn on ") {
            op = On;
        } else if line.starts_with("turn off ") {
            op = Off;
        } else if line.starts_with("toggle ") {
            op = Toggle;
        }
        if op == None { panic!("Couldn't read the op"); }
        let range: Vec<Vec<usize>> = line.split_whitespace()
                                .filter(|s| s.chars().next().unwrap_or('a').is_digit(10))
                                .map(|s| s.split(",")
                                            .map(|s| s.parse::<usize>())
                                            .filter(|e| e.is_ok())
                                            .map(|e| e.unwrap())
                                            .collect()
                                )
                                .collect();
        let start_row = *range.get(0).unwrap().get(0).unwrap();
        let start_col = *range.get(0).unwrap().get(1).unwrap();
        let end_row = *range.get(1).unwrap().get(0).unwrap() + 1;
        let end_col = *range.get(1).unwrap().get(1).unwrap() + 1;
        for row in start_row..end_row {
            for col in start_col..end_col {
                lights[row][col] = match op {
                    On => true,
                    Off => false,
                    Toggle => !lights[row][col],
                    None => lights[row][col],
                };
            }
        }
    }
    let mut lights_on = 0;
    for row in lights.into_iter() {
        for col in row.into_iter() {
            if *col { lights_on += 1 } ;
        }
    }
    vec![Ok(lights_on)]
}
