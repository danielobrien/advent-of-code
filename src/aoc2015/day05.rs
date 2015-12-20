// --- Day 5: Doesn't He Have Intern-Elves For This? ---
//
// Santa needs help figuring out which strings in his text file are naughty or nice.
//
// A nice string is one with all of the following properties:
//
// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
// For example:
//
// ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
// aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
// jchzalrnumimnmhp is naughty because it has no double letter.
// haegwjzuvuyypxyu is naughty because it contains the string xy.
// dvszwmarrgswjxmb is naughty because it contains only one vowel.
// How many strings are nice?
//
// --- Part Two ---
//
// Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.
//
// Now, a nice string is one with all of the following properties:
//
// It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
// It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
// For example:
//
// qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
// xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
// uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
// ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.
// How many strings are nice under these new rules?
//
//

use std::fmt;
use std::fmt::Display;

pub struct Counts(usize, usize);

impl Display for Counts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {} are nice", self.0, self.1)
    }
}

pub fn solve(input: &String) -> Vec<Result<Counts, String>> {
    vec![solve_part_one(input), solve_part_two(input)]
}

fn solve_part_two(input: &String) -> Result<Counts, String> {
    let mut count = 0;
    let mut count_nice = 0;
    for line in input.lines() {
        count += 1;
        let mut pairs = vec![];
        let mut c_prev = '-';
        for c in line.chars() {
            if c_prev == '-' {
                c_prev = c;
                continue;
            }
            pairs.push(c_prev.to_string() + &(c.to_string()));
            c_prev = c;
        }
        let mut has_repeating_pair = false;
        for pair in pairs {
            let f = line.find(&pair).unwrap();
            let r = line.rfind(&pair).unwrap();
            if r > f + 1 {
                has_repeating_pair = true;
                // println!("In {}, found repeating pair {} at {} and {}", line, pair, r, f);
                break;
            }
        }
        let mut has_separated_double = false;
        let v: Vec<char> = line.chars().collect();
        for (i, c) in v.iter().enumerate() {
            if i + 2 >= v.len() {
                break;
            }
            if v.get(i + 2) == Some(c) {
                has_separated_double = true;
                break;
            }
        }
        // println!("{}: rptg_pr:{} sep_doub:{}", line, has_repeating_pair, has_separated_double);
        if has_repeating_pair && has_separated_double {
            count_nice += 1;
        }
        // println!("{:?}", pairs);
    }
    Ok(Counts(count_nice, count))
}

fn solve_part_one(input: &String) -> Result<Counts, String> {
    let mut count = 0;
    let mut count_nice = 0;
    for line in input.lines() {
        count += 1;
        let vowels = line.trim()
                         .chars()
                         .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
                         .count();
        let mut has_double = false;
        let mut c_prev = None;
        for c in line.chars() {
            // println!("Comparing {:?} to {}", c_prev, c);
            if Some(c) == c_prev {
                has_double = true;
                break;
            }
            c_prev = Some(c);
        }
        // println!("v={}, has_d={}", vowels, has_double);
        let is_nice = vowels >= 3 && has_double && !line.contains("ab") &&
                      !line.contains("cd") && !line.contains("pq") &&
                      !line.contains("xy");
        if is_nice {
            count_nice += 1;
        }
        // println!("{}: {}. Now at {} from {}", line, is_nice, count_nice, count);
    }
    Ok(Counts(count_nice, count))
}
