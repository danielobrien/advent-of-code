use std::cmp;

fn area_wrap_needed(l: usize, w: usize, h:usize) -> usize {
    let lw = l*w;
    let lh = l*h;
    let wh = w*h;
    let sa = 2*lw + 2*lh + 2*wh;
    let min_side = cmp::min(cmp::min(lw, lh), wh);
    sa + min_side
}

fn length_ribbon_needed(l: usize, w: usize, h:usize) -> usize {
    let longest = cmp::max(cmp::max(l, w), h);
    let perimeter_smallest_face = 2 * ( l + w + h - longest);
    let volume = l * w * h;
    perimeter_smallest_face + volume
}

fn main() {
    let input = "4x23x21\n22x29x19"; //TODO: Add proper input handling
    let mut total_wrap_needed = 0;
    let mut total_ribbon_needed = 0;
    for line in input.lines() {
        let v: Vec<usize> = line.split('x').map(|s| s.parse::<usize>().expect("Not an int")).collect();
        total_wrap_needed += area_wrap_needed(v[0], v[1], v[2]);
        total_ribbon_needed += length_ribbon_needed(v[0], v[1], v[2]);
    }
    println!("The total area of wrapping paper required is: {}", total_wrap_needed);
    println!("The total length of ribbon required is: {}", total_ribbon_needed);
}
