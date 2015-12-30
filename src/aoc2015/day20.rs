
pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let target = input.trim().parse::<usize>().expect("Could not parse target as usize");
    vec![Ok(get_house_number_getting_at_least(target)), Ok(p2(target))]
}

fn get_house_number_getting_at_least(target: usize) -> usize {
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