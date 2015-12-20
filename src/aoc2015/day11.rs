// --- Day 11: Corporate Policy ---
//
// Santa's previous password expired, and he needs help choosing a new one.
//
// To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.
//
// Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.
//
// Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:
//
// Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
// Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
// Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
// For example:
//
// hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
// abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
// abbcegjk fails the third requirement, because it only has one double letter (bb).
// The next password after abcdefgh is abcdffaa.
// The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.
// Given Santa's current password (your puzzle input), what should his next password be?
//
// --- Part Two ---
//
// Santa's password expired again. What's the next one?
//
//

use std::cmp;

pub fn solve(input: &String) -> Vec<Result<String, String>> {
    // println!("{}", input);
    let p1 = get_next_pwd(input.clone());
    let p2 = get_next_pwd(p1.clone());

    vec![Ok(p1), Ok(p2)]
}

fn get_next_pwd(mut pwd: String) -> String {
    if is_valid(&pwd) {
        pwd = increment(pwd);
    }
    // let mut loops = 0;
    while !is_valid(&pwd) {
        // loops += 1;
        pwd = increment(pwd);
        // if loops > 2 {break;}
    }
    pwd
}

fn increment(pwd: String) -> String {
    let mut pw = pwd.clone();
    let is = pw.find("i").unwrap_or(9);
    let ls = pw.find("l").unwrap_or(9);
    let os = pw.find("o").unwrap_or(9);
    let i = cmp::min(is, cmp::min(ls, os));
    // println!("{}, {}, {}, {}", is, ls, os, i);
    if i < 9 {
        let c = if i == is {
            'j'
        } else if i == ls {
            'm'
        } else if i == os {
            'p'
        } else {
            panic!("unreachable");
        };
        // println!("no mods: {}", pw);
        pw.insert(i, c);
        pw.pop();
        // println!("insert: {}", pw);
    }
    for _ in i + 1..8 {
        pw.remove(i + 1);
        pw.push('a');
        // println!("removed@i, pushed a: {}", pw);
    }
    if i < 9 {
        return pw;
    }

    let mut bs: Vec<u8> = pw.bytes().collect();
    // println!("{:?}", bs);
    let mut i = 8;
    while i > 0 {
        i -= 1;
        if bs[i] == 0x68 || bs[i] == 0x6e || bs[i] == 0x6b {
            bs[i] += 2;
        } else {
            bs[i] = bs[i] + 1;
        }
        // if i<4{println!("bs[{}]={:x}", i, bs[i]);}
        if bs[i] > 0x7A {
            bs[i] = 0x61
        } else {
            break;
        }
    }
    let p = bs.iter().fold(String::new(), |mut acc, b| {
        acc.push(*b as char);
        acc
    });
    // println!("{}", p);
    p
}



fn is_valid(pwd: &String) -> bool {
    if pwd.contains(|c| c == 'i' || c == 'o' || c == 'l') {
        // println!("Contained an i, l, or o");
        false
    } else {
        let mut prev = 0u8;
        let mut prev_prev = 0u8;
        let mut prev_prev_prev = 0u8;
        let mut seq = false;
        let mut doubles = 0;
        for b in pwd.bytes() {
            // println!("p_p={}, p={}, c={}", prev_prev, prev, b);
            if b == prev + 1 && b == prev_prev + 2 {
                seq = true;
            }
            if b == prev && (prev != prev_prev || prev == prev_prev && prev == prev_prev_prev) {
                doubles += 1;
            }
            prev_prev_prev = prev_prev;
            prev_prev = prev;
            prev = b;
        }
        // println!("doubles={}, seq={}", doubles, seq);
        doubles >= 2 && seq
    }
}
