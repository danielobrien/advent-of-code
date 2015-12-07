
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    On,
    Off,
    Toggle,
}

fn calculate<T, F>(init: T, calc: F, input: &String) -> [[T; 1000]; 1000] where 
    T: Copy,
    F: Fn(T, Op) -> T
{
    let mut grid = [[init; 1000]; 1000];

    for line in input.lines() {
        let op = match get_op(line) {
            Some(op) => op,
            None => panic!("Couldn't read the op"),
        };
                
        let range = get_range(line);
               
        for row in range.start.x..range.end.x {
            for col in range.start.y..range.end.y {
                grid[row][col] = calc(grid[row][col], op);
            }
        }
    }

    grid
}

pub fn solve(input: &String) -> Vec<Result<u32, String>> {
    use self::Op::*;

    let p1 = solve_p1(input);
    let p2 = solve_p2(input);    

    vec![Ok(p1), Ok(p2)]
}

fn solve_p1(input: &String) -> u32 {
    use self::Op::*;

    let p1_instructions = |x: bool, op: Op| {
        match op {
            On => true,
            Off => false,
            Toggle => !x
        }
    };
    
    count(calculate(false, p1_instructions, input), |x| x, |x| if x {1} else {0})
}

fn solve_p2(input: &String) -> u32 {
    use self::Op::*;

    let p2_instructions = |x: u8, op: Op| {
        match op {
            On => x+1,
            Off => if x > 0 {x-1} else {x},
            Toggle => x+2
        }
    };

    count(calculate(0u8, p2_instructions, input), |x| true , |x| x as u32 )
}

fn count<T, F, P>(grid: [[T; 1000]; 1000], predicate: P, weight: F) -> u32 where 
    T: Copy,
    P: Fn(T) -> bool,
    F: Fn(T) -> u32,
{
    let mut acc = 0;
    for row in grid.into_iter() {
        for col in row.into_iter() {
            if predicate(*col) { acc += weight(*col) } ;
        }
    }
    acc
}

fn solve_part_two(input: &String) -> u32 {
    use self::Op::*;

    let instructions = |x: u8, op: Op| {
        match op {
            On => x+1,
            Off => if x > 0 {x-1} else {x},
            Toggle => x+2
        }
    };
    
    let mut total_brightness = 0;    
    for row in calculate(0u8, instructions, input).into_iter() {
        for col in row.into_iter() {
            total_brightness += *col as u32;
        }
    }
    total_brightness
}


struct Range{start: Point, end: Point}
struct Point{ x: usize, y: usize}

fn get_range(line: &str) -> Range {
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
    Range{start: Point{ x: start_row, y: start_col }, end: Point{ x: end_row, y: end_col } }
}

fn get_op(line: &str) -> Option<Op> {
    use self::Op::*;

    if line.starts_with("turn on ") {
        Some(On)
    } else if line.starts_with("turn off ") {
        Some(Off)
    } else if line.starts_with("toggle ") {
        Some(Toggle)
    } else {
        None
    }
    
}