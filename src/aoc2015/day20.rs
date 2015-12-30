
pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let target = input.parse::<usize>().unwrap();
    vec![Ok(get_house_number_getting_at_least(target)), Ok(p2(target))]
}

fn get_house_number_getting_at_least(target: usize) -> usize {
    // let min_house = get_triangular_root(target/10);
    // println!("starting at {}", min_house);
    // for h in min_house..(target/10) {
    //     let mut presents = 0;
    //     for n in get_factors(h) {
    //         presents += n;
    //     }
    //     presents *= 10;
    //     if presents >= target { return h }
    // }
    let max_house = target/10;
    let mut house = Vec::with_capacity(max_house);
    for _ in 0..max_house+1 {
        house.push(0);
    }
    for elf in 1..max_house+1 {
        let mut j = elf;
        while j <= max_house {
            house[j] += elf * 10;
            j += elf;
        }
    }
    for i in 0..max_house+1 {
        if house[i] >= target { return i; }
    }
    0
}

fn p2(target: usize) -> usize {
    let max_house = target/10;
    let mut house = Vec::with_capacity(max_house);
    for _ in 0..max_house+1 {
        house.push(0);
    }
    for elf in 1..max_house+1 {
        let mut house_num = elf;
        let mut houses_visited = 0;
        while house_num <= max_house {
            house[house_num] += elf * 11;
            house_num += elf;
            houses_visited += 1;
            if houses_visited > 50 { break; }
        }
    }
    for i in 0..max_house+1 {
        if house[i] >= target { return i; }
    }
    0
}

fn get_triangular_root(n: usize) -> usize {
    let f = n as f64;
    let root = (f64::sqrt(8.0*f + 1.0) - 1.0 ) / 2.0;
    root.floor() as usize
}

fn get_factors(n: usize) -> Vec<usize> {
    let mut factors = vec![1, n];
    let root = f64::sqrt(n as f64).round() as usize;
    for x in root..n-1 {
        if n % x == 0 {
            factors.push(x);
            factors.push(n/x);
        }
    }
    factors
}
