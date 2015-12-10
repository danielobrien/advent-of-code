/* --- Day 9: All in a Single Night ---

    Every year, Santa manages to deliver all of his presents in a single night.

    This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations. He can start and end at any two (different) locations he wants, but he must visit each location exactly once. What is the shortest distance he can travel to achieve this?

    For example, given the following distances:

    London to Dublin = 464
    London to Belfast = 518
    Dublin to Belfast = 141
    The possible routes are therefore:

    Dublin -> London -> Belfast = 982
    London -> Dublin -> Belfast = 605
    London -> Belfast -> Dublin = 659
    Dublin -> Belfast -> London = 659
    Belfast -> Dublin -> London = 605
    Belfast -> London -> Dublin = 982
    The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.

    What is the distance of the shortest route?

    --- Part Two ---

    The next year, just to show off, Santa decides to take the route with the longest distance instead.

    He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.

    For example, given the distances above, the longest route would be 982 via (for example) Dublin -> London -> Belfast.

    What is the distance of the longest route?

*/

use std::collections::{HashMap, HashSet};


pub fn solve(input: &String) -> Vec<Result<u32, String>> {
    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);
    vec![Ok(range.0), Ok(range.1)]
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Route<'a> {
    origin: &'a str,
    dest: &'a str
}

fn route_len_range(routes: HashMap<Route, u32>, cities: Vec<&str>) -> (u32, u32) {
    let permutations = get_permutations(cities);
    //println!("Got permutations");
    let mut min_dist = u32::max_value();
    let mut max_dist = 0;
    for p in permutations {
        let mut dist = 0;
        for i in 0..p.len()-1 {
            let c1 = p.get(i).unwrap();
            let c2 = p.get(i+1).unwrap();
            let r = Route{origin: c1, dest: c2};
            dist += *routes.get(&r).unwrap();
        }
        if dist < min_dist { min_dist = dist; }
        if dist > max_dist { max_dist = dist; }
    }
    (min_dist, max_dist)
}

fn get_permutations<T: Clone>(v: Vec<T>) -> Vec<Vec<T>> {
    match v.len() {
        0 | 1 => vec![v],
        2 => {
            let rev0 = v.get(1).unwrap().clone();
            let rev1 = v.get(0).unwrap().clone();
            vec![v, vec![rev0, rev1]]
        },
        _ => {
            let mut permutations = vec![];
            for i in 0..v.len() {
                let mut v2 = v.to_vec();
                v2.swap(0, i);
                let curr = v2.get(0).unwrap().clone();
                v2.remove(0);
                for mut p in get_permutations(v2.to_vec()) {
                    p.insert(0, curr.clone());
                    permutations.push(p);
                }
            }
            permutations
        },
    }
    
}


fn build_map(input: &String) -> (HashMap<Route, u32>, Vec<&str>) {
    let mut routes: HashMap<Route, u32> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(" to ").collect();
        let origin = split[0];
        let split: Vec<&str> = split[1].split(" = ").collect();
        let dest = split[0];
        let dist = split[1].parse::<u32>().unwrap();
        //println!("{}: O={}, D={}, d={}", line, origin, dest, dist);
        routes.insert(Route{origin: origin, dest: dest}, dist);
        routes.insert(Route{origin: dest, dest: origin}, dist);
        cities.insert(origin);
        cities.insert(dest);
    }
    let mut cs = vec![];
    for city in cities {
        cs.push(city);
    }
    (routes, cs)
}
