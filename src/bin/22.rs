use pathfinding::prelude::dijkstra;

advent_of_code::solution!(22);

const PLAYER_HP: u16 = 50;
const PLAYER_MANA: u32 = 500;
const SHIELD_ARMOR: u16 = 7;
const POISON_DAMAGE: u16 = 3;
const RECHARGE_MANA: u32 = 101;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const ALL: [Self; 5] = [
        Self::MagicMissile,
        Self::Drain,
        Self::Shield,
        Self::Poison,
        Self::Recharge,
    ];

    const fn cost(self) -> u32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Difficulty {
    Normal,
    Hard,
}

/// The fight as it stands at the start of a player turn, before effects apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    player_hp: u16,
    mana: u32,
    boss_hp: u16,
    shield: u8,
    poison: u8,
    recharge: u8,
}

impl State {
    const fn new(player_hp: u16, mana: u32, boss_hp: u16) -> Self {
        Self {
            player_hp,
            mana,
            boss_hp,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }

    const fn boss_dead(&self) -> bool {
        self.boss_hp == 0
    }

    /// Applies start-of-turn effects and ticks their timers. Returns the
    /// player's armor for this turn.
    fn apply_effects(&mut self) -> u16 {
        let mut armor = 0;
        if self.shield > 0 {
            armor = SHIELD_ARMOR;
            self.shield -= 1;
        }
        if self.poison > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(POISON_DAMAGE);
            self.poison -= 1;
        }
        if self.recharge > 0 {
            self.mana += RECHARGE_MANA;
            self.recharge -= 1;
        }
        armor
    }

    const fn can_cast(&self, spell: Spell) -> bool {
        self.mana >= spell.cost()
            && match spell {
                Spell::MagicMissile | Spell::Drain => true,
                Spell::Shield => self.shield == 0,
                Spell::Poison => self.poison == 0,
                Spell::Recharge => self.recharge == 0,
            }
    }

    fn cast(&mut self, spell: Spell) {
        self.mana -= spell.cost();
        match spell {
            Spell::MagicMissile => self.boss_hp = self.boss_hp.saturating_sub(4),
            Spell::Drain => {
                self.boss_hp = self.boss_hp.saturating_sub(2);
                self.player_hp += 2;
            }
            Spell::Shield => self.shield = 6,
            Spell::Poison => self.poison = 6,
            Spell::Recharge => self.recharge = 5,
        }
    }

    /// Plays the boss turn. Returns `false` if the player dies.
    fn boss_turn(&mut self, boss_damage: u16) -> bool {
        let armor = self.apply_effects();
        if self.boss_dead() {
            return true;
        }
        let damage = boss_damage.saturating_sub(armor).max(1);
        self.player_hp = self.player_hp.saturating_sub(damage);
        self.player_hp > 0
    }

    /// Every surviving state reachable by one full round (player turn + boss
    /// turn), paired with the mana spent to get there.
    fn successors(mut self, boss_damage: u16, difficulty: Difficulty) -> Vec<(Self, u32)> {
        if difficulty == Difficulty::Hard {
            self.player_hp -= 1;
            if self.player_hp == 0 {
                return Vec::new();
            }
        }

        self.apply_effects();
        if self.boss_dead() {
            return vec![(self, 0)];
        }

        Spell::ALL
            .into_iter()
            .filter(|&spell| self.can_cast(spell))
            .filter_map(|spell| {
                let mut next = self;
                next.cast(spell);
                (next.boss_dead() || next.boss_turn(boss_damage)).then_some((next, spell.cost()))
            })
            .collect()
    }
}

fn least_mana_to_win(start: State, boss_damage: u16, difficulty: Difficulty) -> Option<u32> {
    dijkstra(
        &start,
        |state| state.successors(boss_damage, difficulty),
        State::boss_dead,
    )
    .map(|(_, cost)| cost)
}

/// Parses the boss's `(hit points, damage)` from the puzzle input.
fn parse_boss(input: &str) -> Option<(u16, u16)> {
    let stat = |name: &str| {
        input.lines().find_map(|line| {
            line.strip_prefix(name)?
                .strip_prefix(':')?
                .trim()
                .parse()
                .ok()
        })
    };
    Some((stat("Hit Points")?, stat("Damage")?))
}

fn solve(input: &str, difficulty: Difficulty) -> Option<u32> {
    let (boss_hp, boss_damage) = parse_boss(input)?;
    let start = State::new(PLAYER_HP, PLAYER_MANA, boss_hp);
    least_mana_to_win(start, boss_damage, difficulty)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, Difficulty::Normal)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, Difficulty::Hard)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Replays a fixed spell order, returning the mana spent if it wins.
    fn replay(mut state: State, boss_damage: u16, spells: &[Spell]) -> Option<u32> {
        let mut spent = 0;
        for &spell in spells {
            state.apply_effects();
            if state.boss_dead() {
                break;
            }
            assert!(state.can_cast(spell), "cannot cast {spell:?} in {state:?}");
            state.cast(spell);
            spent += spell.cost();
            if state.boss_dead() {
                break;
            }
            if !state.boss_turn(boss_damage) {
                return None;
            }
        }
        state.boss_dead().then_some(spent)
    }

    #[test]
    fn test_parse_boss() {
        assert_eq!(parse_boss("Hit Points: 55\nDamage: 8\n"), Some((55, 8)));
        assert_eq!(parse_boss(""), None);
    }

    #[test]
    fn test_example_one_replay() {
        let spells = [Spell::Poison, Spell::MagicMissile];
        assert_eq!(replay(State::new(10, 250, 13), 8, &spells), Some(226));
    }

    #[test]
    fn test_example_two_replay() {
        let spells = [
            Spell::Recharge,
            Spell::Shield,
            Spell::Drain,
            Spell::Poison,
            Spell::MagicMissile,
        ];
        assert_eq!(replay(State::new(10, 250, 14), 8, &spells), Some(641));
    }

    #[test]
    fn test_example_one_least_mana() {
        let result = least_mana_to_win(State::new(10, 250, 13), 8, Difficulty::Normal);
        assert_eq!(result, Some(226));
    }

    #[test]
    fn test_example_two_least_mana() {
        let result = least_mana_to_win(State::new(10, 250, 14), 8, Difficulty::Normal);
        assert_eq!(result, Some(641));
    }

    #[test]
    fn test_hard_mode_drains_player() {
        // 1 HP player dies at the start of their first hard-mode turn.
        let result = least_mana_to_win(State::new(1, 250, 4), 8, Difficulty::Hard);
        assert_eq!(result, None);
        let result = least_mana_to_win(State::new(1, 250, 4), 8, Difficulty::Normal);
        assert_eq!(result, Some(53));
    }
}
