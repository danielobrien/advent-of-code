/* --- Day 13: Knights of the Dinner Table ---

    In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along! This year, you resolve, will be different. You're going to find the optimal seating arrangement and avoid all those awkward conversations.

    You start by writing up a list of everyone invited and the amount their happiness would increase or decrease if they were to find themselves sitting next to each other person. You have a circular table that will be just big enough to fit everyone comfortably, and so each person will have exactly two neighbors.

    For example, suppose you have only four attendees planned, and you calculate their potential happiness as follows:

    Alice would gain 54 happiness units by sitting next to Bob.
    Alice would lose 79 happiness units by sitting next to Carol.
    Alice would lose 2 happiness units by sitting next to David.
    Bob would gain 83 happiness units by sitting next to Alice.
    Bob would lose 7 happiness units by sitting next to Carol.
    Bob would lose 63 happiness units by sitting next to David.
    Carol would lose 62 happiness units by sitting next to Alice.
    Carol would gain 60 happiness units by sitting next to Bob.
    Carol would gain 55 happiness units by sitting next to David.
    David would gain 46 happiness units by sitting next to Alice.
    David would lose 7  happiness units by sitting next to Bob.
    David would gain 41 happiness units by sitting next to Carol.
    Then, if you seat Alice next to David, Alice would lose 2 happiness units (because David talks so much), but David would gain 46 happiness units (because Alice is such a good listener), for a total change of 44.

    If you continue around the table, you could then seat Bob next to Alice (Bob gains 83, Alice gains 54). Finally, seat Carol, who sits next to Bob (Carol gains 60, Bob loses 7) and David (Carol gains 55, David gains 41). The arrangement looks like this:

         +41 +46
    +55   David    -2
    Carol       Alice
    +60    Bob    +54
         -7  +83
    After trying every other seating arrangement in this hypothetical scenario, you find that this one is the most optimal, with a total change in happiness of 330.

    What is the total change in happiness for the optimal seating arrangement of the actual guest list?
*/

use std::collections::HashMap;
use aoc2015::util::get_permutations;


pub fn solve(input: &String) -> Vec<Result<i64, String>> {
    let mut map = get_map(input);
    let p1 = solve_for(&map);

    let mut my_pref: HashMap<String, i64> = HashMap::new();
    let people: Vec<&str> = map.keys().cloned().collect();
    for person in people {
        map.get_mut(person).unwrap().insert("me".to_string(), 0);
        my_pref.insert(person.to_string(), 0);
    }
    map.insert("me", my_pref);
    let p2 = solve_for(&map);

    vec![Ok(p1), Ok(p2)]
}

fn solve_for(map: &HashMap<&str, HashMap<String, i64>>) -> i64 {
    let people = map.keys().cloned().collect();
    let permutations = get_permutations(people);

    let mut max_happy = i64::min_value();
    for p in permutations {
        let mut happy = 0;
        for (i, pers) in p.iter().enumerate() {
            let left = if i != 0 {p[i-1]} else {p[p.len()-1]};
            let right = if i != p.len()-1 {p[i+1]} else {p[0]};
            let pers_map = map.get(pers).unwrap();
            happy += pers_map.get(left).unwrap() + pers_map.get(right).unwrap();
        }
        if happy > max_happy { max_happy = happy; }
    }
    max_happy
}

fn get_map(input: &String) -> HashMap<&str, HashMap<String, i64>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        //println!("{}", line);
        let data: Vec<&str> = line.split_whitespace().collect();
        let person = data[0];
        let gain = data[2] == "gain";
        let abs = data[3].parse::<i64>().expect("Error parsing change in happiness");
        let delta = if gain {abs} else {-abs};
        let mut neighbour = data[10].to_string();
        neighbour.pop();
        if !map.contains_key(person) {
            map.insert(person, HashMap::new());
        }
        map.get_mut(person).unwrap().insert(neighbour.clone(), delta);
        //println!("Inserted {}: {} {}", person, delta, neighbour);
    }
    map
}