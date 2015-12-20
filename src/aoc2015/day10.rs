// --- Day 10: Elves Look, Elves Say ---
//
// Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence. For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).
//
// Look-and-say sequences are generated iteratively, using the previous value as input for the next step. For each step, take the previous value, and replace each run of digits (like 111) with the number of digits (3) followed by the digit itself (1).
//
// For example:
//
// 1 becomes 11 (1 copy of digit 1).
// 11 becomes 21 (2 copies of digit 1).
// 21 becomes 1211 (one 2 followed by one 1).
// 1211 becomes 111221 (one 1, one 2, and two 1s).
// 111221 becomes 312211 (three 1s, two 2s, and one 1).
// Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?
//
// --- Part Two ---
//
// Neat, right? You might also enjoy hearing John Conway talking about this sequence (that's Conway of Conway's Game of Life fame).
//
// Now, starting again with the digits in your puzzle input, apply this process 50 times. What is the length of the new result?
//
//
pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut out = input.clone();
    for _ in 0..40 {
        out = look_and_say(&out);
    }
    let part1 = Ok(out.len());
    for _ in 0..10 {
        out = look_and_say(&out);
    }
    let part2 = Ok(out.len());

    vec![part1, part2]
}

fn look_and_say(input: &String) -> String {
    let mut out = String::new();
    let mut count = 0;
    let mut prev = input.chars().next().unwrap();
    let len = input.len();
    for (i, c) in input.chars().enumerate() {
        if c == prev {
            count += 1;
        } else {
            out = out + &count.to_string();
            out.push(prev);
            prev = c;
            count = 1;
        }
        if i == len - 1 {
            out = out + &count.to_string();
            out.push(c);
        }
    }
    out
}
