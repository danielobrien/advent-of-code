/* --- Day 14: Reindeer Olympics ---
    
    This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must rest occasionally to recover their energy. Santa would like to know which of his reindeer is fastest, and so he has them race.
    
    Reindeer can only either be flying (always at their top speed) or resting (not moving at all), and always spend whole seconds in either state.
    
    For example, suppose you have the following Reindeer:
    
    Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds, Comet has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins resting (staying at 140 km), and Dancer continues on for a total distance of 176 km. On the 12th second, both reindeer are resting. They continue to rest until the 138th second, when Comet flies for another ten seconds. On the 174th second, Dancer flies for another 11 seconds.
    
    In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead at 1120 km (poor Dancer has only gotten 1056 km by that point). So, in this situation, Comet would win (if the race ended at 1000 seconds).
    
    Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, what distance has the winning reindeer traveled?
    
    --- Part Two ---
    
    Seeing how reindeer move in bursts, Santa decides he's not pleased with the old scoring system.
    
    Instead, at the end of each second, he awards one point to the reindeer currently in the lead. (If there are multiple reindeer tied for the lead, they each get one point.) He keeps the traditional 2503 second time limit, of course, as doing otherwise would be entirely ridiculous.
    
    Given the example reindeer from above, after the first second, Dancer is in the lead and gets one point. He stays in the lead until several seconds into Comet's second burst: after the 140th second, Comet pulls into the lead and gets his first point. Of course, since Dancer had been in the lead for the 139 seconds before that, he has accumulated 139 points by the 140th second.
    
    After the 1000th second, Dancer has accumulated 689 points, while poor Comet, our old champion, only has 312. So, with the new scoring system, Dancer would win (if the race ended at 1000 seconds).
    
    Again given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, how many points does the winning reindeer have?
*/
struct Reindeer {
    speed: usize,
    endurance: usize,
    recovery: usize,
    resting: bool,
    distance: usize,
    section_time: usize,
    points: usize,
}

impl Reindeer {
    fn new(speed: usize, endurance: usize, recovery: usize) -> Reindeer {
        Reindeer{ speed: speed, endurance: endurance, recovery: recovery,
            resting: false, distance: 0, section_time: 0, points: 0}
    }
    
    fn increment_time(&mut self) {
        self.section_time += 1;
        if self.resting {
            if self.section_time == self.recovery {
                self.section_time = 0;
                self.resting = false;
            }
        } else {
            self.distance += self.speed;
            if self.section_time == self.endurance {
                self.section_time = 0;
                self.resting = true;
            }
        }
    }
    
    fn get_points(&mut self, winning_dist: usize) {
        if winning_dist == self.distance {
            self.points += 1;
        }
    }
}

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let mut v = vec![];
    for line in input.lines() {
        let arr: Vec<&str> = line.split_whitespace().collect();
        let s = arr[3].parse::<usize>().unwrap();
        let e = arr[6].parse::<usize>().unwrap();
        let r = arr[13].parse::<usize>().unwrap();
        v.push(Reindeer::new(s, e, r));
    }
    
    for _ in 0..2503 {
        for reindeer in v.iter_mut() {
            reindeer.increment_time();
        }
        let winning_dist = v.iter().map(|r| r.distance).max().unwrap();
        for reindeer in v.iter_mut() {
            reindeer.get_points(winning_dist);
        }
    }

    let winning_dist = v.iter().map(|r| r.distance).max().unwrap();
    let winning_pts = v.iter().map(|r| r.points).max().unwrap();
    
    vec![Ok(winning_dist), Ok(winning_pts)]
}
