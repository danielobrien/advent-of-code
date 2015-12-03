

fn main() {
    println!("Please input sequence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
        
    let mut floor = 0;
    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }
    println!("{}", floor);
}
