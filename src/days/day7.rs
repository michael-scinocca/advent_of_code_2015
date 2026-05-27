use std::{collections::HashMap, fs::read_to_string};

pub fn part1() {
    let mut dictionary = HashMap::new();
    let mut cache = HashMap::new();

    for line in read_to_string("data/day7.txt").unwrap().lines() {
        process_input(&mut dictionary, line);
    }

    let key = String::from("a");

    println!(
        "{}: {}",
        key,
        dictionary.get(&key).unwrap().operate(&dictionary, &mut cache)
    );
}

pub fn part2() {
    let mut dictionary = HashMap::new();
    let mut cache = HashMap::new();

    cache.insert(String::from("b"), 16076);
    
    for line in read_to_string("data/day7.txt").unwrap().lines() {
        process_input(&mut dictionary, line);
    }

    let key = String::from("a");

    println!(
        "{}: {}",
        key,
        dictionary.get(&key).unwrap().operate(&dictionary, &mut cache)
    );
}

fn process_input(dictionary: &mut HashMap<String, Operation>, input: &str) {
    let mut input_parts = input.split("->");

    let statement = input_parts.next().unwrap();
    let wire = input_parts.next().unwrap().trim().to_string();

    let result = {
        if statement.contains("AND") {
            let operation_params = get_operation_params(statement, "AND");

            Operation::And(operation_params.0, operation_params.1)
        } else if statement.contains("OR") {
            let operation_params = get_operation_params(statement, "OR");

            Operation::Or(operation_params.0, operation_params.1)
        } else if statement.contains("LSHIFT") {
            let operation_params = get_operation_params(statement, "LSHIFT");

            Operation::LShift(operation_params.0, operation_params.1)
        } else if statement.contains("RSHIFT") {
            let operation_params = get_operation_params(statement, "RSHIFT");

            Operation::RShift(operation_params.0, operation_params.1)
        } else if statement.contains("NOT") {
            let operation_param = OperationParam {
                value: statement.split("NOT").last().unwrap().trim().to_string(),
            };

            Operation::Not(operation_param)
        } else {
            let operation_param = OperationParam {
                value: statement.trim().to_string(),
            };

            Operation::Assign(operation_param)
        }
    };

    dictionary.entry(wire.clone()).or_insert(result);
}

fn get_operation_params(statement: &str, operator: &str) -> (OperationParam, OperationParam) {
    let statement_parts = statement
        .split(operator)
        .map(|v| v.trim())
        .collect::<Vec<_>>();

    (
        OperationParam {
            value: statement_parts[0].to_string(),
        },
        OperationParam {
            value: statement_parts[1].to_string(),
        },
    )
}

struct OperationParam {
    value: String,
}

impl OperationParam {
    fn resolve(&self, dictionary: &HashMap<String, Operation>, cache: &mut HashMap<String, u16>) -> u16 {
        let key = self.value.clone();
        
        if cache.contains_key(&key) {
            return *cache.get(&key).unwrap();
        }

        let resolved = key
            .parse()
            .unwrap_or_else(|_| dictionary.get(&key).unwrap().operate(dictionary, cache));

        cache.insert(key, resolved);
        
        resolved
    }
}

enum Operation {
    Assign(OperationParam),
    Not(OperationParam),
    And(OperationParam, OperationParam),
    Or(OperationParam, OperationParam),
    LShift(OperationParam, OperationParam),
    RShift(OperationParam, OperationParam),
}

impl Operation {
    fn operate(&self, dictionary: &HashMap<String, Operation>, cache: &mut HashMap<String, u16>) -> u16 {
        match self {
            Operation::Assign(val) => val.resolve(dictionary, cache),
            Operation::Not(val) => !val.resolve(dictionary, cache),
            Operation::And(val1, val2) => val1.resolve(dictionary, cache) & val2.resolve(dictionary, cache),
            Operation::Or(val1, val2) => val1.resolve(dictionary, cache) | val2.resolve(dictionary, cache),
            Operation::LShift(val1, shift) => val1.resolve(dictionary, cache) << shift.resolve(dictionary, cache),
            Operation::RShift(val1, shift) => val1.resolve(dictionary, cache) >> shift.resolve(dictionary, cache),
        }
    }
}
