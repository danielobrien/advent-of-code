use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::env;
use std::collections::BTreeSet;

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
    let unimplemented_msg: Result<usize, &str> = Err("Not yet implemented");
    let unimpl = vec![unimplemented_msg];
    let mut day = 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day01::solve(&get_input(day, "()())(("))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day02::solve(&get_input(day, "2x3x4"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day03::solve(&get_input(day, "^v^v^v^v^v"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day04::solve(&get_input(day, "abcdef"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day05::solve(&get_input(day, "aeiouaeiouaeiou"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day06::solve(&get_input(day, "toggle 0,0 through 999,0"))); } 
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day07::solve(&get_input(day, "123 -> x\n456 -> b\nx AND b -> d\nx OR b -> e\nx LSHIFT 2 -> f\nb RSHIFT 2 -> g\nNOT x -> h\nNOT b -> a"))); } 
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day08::solve(&get_input(day, ""))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day09::solve(&get_input(day, "Dublin to London = 50\nLondon to Dulwich = 20\nDulwich to Dublin = 30"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day10::solve(&get_input(day, "1121"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day11::solve(&get_input(day, "ghijklmn"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day12::solve(&get_input(day, "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day13::solve(&get_input(day, "Alice would gain 54 happiness units by sitting next to Bob.\nAlice would lose 79 happiness units by sitting next to Carol.\nAlice would lose 2 happiness units by sitting next to David.\nBob would gain 83 happiness units by sitting next to Alice.\nBob would lose 7 happiness units by sitting next to Carol.\nBob would lose 63 happiness units by sitting next to David.\nCarol would lose 62 happiness units by sitting next to Alice.\nCarol would gain 60 happiness units by sitting next to Bob.\nCarol would gain 55 happiness units by sitting next to David.\nDavid would gain 46 happiness units by sitting next to Alice.\nDavid would lose 7 happiness units by sitting next to Bob.\nDavid would gain 41 happiness units by sitting next to Carol."))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day14::solve(&get_input(day, "Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.\nRudolph can fly 3 km/s for 15 seconds, but then must rest for 28 seconds."))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day15::solve(&get_input(day, "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day16::solve(&get_input(day, "Sue 1: goldfish: 9, cars: 0, samoyeds: 9\nSue 2: perfumes: 5, trees: 8, goldfish: 8"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day17::solve(&get_input(day, "20\n10\n15\n5\n5"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day18::solve(&get_input(day, ".#.#.#\n...##\n#....#\n..#...\n#.#..#\n####.."))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day19::solve(&get_input(day, "H => HO\nH => OH\n=>O => HH\nHOH"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day20::solve(&get_input(day, "34000000"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day21::solve(&get_input(day, "Hit Points: 104\nDamage: 8\nArmor: 1"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, unimpl); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day23::solve(&get_input(day, "inc b\njio b, +2\ntpl b\ninc b"))); }
    day += 1;
    if v.is_empty() || v.contains(&day) { print_results(day, aoc2015::day24::solve(&get_input(day, "1\n2\n3\n4\n5\n7\n8\n9\n10\n11"))); }

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
