
use std::collections::HashSet;

fn solve(input: &String) -> Vec<Result<usize, String>> {
    vec![Ok(deliver(input, false)), Ok(deliver(input, true))]
}

fn deliver(input: &String, robo_active: bool) -> usize {
    let mut visited = HashSet::<(isize, isize)>::new();
    visited.insert((0,0));
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    
    for (i, c) in (1..).zip(input.chars()) {
        let delta = get_delta(c);
        if !robo_active || i % 2 == 0 {
            santa_pos = (santa_pos.0 + delta.0, santa_pos.1 + delta.1);
            visited.insert(santa_pos);
        } else {
            robo_pos = (robo_pos.0 + delta.0, robo_pos.1 + delta.1);
            visited.insert(robo_pos);
        }
    }
    visited.len()
}

fn get_delta(c: char) -> (isize, isize) {
    match c {
        '>' => (1, 0),
        '^' => (0, 1),
        'v' => (0, -1),
        '<' => (-1, 0),
        _ => (0, 0),
    }
}
