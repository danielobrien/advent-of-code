/* --- Day 18: Like a GIF For Your Yard ---

    After the million lights incident, the fire code has gotten stricter: now, at most ten thousand lights are allowed. You arrange them in a 100x100 grid.

    Never one to let you down, Santa again mails you instructions on the ideal lighting configuration. With so few lights, he says, you'll have to resort to animation.

    Start by setting your lights to the included initial configuration (your puzzle input). A # means "on", and a . means "off".

    Then, animate your grid in steps, where each step decides the next configuration based on the current one. Each light's next state (either on or off) depends on its current state and the current states of the eight lights adjacent to it (including diagonals). Lights on the edge of the grid might have fewer than eight neighbors; the missing ones always count as "off".

    For example, in a simplified 6x6 grid, the light marked A has the neighbors numbered 1 through 8, and the light marked B, which is on an edge, only has the neighbors marked 1 through 5:

    1B5...
    234...
    ......
    ..123.
    ..8A4.
    ..765.
    The state a light should have next is based on its current state (on or off) plus the number of neighbors that are on:

    A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
    A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
    All of the lights update simultaneously; they all consider the same current state before moving to the next.

    Here's a few steps from an example configuration of another 6x6 grid:

    Initial state:
    .#.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####..

    After 1 step:
    ..##..
    ..##.#
    ...##.
    ......
    #.....
    #.##..

    After 2 steps:
    ..###.
    ......
    ..###.
    ......
    .#....
    .#....

    After 3 steps:
    ...#..
    ......
    ...#..
    ..##..
    ......
    ......

    After 4 steps:
    ......
    ......
    ..##..
    ..##..
    ......
    ......
    After 4 steps, this example has four lights on.

    In your grid of 100x100 lights, given your initial configuration, how many lights are on after 100 steps?
*/

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let grid = build_grid(input);
    let steps = 100;
    let p1_grid = play_conway(&grid, vec![], steps);
    let p2_grid = play_conway(&grid, vec![0, 99, 9900, 9999], steps);
    if false {display_grid(&p2_grid);}
    vec![Ok(count_on(&p1_grid)), Ok(count_on(&p2_grid))]
}

fn display_grid(grid: &[bool; 10000]) {
    for i in 0..100 {
        let mut s = String::new();
        for j in 0..100 {
            if grid[i*100 + j] { s.push('#') } else { s.push('.') }
        }
        println!("{}", s);
    }
}

fn count_on(grid: &[bool; 10000]) -> usize {
    let mut count = 0;
    for i in 0..100 {
        for j in 0..100 {
            if grid[i*100 + j] { count += 1; }
        }
    }
    count
}

fn play_conway (starting_grid: &[bool; 10000], locked_on: Vec<usize>, steps: usize) -> [bool; 10000] {
    let mut grid = *starting_grid.clone();
    for lock_coord in &locked_on {
        grid[*lock_coord] = true;
    }
    for _ in 0..steps {
        let mut new_grid = [false; 10000];
        for i in 0..100 {
            for j in 0..100 {
                let curr_pos = i*100 + j;
                let curr_state = grid[curr_pos];
                let active_neighbours = count_active_neighbours(&grid, i, j);
                if curr_state {
                    if active_neighbours == 2 || active_neighbours == 3 {
                        new_grid[curr_pos] = true;
                    }
                } else if active_neighbours == 3 {
                    new_grid[curr_pos] = true;
                }
            }
        }  
        grid = new_grid;     
        for lock_coord in &locked_on {
            grid[*lock_coord] = true;
        }
    }
    grid
}

fn build_grid(input: &String) -> [bool; 10000] {
    let mut grid = [false; 10000];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i*100 + j] = match c {
                '#' => true,
                '.' => false,
                _ => panic!("Unrecognised input")
            }
        }
    }
    grid
}

fn count_active_neighbours(grid: &[bool; 10000], i: usize, j: usize) -> usize { 
    let mut count = 0;
    if i != 0 {
        if j != 0 && grid[(i-1)*100 + j-1] {count += 1;}
        if grid[(i-1)*100 + j] { count += 1; }
        if j != 99 && grid[(i-1)*100 + j+1] { count += 1; }
    }
    if j != 0 && grid[i*100 + j -1] { count += 1; }
    if j != 99 && grid[i*100 + j + 1] { count += 1; }
    if i != 99 {
        if j != 0 && grid[(i+1)*100 + j-1] {count += 1;}
        if grid[(i+1)*100 + j] { count += 1; }
        if j != 99 && grid[(i+1)*100 + j+1] { count += 1; }
    }
    count
}