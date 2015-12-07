use std::fmt;
use std::fmt::Display;

struct Counts(usize, usize);

impl Display for Counts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {} are nice", self.0, self.1)
    }
}

fn solve(input: &String) -> Vec<Result<Counts, String>> {
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
            if c_prev == '-' { c_prev = c; continue; }
            pairs.push(c_prev.to_string() + &(c.to_string()));
            c_prev = c;
        }
        let mut has_repeating_pair = false;
        for pair in pairs {
            let f = line.find(&pair).unwrap();
            let r = line.rfind(&pair).unwrap();
            if r > f + 1 {
                has_repeating_pair = true;
                //println!("In {}, found repeating pair {} at {} and {}", line, pair, r, f);
                break;
            }
        }
        let mut has_separated_double = false;
        let v: Vec<char> = line.chars().collect();
        for (i, c) in v.iter().enumerate() {
            if i + 2 >= v.len() { break; }
            if v.get(i + 2) == Some(c) { has_separated_double = true; break; }
        }
        //println!("{}: rptg_pr:{} sep_doub:{}", line, has_repeating_pair, has_separated_double);
        if has_repeating_pair && has_separated_double { count_nice += 1; }
        //println!("{:?}", pairs);
    }
    Ok(Counts(count_nice, count))
}

fn solve_part_one(input: &String) -> Result<Counts, String> {
    let mut count = 0;
    let mut count_nice = 0;
    for line in input.lines() {
        count += 1;
        let vowels = line.trim().chars()
            .filter(|&c| c == 'a' || c == 'e' || c =='i' || c == 'o' || c == 'u')
            .count();
        let mut has_double = false;
        let mut c_prev = None;
        for c in line.chars() {
            //println!("Comparing {:?} to {}", c_prev, c);
            if Some(c) == c_prev {
                has_double = true;
                break;
            }
            c_prev = Some(c);
        }
        //println!("v={}, has_d={}", vowels, has_double);
        let is_nice =   vowels >=3 
                        && has_double
                        && !line.contains("ab") 
                        && !line.contains("cd")
                        && !line.contains("pq") 
                        && !line.contains("xy");
        if is_nice { count_nice += 1; }
        //println!("{}: {}. Now at {} from {}", line, is_nice, count_nice, count);
    }        
    Ok(Counts(count_nice, count))
}
