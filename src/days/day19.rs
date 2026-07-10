use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part1() {
    let transformations = get_transformations();

    let seed = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";

    let combinations = get_combinations(seed, &transformations);

    println!("{}", combinations.len());
}

pub fn part2() {
    let transformations = get_transformations();

    let seed = "e";
    let target = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";

    let steps = fabricate_molecule(seed, target, &transformations);

    println!("{steps}");
}

fn get_transformations() -> HashMap<String, Vec<String>> {
    let mut transformations = HashMap::new();

    for line in read_to_string("data/day19.txt").unwrap().lines() {
        let mut split = line.split("=>");

        let key = split.next().unwrap().trim().to_string();
        let val = split.next().unwrap().trim().to_string();

        transformations
            .entry(key)
            .and_modify(|t: &mut Vec<String>| t.push(val.clone()))
            .or_insert(vec![val]);
    }

    transformations
}

fn get_combinations(seed: &str, transformations: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut combinations = HashSet::new();

    for key in transformations.keys() {
        for index in 0..seed.len() {
            if seed[index..].starts_with(key) {
                let char_transformations = transformations.get(key);

                if let Some(char_transformations) = char_transformations {
                    for char_transformation in char_transformations {
                        combinations.insert(
                            seed[0..index].to_string()
                                + &seed[index..].replacen(&key.to_string(), char_transformation, 1),
                        );
                    }
                }
            }
        }
    }

    combinations
}

fn fabricate_molecule(
    seed: &str,
    target: &str,
    transformations: &HashMap<String, Vec<String>>,
) -> u32 {
    let mut steps = 0;

    let mut reversed: HashMap<String, Vec<String>> = HashMap::new();

    for (key, vals) in transformations {
        for val in vals {
            reversed.entry(val.clone()).or_default().push(key.clone());
        }
    }

    while steps == 0 {
        let mut working_steps = 0;
        let mut iterations = 0;
        let mut quit = false;

        fabricate_molecule_work(
            target,
            seed,
            &reversed,
            &mut working_steps,
            &mut steps,
            &mut iterations,
            &mut quit,
        );
    }

    steps as u32
}

fn fabricate_molecule_work(
    seed: &str,
    target: &str,
    transformations: &HashMap<String, Vec<String>>,
    working_steps: &mut i32,
    steps: &mut i32,
    iterations: &mut i32,
    quit: &mut bool,
) {
    if *quit {
        return;
    }

    let mut keys: Vec<_> = transformations.keys().collect();
    keys.sort_unstable_by_key(|b| std::cmp::Reverse(b.len()));

    for key in keys {
        for index in 0..seed.len() {
            if seed[index..].starts_with(key) {
                let char_transformations = transformations.get(key);

                if let Some(char_transformations) = char_transformations {
                    for char_transformation in char_transformations {
                        let transformed = seed[0..index].to_string()
                            + &seed[index..].replacen(&key.to_string(), char_transformation, 1);

                        *working_steps += 1;
                        *iterations += 1;

                        if transformed == target && !*quit {
                            *steps = *working_steps;

                            *quit = true;

                            return;
                        }

                        if *iterations > 10000 {
                            *quit = true;

                            return;
                        }

                        if transformed.len() >= target.len() && !*quit {
                            fabricate_molecule_work(
                                &transformed,
                                target,
                                transformations,
                                working_steps,
                                steps,
                                iterations,
                                quit,
                            );

                            *working_steps -= 1;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_data() {
        let mut transformations = HashMap::new();
        transformations.insert("H".to_owned(), vec!["HO".to_owned(), "OH".to_owned()]);
        transformations.insert("O".to_owned(), vec!["HH".to_owned()]);

        let combinations = get_combinations("HOH", &transformations);

        assert_eq!(4, combinations.len());
        assert_eq!(true, combinations.contains("HOOH"));
        assert_eq!(true, combinations.contains("HOHO"));
        assert_eq!(true, combinations.contains("OHOH"));
        assert_eq!(true, combinations.contains("HHHH"));
    }

    #[test]
    fn test_test_data_2() {
        let mut transformations = HashMap::new();
        transformations.insert("H".to_owned(), vec!["HO".to_owned(), "OH".to_owned()]);
        transformations.insert("O".to_owned(), vec!["HH".to_owned()]);

        let combinations = get_combinations("HOHOHO", &transformations);

        assert_eq!(7, combinations.len());
    }

    #[test]
    fn test_test_data_multi_char() {
        let mut transformations = HashMap::new();
        transformations.insert("Ag".to_owned(), vec!["H".to_owned(), "Y".to_owned()]);
        transformations.insert("O".to_owned(), vec!["B".to_owned()]);

        let combinations = get_combinations("AgKOAg", &transformations);

        assert_eq!(5, combinations.len());
        assert_eq!(true, combinations.contains("HKOAg"));
        assert_eq!(true, combinations.contains("YKOAg"));
        assert_eq!(true, combinations.contains("AgKOH"));
        assert_eq!(true, combinations.contains("AgKOY"));
        assert_eq!(true, combinations.contains("AgKBAg"));
    }

    #[test]
    fn test_molecule_fabrications() {
        let mut transformations = HashMap::new();
        transformations.insert("e".to_owned(), vec!["H".to_owned(), "O".to_owned()]);
        transformations.insert("H".to_owned(), vec!["HO".to_owned(), "OH".to_owned()]);
        transformations.insert("O".to_owned(), vec!["HH".to_owned()]);

        let steps = fabricate_molecule("e", "HOH", &transformations);

        assert_eq!(3, steps);

        let steps = fabricate_molecule("e", "HOHOHO", &transformations);

        assert_eq!(6, steps);
    }
}
