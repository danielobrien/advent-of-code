/* --- Day 4: The Ideal Stocking Stuffer ---

    Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

    To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

    For example:

    If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

    --- Part Two ---

    Now find one that starts with six zeroes.
*/

extern crate md5;

pub fn solve(input: &String) -> Vec<Result<usize, String>> {   
    let root = input.trim();
    let mut i = 0;
    let mut p1 = 0;
    loop {
        i += 1;
        let s = format!("{}{}", root, i);
        let d = md5::compute(s.as_bytes());
        let valid_p1 = d[0] == 0x0  && d[1] == 0x0 && d[2] & 0xF0 == 0x0;
        if valid_p1 { 
            if p1 == 0 { p1 = i; }
            if d[2] == 0x0 { break; }
        }
    }
    vec!(Ok(p1), Ok(i))
}