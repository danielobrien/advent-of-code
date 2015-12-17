
use std::fmt::Debug;

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut containers: Vec<usize> = vec![];
    for line in input.lines() {
        containers.push(line.trim().parse::<usize>().unwrap())
    }
    vec![Ok(alt_solve(containers))]
}

fn alt_solve(v: Vec<usize>) -> usize {
    let mut c = 0;
    let len = v.len();
    for i in 1..(1 << len) {
        let mut t = i;
        let mut s = 0;
        for j in v.clone() {
            if t % 2 == 1 { s += j; }
            t /= 2;
        }
        if s == 25 {
            c +=1
        }
    }
    c
}

fn get_combos<T: Copy + Debug>(v: Vec<T>) -> Vec<Vec<T>> {
    let mut combos: Vec<Vec<T>> = vec![];
    for t in v {
        let co = combos.clone();
        for c in co {
            let mut d = c.clone();
            d.push(t);
            combos.push(d);
        }
        combos.push(vec![t]);
    }
    combos
}
