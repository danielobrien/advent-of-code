// --- Day 15: Science for Hungry People ---
//
// Today, you set out on the task of perfecting your milk-dunking cookie recipe. All you have to do is find the right balance of ingredients.
//
// Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a list of the remaining ingredients you could use to finish the recipe (your puzzle input) and their properties per teaspoon:
//
// capacity (how well it helps the cookie absorb milk)
// durability (how well it keeps the cookie intact when full of milk)
// flavor (how tasty it makes the cookie)
// texture (how it improves the feel of the cookie)
// calories (how many calories it adds to the cookie)
// You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be accurate so you can reproduce your results in the future. The total score of a cookie can be found by adding up each of the properties (negative totals become 0) and then multiplying together everything except calories.
//
// For instance, suppose you have these two ingredients:
//
// Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
// Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
// Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon (because the amounts of each ingredient must add up to 100) would result in a cookie with the following properties:
//
// A capacity of 44*-1 + 56*2 = 68
// A durability of 44*-2 + 56*3 = 80
// A flavor of 44*6 + 56*-2 = 152
// A texture of 44*3 + 56*-1 = 76
// Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now) results in a total score of 62842880, which happens to be the best score possible given these ingredients. If any properties had produced a negative total, it would have instead become zero, causing the whole score to multiply to zero.
//
// Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// Your cookie recipe becomes wildly popular! Someone asks if you can make another recipe that has exactly 500 calories per cookie (so they can use it as a meal replacement). Keep the rest of your award-winning process the same (100 teaspoons, same ingredients, same scoring system).
//
// For example, given the ingredients above, if you had instead selected 40 teaspoons of butterscotch and 60 teaspoons of cinnamon (which still adds to 100), the total calorie count would be 40*8 + 60*3 = 500. The total score would go down, though: only 57600000, the best you can do in such trying circumstances.
//
// Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make with a calorie total of 500?
//
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;
use std::fmt::Debug;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Ingredient<'a> {
    name: &'a str,
    capacity: isize,
    durability: isize,
    flavour: isize,
    texture: isize,
    calories: usize,
}

impl<'a> Debug for Ingredient<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut ingredients = vec![];
    for line in input.lines() {
        let arr: Vec<&str> = line.split_whitespace().collect();
        let c = parse(arr[2]);
        let d = parse(arr[4]);
        let f = parse(arr[6]);
        let t = parse(arr[8]);
        let cal = arr[10].parse::<usize>().expect("Couldn't parse");
        ingredients.push(Ingredient {
            name: arr[0],
            capacity: c,
            durability: d,
            flavour: f,
            texture: t,
            calories: cal,
        });
    }
    let mut best_score = 0;
    let mut best_score_calorie_restricted = 0;
    for c in combinations(ingredients, 100) {
        let mut combined_ingredient = Ingredient {
            name: "combined",
            capacity: 0,
            durability: 0,
            flavour: 0,
            texture: 0,
            calories: 0,
        };
        for (ing, n) in c {
            combined_ingredient.capacity += ing.capacity * n as isize;
            combined_ingredient.durability += ing.durability * n as isize;
            combined_ingredient.flavour += ing.flavour * n as isize;
            combined_ingredient.texture += ing.texture * n as isize;
            combined_ingredient.calories += ing.calories * n;
        }
        let score = if combined_ingredient.capacity <= 0 || combined_ingredient.durability <= 0 ||
                       combined_ingredient.flavour <= 0 ||
                       combined_ingredient.texture <= 0 {
            0
        } else {
            combined_ingredient.capacity * combined_ingredient.durability *
            combined_ingredient.flavour * combined_ingredient.texture
        } as usize;
        if score > best_score {
            best_score = score;
        }
        if combined_ingredient.calories == 500 && score > best_score_calorie_restricted {
            best_score_calorie_restricted = score;
        }
    }



    vec![Ok(best_score), Ok(best_score_calorie_restricted)]
}

fn combinations<T: Debug + Copy + Eq + Hash>(items: Vec<T>, n: usize) -> Vec<HashMap<T, usize>> {
    // println!("getting combos for {:?}", items);
    if items.is_empty() {
        vec![]
    } else if items.len() == 1 {
        let mut map = HashMap::new();
        map.insert(items[0], n);
        vec![map]
    } else if items.len() == 2 {
        let mut v = vec![];
        for i in 0..n {
            let mut map = HashMap::new();
            map.insert(items[0], i);
            map.insert(items[1], n - i);
            v.push(map);
        }
        v
    } else {
        let mut v = items.clone();
        let first = v.pop().unwrap();
        let mut combos = vec![];
        for i in 0..n {
            for map in combinations(v.clone(), n - i).iter_mut() {
                map.insert(first, i);
                combos.push(map.clone());
            }
        }
        // println!("combos={:?}", combos);
        // println!("combos.len={}", combos.len());
        combos
    }
}

fn parse(string: &str) -> isize {
    let mut s = string.to_string();
    s.pop();
    s.parse::<isize>().expect("Couldn't parse")
}
