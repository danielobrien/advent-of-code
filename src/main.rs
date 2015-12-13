use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::env;
use std::collections::BTreeSet;
use aoc2015::day01;
use aoc2015::day02;
use aoc2015::day03;
use aoc2015::day04;
use aoc2015::day05;
use aoc2015::day06;
use aoc2015::day07;
use aoc2015::day08;
use aoc2015::day09;
use aoc2015::day10;
use aoc2015::day11;
use aoc2015::day12;

mod aoc2015;

fn main() {
    let v: BTreeSet<usize> = env::args().map(|arg| arg.parse::<usize>()).filter_map(|r| match r { Ok(n) => Some(n), Err(_) => None }).collect();
    if v.is_empty() {
        println!("Solving all puzzles");
    } else {
        let mut iter = v.iter();
        let head = iter.next().unwrap();
        println!("Solving puzzles: {}", iter.fold(head.to_string(), |acc, x| format!("{}, {}", acc, x)));
    }
    let mut day = 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day01::solve(&get_input(day, "()())(("))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day02::solve(&get_input(day, "2x3x4"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day03::solve(&get_input(day, "^v^v^v^v^v"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day04::solve(&get_input(day, "abcdef"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day05::solve(&get_input(day, "aeiouaeiouaeiou"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day06::solve(&get_input(day, "toggle 0,0 through 999,0"))); } 
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day07::solve(&get_input(day, "123 -> x\n456 -> b\nx AND b -> d\nx OR b -> e\nx LSHIFT 2 -> f\nb RSHIFT 2 -> g\nNOT x -> h\nNOT b -> a"))); } 
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day08::solve(&get_input(day, ""))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day09::solve(&get_input(day, "Dublin to London = 50\nLondon to Dulwich = 20\nDulwich to Dublin = 30"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day10::solve(&get_input(day, "1121"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day11::solve(&get_input(day, "ghijklmn"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, day12::solve(&get_input(day, "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"))); }

}

fn get_input(day: usize, or_else: &str) -> String {
    let file_name = format!("input/day-{:02}.txt", day);
    let mut input = String::new();
    match File::open(file_name) {
        Ok(mut f) => { f.read_to_string(&mut input).unwrap(); () },
        Err(_) => input = or_else.to_string(),
    }
  
    println!("Day {}", day);
    input
}

fn print_results<T, E>(day: usize, results: Vec<Result<T, E>>)
where T: Display,
      E: Display,
{
    for pair in (1..).zip(results) {
        match pair {
            (i, Ok(a)) => println!("  Part {}: {}", i, a),
            (i, Err(e)) => println!("  Could not solve Part {} due to error: {}", i, e),
        }
    }
}
