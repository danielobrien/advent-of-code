/* --- Day 17: No Such Thing as Too Much ---

    The elves bought too much eggnog again - 150 liters this time. To fit it all into your refrigerator, you'll need to move it into smaller containers. You take an inventory of the capacities of the available containers.

    For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If you need to store 25 liters, there are four ways to do it:

    15 and 10
    20 and 5 (the first 5)
    20 and 5 (the second 5)
    15, 5, and 5
    Filling all containers entirely, how many different combinations of containers can exactly fit all 150 liters of eggnog?

    Your puzzle answer was 1304.

    The first half of this puzzle is complete! It provides one gold star: *

    --- Part Two ---

    While playing with all the containers in the kitchen, another load of eggnog arrives! The shipping and receiving department is requesting as many containers as you can spare.

    Find the minimum number of containers that can exactly fit all 150 liters of eggnog. How many different ways can you fill that number of containers and still hold exactly 150 litres?

    In the example above, the minimum number of containers was two. There were three ways to use that many containers, and so the answer there would be 3.
*/

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut containers: Vec<usize> = vec![];
    for line in input.lines() {
        containers.push(line.trim().parse::<usize>().unwrap())
    }
    let (p1, p2) = alt_solve(containers);
    vec![Ok(p1), Ok(p2)]
}

fn alt_solve(v: Vec<usize>) -> (usize, usize) {
    let desired_capacity = 150;
    let mut count = 0;
    let mut min_containers = usize::max_value();
    let mut count_min_containers = 0;
    let len = v.len();
    for i in 1..(1 << len) {
        let mut t = i;
        let mut total_capacity = 0;
        let mut num_containers = 0;
        for capacity in v.clone() {
            if t % 2 == 1 { 
                total_capacity += capacity; 
                num_containers += 1;
            }
            t /= 2;
        }
        if total_capacity == desired_capacity {
            count +=1;
            if num_containers < min_containers {
                min_containers = num_containers;
                count_min_containers = 1;
            } else if num_containers == min_containers {
                count_min_containers += 1;
            }
        }
    }
    (count, count_min_containers)
}