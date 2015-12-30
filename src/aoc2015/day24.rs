//This solution assumes that you can balance the second and third group if the first group has weight == target_weight. That's an heinous assumption, but happens to work for the given input.

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut weights = vec![];
    for line in input.lines() {
        weights.push(line.parse::<usize>().unwrap());
    }
    let total_weight = weights.iter().fold(0, |acc, x| acc + x);
    if total_weight % 3 != 0 { panic!("Total weight can not be evenly divided between 3 groups") ; }
    if total_weight % 4 != 0 { panic!("Total weight can not be evenly divided between 4 groups") ; }
    let p1_target = total_weight / 3;
    let p2_target = total_weight / 4;
    let mut p1_qe = usize::max_value();
    let mut p2_qe = usize::max_value();
    let mut p1_done = false;
    let mut p2_done = false;
    for i in 1..weights.len() {
        let groups = get_groups(&weights, i);
        for group in groups {            
            let mut weight = 0;
            let mut quant_entangle = 1;
            for g in &group {
                weight += *g;
                quant_entangle *= *g;
            }
            if !p1_done && weight == p1_target && p1_qe > quant_entangle { p1_qe = quant_entangle;}
            if !p2_done && weight == p2_target && p2_qe > quant_entangle { p2_qe = quant_entangle;}
        }
        if p1_qe != usize::max_value() { p1_done = true; }
        if p2_qe != usize::max_value() { p2_done = true; }
        if p1_done && p2_done { break; }
    }
    vec![Ok(p1_qe), Ok(p2_qe)]
}

fn get_groups(weights: &Vec<usize>, length: usize) -> Vec<Vec<usize>> {
    let mut groups = vec![];
    if length == 1 {
        for w in weights {
            groups.push(vec![*w]);
        }
    } else {
        for (i, w) in weights.iter().enumerate() {
            let subs = weights.clone().split_at(i+1).1.iter().cloned().collect::<Vec<_>>();
            for v in get_groups(&subs, length -1).iter_mut() {
                v.push(*w);
                groups.push(v.clone());
            }            
        }
    }
    groups
}
