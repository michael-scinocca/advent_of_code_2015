use std::{collections::HashMap, fs::read_to_string};

pub fn part1() {
    let mut sue = HashMap::new();
    sue.insert("children", 3);
    sue.insert("cats", 7);
    sue.insert("samoyeds", 2);
    sue.insert("pomeranians", 3);
    sue.insert("akitas", 0);
    sue.insert("vizslas", 0);
    sue.insert("goldfish", 5);
    sue.insert("trees", 3);
    sue.insert("cars", 2);
    sue.insert("perfumes", 1);

    for line in read_to_string("data/day16.txt").unwrap().lines() {
        let definitions = line.split_once(":").unwrap();

        let name = definitions.0;

        let mut is_sue = true;

        for mut spec in definitions.1.split(",") {
            spec = spec.trim();

            let mut spec_definition = spec.trim().split(":");

            let spec_name = spec_definition.next().unwrap();
            let spec_quantity = spec_definition
                .next()
                .unwrap()
                .trim()
                .trim_matches(',')
                .parse::<u32>()
                .unwrap();

            if sue.get(spec_name).unwrap() != &spec_quantity {
                is_sue = false;
                break;
            }
        }

        if is_sue {
            println!("{}", name);
        }
    }
}

pub fn part2() {
    let mut sue = HashMap::new();
    sue.insert("children", 3);
    sue.insert("cats", 7);
    sue.insert("samoyeds", 2);
    sue.insert("pomeranians", 3);
    sue.insert("akitas", 0);
    sue.insert("vizslas", 0);
    sue.insert("goldfish", 5);
    sue.insert("trees", 3);
    sue.insert("cars", 2);
    sue.insert("perfumes", 1);

    for line in read_to_string("data/day16.txt").unwrap().lines() {
        let definitions = line.split_once(":").unwrap();

        let name = definitions.0;

        let mut is_sue = true;

        for mut spec in definitions.1.split(",") {
            spec = spec.trim();

            let mut spec_definition = spec.trim().split(":");

            let spec_name = spec_definition.next().unwrap();
            let spec_quantity = spec_definition
                .next()
                .unwrap()
                .trim()
                .trim_matches(',')
                .parse::<u32>()
                .unwrap();

            let satisfied = match spec_name {
                "cats" | "trees" => sue.get(spec_name).unwrap() < &spec_quantity,
                "pomeranians" | "goldfish" => sue.get(spec_name).unwrap() > &spec_quantity,
                _ => sue.get(spec_name).unwrap() == &spec_quantity,
            };

            if !satisfied {
                is_sue = false;
                break;
            }
        }

        if is_sue {
            println!("{}", name);
        }
    }
}
