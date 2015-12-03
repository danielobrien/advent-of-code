use std::fmt::Display;
use std::fs::File;
use std::io::Read;

fn main() {
  print_results(day01::solve(&get_input(2, "()())(("))); // 1, 5
  print_results(day02::solve(&get_input(2, "2x3x4"))); // 58, 34
}

fn get_input(day: usize, or_else: &str) -> String {
  let file_name = format!("inputs/day-{:02}.txt", day);
  let mut input = String::new();
  match File::open(file_name) {
    Ok(mut f) => f.read_to_string(&mut input).unwrap(),
    Err(_) => input = or_else.to_string(),
  }
  input
}

fn print_results<T, E>(results: Vec<Result<T, E>>)
where T: Display,
      E: Display,
{
  for pair in (1..).zip(results) {
    match pair {
      (i, Ok(a)) => println!("Part {}: {}", i, a);
      (i, Err(e)) => println!("Could not solve Part {} due to error: {}", i, e);
    }
  }
}
