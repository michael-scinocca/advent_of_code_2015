use rand::{rngs::ThreadRng, seq::IndexedRandom};

#[derive(Clone)]
struct Character {
    name: String,
    hp: u32,
    mana: u32,
    mana_spent: u32,
    atk: u32,
    ac: u32,
    effects: Vec<Effect>,
    spells: Vec<Spell>,
    ac_buff: u32,
}

impl Character {
    fn new_wizard(name: String, hp: u32, mana: u32, spells: Vec<Spell>) -> Character {
        Character {
            name,
            hp,
            mana,
            mana_spent: 0,
            atk: 0,
            ac: 0,
            effects: Vec::new(),
            spells,
            ac_buff: 0,
        }
    }

    fn new_boss(name: String, hp: u32, atk: u32) -> Character {
        Character {
            name,
            hp,
            mana: 0,
            mana_spent: 0,
            atk,
            ac: 0,
            effects: Vec::new(),
            spells: Vec::new(),
            ac_buff: 0,
        }
    }

    fn resolve_effects(&mut self, trace: bool) {
        self.ac_buff = 0;

        let mut effects = std::mem::take(&mut self.effects);

        for effect in effects.iter_mut() {
            effect.turns -= 1;

            match effect.kind {
                EffectKind::Shield { ac_buff } => {
                    self.ac_buff = ac_buff;
                }
                EffectKind::Poison { damage } => {
                    self.take_damage(damage);

                    if trace {
                        println!(
                            "{} takes {} damage from poison - down to {} hit points.",
                            self.name, damage, self.hp
                        );
                    }
                }
                EffectKind::Recharge { mana_regen } => {
                    self.mana += mana_regen;
                }
            }
        }

        effects.retain(|e| e.turns > 0);

        self.effects = effects;
    }

    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }

    fn spend_mana(&mut self, amount: u32) {
        self.mana = self.mana.saturating_sub(amount);
        self.mana_spent += amount;
    }

    fn ac(&self) -> u32 {
        self.ac + self.ac_buff
    }
}

#[derive(Clone)]
struct Spell {
    cost: u32,
    kind: SpellKind,
}

#[derive(Clone)]
enum SpellKind {
    MagicMissile { damage: u32 },
    Drain { damage: u32, regen: u32 },
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn magic_missile() -> Spell {
        Spell {
            cost: 53,
            kind: SpellKind::MagicMissile { damage: 4 },
        }
    }

    fn drain() -> Spell {
        Spell {
            cost: 73,
            kind: SpellKind::Drain {
                damage: 2,
                regen: 2,
            },
        }
    }

    fn shield() -> Spell {
        Spell {
            cost: 113,
            kind: SpellKind::Shield,
        }
    }

    fn poison() -> Spell {
        Spell {
            cost: 173,
            kind: SpellKind::Poison,
        }
    }

    fn recharge() -> Spell {
        Spell {
            cost: 229,
            kind: SpellKind::Recharge,
        }
    }

    fn cast(&self, caster: &mut Character, defender: &mut Character, trace: bool) -> bool {
        if caster.mana < self.cost {
            return false;
        }

        match self.kind {
            SpellKind::MagicMissile { damage } => {
                caster.spend_mana(self.cost);

                defender.take_damage(damage);

                if trace {
                    println!(
                        "{} casts Magic Missile, dealing {} damage to {} - down to {} hit points.",
                        caster.name, damage, defender.name, defender.hp
                    );
                }
            }
            SpellKind::Drain { damage, regen } => {
                caster.spend_mana(self.cost);

                defender.take_damage(damage);
                caster.hp += regen;

                if trace {
                    println!(
                        "{} casts Drain, dealing {} damage to {} - down to {} hit points.",
                        caster.name, 2, defender.name, defender.hp
                    );
                }
            }
            SpellKind::Shield => {
                if caster
                    .effects
                    .iter()
                    .find(|x| matches!(x.kind, EffectKind::Shield { ac_buff: _ }))
                    .is_some()
                {
                    return false;
                }

                caster.spend_mana(self.cost);

                caster.effects.push(Effect {
                    turns: 6,
                    kind: EffectKind::Shield { ac_buff: 7 },
                });

                if trace {
                    println!("{} casts Shield.", caster.name);
                }
            }
            SpellKind::Poison => {
                if defender
                    .effects
                    .iter()
                    .find(|x| matches!(x.kind, EffectKind::Poison { damage: _ }))
                    .is_some()
                {
                    return false;
                }

                caster.spend_mana(self.cost);

                defender.effects.push(Effect {
                    turns: 6,
                    kind: EffectKind::Poison { damage: 3 },
                });

                if trace {
                    println!("{} casts Poison.", caster.name);
                }
            }
            SpellKind::Recharge => {
                if caster
                    .effects
                    .iter()
                    .find(|x| matches!(x.kind, EffectKind::Recharge { mana_regen: _ }))
                    .is_some()
                {
                    return false;
                }

                caster.spend_mana(self.cost);

                caster.effects.push(Effect {
                    turns: 5,
                    kind: EffectKind::Recharge { mana_regen: 101 },
                });

                if trace {
                    println!("{} casts Recharge.", caster.name);
                }
            }
        }

        true
    }
}

#[derive(Clone)]
struct Effect {
    turns: u32,
    kind: EffectKind,
}

#[derive(Clone)]
enum EffectKind {
    Shield { ac_buff: u32 },
    Poison { damage: u32 },
    Recharge { mana_regen: u32 },
}

impl Character {
    fn attack(&self, defender: &mut Character, trace: bool) {
        let damage = self.atk.saturating_sub(defender.ac()).max(1);

        defender.take_damage(damage);

        if trace {
            println!(
                "{} deals {} damage to {} - down to {} hit points.",
                self.name, damage, defender.name, defender.hp
            );
        }
    }

    fn cast_spell(&mut self, spell: &Spell, target: &mut Character, trace: bool) -> bool {
        spell.cast(self, target, trace)
    }
}

pub fn part1() {
    let mut min_mana_spent = u32::MAX;

    let mut rng = rand::rng();

    for _ in 0..1000000 {
        let player = Character::new_wizard(
            "Player".to_owned(),
            50,
            500,
            vec![
                Spell::magic_missile(),
                Spell::drain(),
                Spell::shield(),
                Spell::poison(),
                Spell::recharge(),
            ],
        );

        let boss = Character::new_boss("Boss".to_owned(), 58, 9);

        let (win, mana_spent) = battle(player, boss, false, &mut rng, false);

        if win && mana_spent < min_mana_spent {
            min_mana_spent = mana_spent;
        }
    }

    println!("{}", min_mana_spent);
}

pub fn part2() {
    let mut min_mana_spent = u32::MAX;

    let mut rng = rand::rng();

    for _ in 0..1000000 {
        let player = Character::new_wizard(
            "Player".to_owned(),
            50,
            500,
            vec![
                Spell::magic_missile(),
                Spell::drain(),
                Spell::shield(),
                Spell::poison(),
                Spell::recharge(),
            ],
        );

        let boss = Character::new_boss("Boss".to_owned(), 58, 9);

        let (win, mana_spent) = battle(player, boss, true, &mut rng, false);

        if win && mana_spent < min_mana_spent {
            min_mana_spent = mana_spent;
        }
    }

    println!("{}", min_mana_spent);
}

fn battle(
    mut player: Character,
    mut boss: Character,
    hard_mode: bool,
    mut rng: &mut ThreadRng,
    trace: bool,
) -> (bool, u32) {
    let mut win = false;

    while player.hp > 0 && boss.hp > 0 {
        if hard_mode {
            player.take_damage(1);
        }

        player.resolve_effects(trace);
        boss.resolve_effects(trace);

        let min_spell_cost = player.spells.iter().map(|s| s.cost).min().unwrap();

        if player.mana < min_spell_cost {
            player.hp = 0;
            continue;
        }

        let mut cast = false;

        while !cast {
            let spell = player.spells.choose(&mut rng).unwrap().clone();
            cast = player.cast_spell(&spell, &mut boss, trace);
        }

        player.resolve_effects(trace);
        boss.resolve_effects(trace);
        if boss.hp > 0 {
            boss.attack(&mut player, trace);
        } else {
            win = true;
        }
    }

    (win, player.mana_spent)
}
