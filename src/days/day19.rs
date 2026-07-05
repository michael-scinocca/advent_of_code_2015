use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part1() {
    let transformations = get_transformations();

    let seed = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";

    let combinations = get_combinations(seed.to_string(), &transformations);

    println!("{}", combinations.len());
}

pub fn part2() {}

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

fn get_combinations(
    seed: String,
    transformations: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    let mut combinations = HashSet::new();

    for key in transformations.keys() {
        for index in 0..seed.len() {
            if seed[index..].starts_with(key) {
                let char_transformations = transformations.get(key);

                if let Some(char_transformations) = char_transformations {
                    for char_transformation in char_transformations {
                        combinations.insert(
                            String::from(&seed[0..index])
                                + &seed[index..].replacen(&key.to_string(), char_transformation, 1),
                        );
                    }
                }
            }
        }
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_data() {
        let mut transformations = HashMap::new();
        transformations.insert("H".to_owned(), vec!["HO".to_owned(), "OH".to_owned()]);
        transformations.insert("O".to_owned(), vec!["HH".to_owned()]);

        let combinations = get_combinations(String::from("HOH"), &transformations);

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

        let combinations = get_combinations(String::from("HOHOHO"), &transformations);

        assert_eq!(7, combinations.len());
    }

    #[test]
    fn test_test_data_multi_char() {
        let mut transformations = HashMap::new();
        transformations.insert("Ag".to_owned(), vec!["H".to_owned(), "Y".to_owned()]);
        transformations.insert("O".to_owned(), vec!["B".to_owned()]);

        let combinations = get_combinations(String::from("AgKOAg"), &transformations);

        assert_eq!(5, combinations.len());
        assert_eq!(true, combinations.contains("HKOAg"));
        assert_eq!(true, combinations.contains("YKOAg"));
        assert_eq!(true, combinations.contains("AgKOH"));
        assert_eq!(true, combinations.contains("AgKOY"));
        assert_eq!(true, combinations.contains("AgKBAg"));
    }
}
