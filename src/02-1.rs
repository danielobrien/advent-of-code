use std::cmp;
//TODO: Compare performance to a version that doesn't use a Gift struct
struct Gift {
    length: usize,
    width: usize,
    height: usize,
}

impl Gift {
    fn surface_area(&self) -> usize {
        2 * self.length * self.width + 2 * self.length * self.height
        + 2 * self.width * self.height
    }
    
    fn area_smallest_side(&self) -> usize {
        let lw = self.length * self.width;
        let lh = self.length * self.height;
        let wh = self.width * self.height;
        cmp::min(cmp::min(lw, lh), wh)
    }
    
    fn area_wrap_needed(&self) -> usize {
        self.surface_area() + self.area_smallest_side()
    }
}

fn main() {
    let input = "20x29x30\n23x11x5"; //TODO: Allow for file-based input
    let mut total_wrap_needed = 0;
    for line in input.lines() {
        let v: Vec<usize> = line.split('x').map(|s| s.parse::<usize>().expect("Not an int")).collect();
        let g = Gift {length: v[0], width: v[1], height: v[2]};
        total_wrap_needed += g.area_wrap_needed();
    }
    println!("The total area of wrapping paper required is: {}", total_wrap_needed);
}
