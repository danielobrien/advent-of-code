use std::io;

fn main() {
    println!("Please input sequence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let mut floor = 0;
    let mut steps = 0;
    for c in input.chars() {
        steps += 1;
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 { break; }
    }
    if floor == -1 {
        println!("First entered a basement at instruction #{}", steps);
    } else {
        println!("Never entered the basement, final floor was {}", floor);
    }
}
