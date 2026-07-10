#[derive(Clone)]
struct Character {
    name: String,
    hp: u32,
    atk: u32,
    ac: u32,
}

enum Inventory {
    Weapon { cost: u32, atk: u32 },
    Armor { cost: u32, ac: u32 },
    Ring { cost: u32, atk: u32, ac: u32 },
}

impl Character {
    fn attack(&self, defender: &mut Character, trace: bool) {
        let damage = self.atk.saturating_sub(defender.ac).max(1);

        defender.hp = defender.hp.saturating_sub(damage);

        if trace {
            println!(
                "{} deals {} damage to {} - down to {} hit points.",
                self.name, damage, defender.name, defender.hp
            );
        }
    }
}

pub fn part1() {
    let boss = Character {
        name: "Boss".to_owned(),
        hp: 109,
        atk: 8,
        ac: 2,
    };

    let mut winning_combos = Vec::new();

    for atk in 4..=11 {
        for ac in 0..8 {
            let player = Character {
                name: "Player".to_owned(),
                hp: 100,
                atk,
                ac,
            };

            if battle(player.clone(), boss.clone(), false) {
                winning_combos.push((atk, ac));
            }
        }
    }

    let inventory = get_inventory();

    let mut lowest_cost = u32::MAX;

    for winning_combo in winning_combos {
        let cost = get_lowest_cost(&inventory, winning_combo.0, winning_combo.1);

        if cost < lowest_cost {
            lowest_cost = cost;
        }
    }

    println!("{}", lowest_cost);
}

pub fn part2() {
    let boss = Character {
        name: "Boss".to_owned(),
        hp: 109,
        atk: 8,
        ac: 2,
    };

    let mut losing_combos = Vec::new();

    for atk in 4..=11 {
        for ac in 0..8 {
            let player = Character {
                name: "Player".to_owned(),
                hp: 100,
                atk,
                ac,
            };

            if !battle(player.clone(), boss.clone(), false) {
                losing_combos.push((atk, ac));
            }
        }
    }

    let inventory = get_inventory();

    let mut highest_cost = 0;

    for losing_combo in losing_combos {
        let cost = get_highest_cost(&inventory, losing_combo.0, losing_combo.1);

        if cost > highest_cost {
            highest_cost = cost;
        }
    }

    println!("{}", highest_cost);
}

fn get_inventory() -> Vec<Inventory> {
    vec![
        Inventory::Weapon { cost: 8, atk: 4 },
        Inventory::Weapon { cost: 10, atk: 5 },
        Inventory::Weapon { cost: 25, atk: 6 },
        Inventory::Weapon { cost: 40, atk: 7 },
        Inventory::Weapon { cost: 74, atk: 8 },
        Inventory::Armor { cost: 13, ac: 1 },
        Inventory::Armor { cost: 31, ac: 2 },
        Inventory::Armor { cost: 53, ac: 3 },
        Inventory::Armor { cost: 75, ac: 4 },
        Inventory::Armor { cost: 102, ac: 5 },
        Inventory::Ring {
            cost: 25,
            atk: 1,
            ac: 0,
        },
        Inventory::Ring {
            cost: 50,
            atk: 2,
            ac: 0,
        },
        Inventory::Ring {
            cost: 100,
            atk: 3,
            ac: 0,
        },
        Inventory::Ring {
            cost: 20,
            atk: 0,
            ac: 1,
        },
        Inventory::Ring {
            cost: 40,
            atk: 0,
            ac: 2,
        },
        Inventory::Ring {
            cost: 80,
            atk: 0,
            ac: 3,
        },
    ]
}

fn get_lowest_cost(inventory: &Vec<Inventory>, needed_atk: u32, needed_ac: u32) -> u32 {
    let mut lowest_cost_atk = u32::MAX;
    let mut lowest_cost_ac = u32::MAX;

    for item in inventory {
        match item {
            Inventory::Weapon { cost, atk } => {
                let mut total_atk_cost = 0;
                let mut total_atk = 0;

                if *atk > needed_atk {
                    continue;
                }

                total_atk += atk;
                total_atk_cost += cost;

                if total_atk == needed_atk {
                    if total_atk_cost < lowest_cost_atk {
                        lowest_cost_atk = total_atk_cost;
                    }

                    continue;
                }

                for ring in inventory {
                    match ring {
                        Inventory::Ring { cost, atk, ac: _ } => {
                            if *atk == 0 {
                                continue;
                            }

                            if *atk + total_atk == needed_atk {
                                total_atk_cost += cost;

                                if total_atk_cost < lowest_cost_atk {
                                    lowest_cost_atk = total_atk_cost;
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Inventory::Armor { cost, ac } => {
                let mut total_ac_cost = 0;
                let mut total_ac = 0;

                if *ac > needed_ac {
                    continue;
                }

                total_ac += ac;
                total_ac_cost += cost;

                if total_ac == needed_ac {
                    if total_ac_cost < lowest_cost_ac {
                        lowest_cost_ac = total_ac_cost;
                    }

                    continue;
                }

                for ring in inventory {
                    match ring {
                        Inventory::Ring { cost, atk: _, ac } => {
                            if *ac == 0 {
                                continue;
                            }

                            if *ac + total_ac == needed_ac {
                                total_ac_cost += cost;

                                if total_ac_cost < lowest_cost_ac {
                                    lowest_cost_ac = total_ac_cost;
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }

    if lowest_cost_ac == u32::MAX {
        lowest_cost_ac = 0;
    }

    lowest_cost_atk + lowest_cost_ac
}

fn get_highest_cost(inventory: &Vec<Inventory>, needed_atk: u32, needed_ac: u32) -> u32 {
    let mut highest_cost_atk = 0;
    let mut highest_cost_ac = 0;

    for item in inventory {
        match item {
            Inventory::Weapon { cost, atk } => {
                let mut total_atk_cost = 0;
                let mut total_atk = 0;

                if *atk > needed_atk {
                    continue;
                }

                total_atk += atk;
                total_atk_cost += cost;

                if total_atk == needed_atk {
                    if total_atk_cost > highest_cost_atk {
                        highest_cost_atk = total_atk_cost;
                    }

                    continue;
                }

                for ring in inventory {
                    match ring {
                        Inventory::Ring { cost, atk, ac: _ } => {
                            if *atk == 0 {
                                continue;
                            }

                            if *atk + total_atk == needed_atk {
                                total_atk_cost += cost;

                                if total_atk_cost > highest_cost_atk {
                                    highest_cost_atk = total_atk_cost;
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Inventory::Armor { cost, ac } => {
                let mut total_ac_cost = 0;
                let mut total_ac = 0;

                if *ac > needed_ac {
                    continue;
                }

                total_ac += ac;
                total_ac_cost += cost;

                if total_ac == needed_ac && total_ac_cost > highest_cost_ac {
                    highest_cost_ac = total_ac_cost;

                    continue;
                }

                for ring in inventory {
                    match ring {
                        Inventory::Ring { cost, atk: _, ac } => {
                            if *ac == 0 {
                                continue;
                            }

                            if *ac + total_ac == needed_ac {
                                total_ac_cost += cost;

                                if total_ac_cost > highest_cost_ac {
                                    highest_cost_ac = total_ac_cost;
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Inventory::Ring { cost, atk: _, ac } => {
                let mut total_ac_cost = 0;
                let mut total_ac = 0;

                if *ac > 0 && *ac <= needed_ac {
                    total_ac += ac;
                    total_ac_cost += cost;

                    if total_ac == needed_ac && total_ac_cost > highest_cost_ac {
                        highest_cost_ac = total_ac_cost;
                    }
                }
            }
        }
    }

    highest_cost_atk + highest_cost_ac
}

fn battle(mut player: Character, mut boss: Character, trace: bool) -> bool {
    let mut win = false;

    while player.hp > 0 && boss.hp > 0 {
        player.attack(&mut boss, trace);

        if boss.hp > 0 {
            boss.attack(&mut player, trace);
        } else {
            win = true;
        }
    }

    win
}
