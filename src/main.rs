use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use aoc2015::day01;
use aoc2015::day02;
use aoc2015::day03;
use aoc2015::day04;
use aoc2015::day05;
use aoc2015::day06;
use aoc2015::day07;

mod aoc2015;

fn main() {
    let mut day = 1;
    print_results(day, day01::solve(&get_input(day, "()())(("))); // 1, 5
    day += 1;
    print_results(day, day02::solve(&get_input(day, "2x3x4"))); // 58, 34
    day += 1;
    print_results(day, day03::solve(&get_input(day, "^v^v^v^v^v"))); //2, 11
    day += 1;
    print_results(day, day04::solve(&get_input(day, "abcdef"))); //609043
    day += 1;
    print_results(day, day05::solve(&get_input(day, "aeiouaeiouaeiou"))); //1, 0
    day += 1;
    print_results(day, day06::solve(&get_input(day, "toggle 0,0 through 999,0"))); //1000, 
    day += 1;
    print_results(day, day07::solve(&get_input(day, "123 -> x\n456 -> b\nx AND b -> d\nx OR b -> e\nx LSHIFT 2 -> f\nb RSHIFT 2 -> g\nNOT x -> h\nNOT b -> a"))); //65079, 
}

fn get_input(day: usize, or_else: &str) -> String {
  let file_name = format!("input/day-{:02}.txt", day);
  let mut input = String::new();
  match File::open(file_name) {
    Ok(mut f) => { f.read_to_string(&mut input).unwrap(); () },
    Err(_) => input = or_else.to_string(),
  }
  input
}

fn print_results<T, E>(day: usize, results: Vec<Result<T, E>>)
where T: Display,
      E: Display,
{
  println!("Day {}", day);
  for pair in (1..).zip(results) {
    match pair {
      (i, Ok(a)) => println!("  Part {}: {}", i, a),
      (i, Err(e)) => println!("  Could not solve Part {} due to error: {}", i, e),
    }
  }
}
