fn solve(input: &String) -> Vec<Result<usize, String>> {
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
