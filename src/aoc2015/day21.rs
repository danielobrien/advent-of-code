
use std::cmp;

#[derive(Debug)]
struct Entity {
    armour: isize,
    damage: isize,
    hp: isize,
}

impl Entity {
    fn from_str(data: &String) -> Entity {
        let mut hp = 0;
        let mut dmg = 0;
        let mut armour = 0;
        for line in data.lines() {
            let value = line.split(" ").last().expect("input line was unexpectedly empty").parse::<isize>().expect("Value for entity characteristic is not a valid isize");
            if line.contains("Hit Points") { hp = value; }
            if line.contains("Damage") { dmg = value; }
            if line.contains("Armor") { armour = value; }
        }
        Entity { armour: armour, damage: dmg, hp: hp }
    }
}

pub fn solve(input: &String) -> Vec<Result<isize, String>> {
    let boss = Entity::from_str(input);
    let mut min_gold_to_win = isize::max_value();
    let mut max_gold_to_lose = isize::min_value();
    let mut players = get_possible_players();
    players.sort_by(|a, b| a.1.cmp(&b.1));
    for (player, gold) in players {
        let victory = play(&player, &boss);
        if victory && gold < min_gold_to_win { min_gold_to_win = gold; }
        else if !victory && gold > max_gold_to_lose { max_gold_to_lose = gold; }
    }
    vec![Ok(min_gold_to_win), Ok(max_gold_to_lose)]
}

fn get_possible_players() -> Vec<(Entity, isize)> {
    let mut players = vec![];
    let weapons = vec![(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
    let armours = vec![(13, 1), (31, 2), (53, 3), (75, 4), (102, 5), (0, 0)];
    let rings = get_rings();
    for (wpn_cost, damage) in weapons {
        for (arm_cost, armour) in armours.iter().cloned() {
            for (ring_cost, ring_dmg, ring_arm) in rings.iter().cloned() {
                let player = Entity {armour: armour + ring_arm, damage: damage + ring_dmg, hp: 100};
                players.push((player, wpn_cost + arm_cost + ring_cost));
            }
        }
    }
    players
}

fn get_rings() -> Vec<(isize, isize, isize)> {
    let rings = vec![(25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3)];
    let mut combos = vec![(0,0,0)];
    for (i, ring) in rings.clone().iter().enumerate() {
        combos.push(*ring);
        for j in i..rings.len() {
            let r1 = rings[j];
            let cr = (r1.0 + ring.0, r1.1 + ring.1, r1.2 + ring.2);
            combos.push(cr);
        }
    }
    combos
}

// Returns true if the player wins, otherwise false.
fn play(player: &Entity, boss: &Entity) -> bool {
    let mut b_hp = boss.hp;
    let mut p_hp = player.hp;
    loop {
        b_hp -= cmp::max(player.damage- boss.armour, 1);
        if b_hp <= 0 { return true; }
        p_hp -= cmp::max(boss.damage - player.armour, 1);
        if p_hp <= 0 { return false; }
    }
}
