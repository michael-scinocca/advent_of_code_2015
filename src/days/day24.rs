use std::fs::read_to_string;

pub fn part1() {
    let input: Vec<_> = read_to_string("data/day24.txt")
        .unwrap()
        .lines()
        .flat_map(|line| line.parse::<i64>())
        .collect();

    process_input(&input, 3);
}

pub fn part2() {
    let input: Vec<_> = read_to_string("data/day24.txt")
        .unwrap()
        .lines()
        .flat_map(|line| line.parse::<i64>())
        .collect();

    process_input(&input, 4);
}

fn process_input(input: &[i64], groups: i64) {
    let sum: i64 = input.iter().sum();

    let group_sum = sum / groups;

    let combinations = find_combinations(group_sum, input);

    let min_length = combinations.iter().map(|x| x.len()).min().unwrap();

    let answer = combinations
        .iter()
        .filter(|x| x.len() == min_length)
        .map(|x| x.iter().product::<i64>())
        .min()
        .unwrap();

    println!("{answer}");
}

fn find_combinations(total: i64, weights: &[i64]) -> Vec<Vec<i64>> {
    let mut combination = Vec::new();
    let mut combinations = Vec::new();

    find_working_combination(total, weights, &mut combination, &mut combinations);

    combinations
}

fn find_working_combination(
    total: i64,
    weights: &[i64],
    combination: &mut Vec<i64>,
    combinations: &mut Vec<Vec<i64>>,
) {
    for i in 0..=1 {
        let weight = &weights[0];

        if i == 1 {
            combination.push(*weight);
        } else {
            let position = combination.iter().position(|x| x == weight);

            if let Some(position) = position {
                combination.remove(position);
            }
        }

        if weights.len() > 1 {
            find_working_combination(total, &weights[1..], combination, combinations);
        } else {
            let working_total: i64 = combination.iter().sum();

            if working_total == total {
                combinations.push(combination.clone());
            }
        }
    }
}
