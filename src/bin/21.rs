use cached::proc_macro::cached;
use itertools::Itertools;

advent_of_code::solution!(21);

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMORS: [Item; 6] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 8] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stats {
    pub hp: u8,
    pub armor: u8,
    pub damage: u8,
}

#[cached]
pub fn calculate_damage(attacker: Stats, defender: Stats) -> u8 {
    let mut damage = attacker.damage.saturating_sub(defender.armor);
    if damage == 0 {
        damage = 1;
    }

    damage
}

const BOSS: Stats = Stats {
    hp: 103,
    damage: 9,
    armor: 2,
};

fn player_wins(player: Stats) -> bool {
    simulate_battle(player, BOSS)
}

fn simulate_battle(mut player: Stats, mut boss: Stats) -> bool {
    let player_damage = (player.damage.saturating_sub(boss.armor)).max(1);
    let boss_damage = (boss.damage.saturating_sub(player.armor)).max(1);

    loop {
        // Player attacks
        boss.hp = boss.hp.saturating_sub(player_damage);
        if boss.hp == 0 {
            return true;
        }

        // Boss attacks
        player.hp = player.hp.saturating_sub(boss_damage);
        if player.hp == 0 {
            return false;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: u16,
    damage: u8,
    armor: u8,
}

pub fn part_one(_input: &str) -> Option<u16> {
    let mut min_cost = u16::MAX;

    for weapon in &WEAPONS {
        for armor in &ARMORS {
            for ring_result in RINGS.iter().combinations(2) {
                let ring1 = ring_result[0];
                let ring2 = ring_result[1];
                let total_cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                let total_damage = weapon.damage + armor.damage + ring1.damage + ring2.damage;
                let total_armor = weapon.armor + armor.armor + ring1.armor + ring2.armor;

                if total_cost >= min_cost {
                    // skip if the cost is already higher than the minimum found
                    continue;
                }

                // Create player stats
                let player = Stats {
                    hp: 100,
                    damage: total_damage,
                    armor: total_armor,
                };

                // Simulate the battle
                if player_wins(player) {
                    min_cost = total_cost;
                }
            }
        }
    }

    Some(min_cost)
}

pub fn part_two(_input: &str) -> Option<u16> {
    let mut max_cost: u16 = 0;

    for weapon in &WEAPONS {
        for armor in &ARMORS {
            for ring_result in RINGS.iter().combinations(2) {
                let ring1 = ring_result[0];
                let ring2 = ring_result[1];
                let total_cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                let total_damage = weapon.damage + armor.damage + ring1.damage + ring2.damage;
                let total_armor = weapon.armor + armor.armor + ring1.armor + ring2.armor;

                if total_cost <= max_cost {
                    // skip if the cost is already lower than the maximum found
                    continue;
                }

                // Create player stats
                let player = Stats {
                    hp: 100,
                    damage: total_damage,
                    armor: total_armor,
                };

                // Simulate the battle
                if !player_wins(player) {
                    max_cost = total_cost;
                }
            }
        }
    }

    Some(max_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_simulation() {
        let player = Stats {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Stats {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        assert!(simulate_battle(player, boss));
    }

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
