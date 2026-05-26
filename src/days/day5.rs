use std::{collections::HashMap, fs::read_to_string};

pub fn part1() {
    let mut nice_count = 0;

    for line in read_to_string("data/day5.txt").unwrap().lines() {
        if is_nice(line) {
            nice_count += 1;
        }
    }

    println!("{}", nice_count);
}

pub fn part2() {
    let mut nice_count = 0;

    for line in read_to_string("data/day5.txt").unwrap().lines() {
        if is_nice_2(line) {
            nice_count += 1;
        }
    }

    println!("{}", nice_count);

    println!("{}", is_nice_2("xxxxyx"))
}

fn is_nice(input: &str) -> bool {
    let mut vowel_count = 0;

    for c in input.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_count += 1;
        }
    }

    let mut good_pattern_found = false;
    let mut bad_pattern_found = false;

    for window in input.chars().collect::<Vec<_>>().windows(2) {
        if window[0] == window[1] {
            good_pattern_found = true;
        }

        if window == ['a', 'b']
            || window == ['c', 'd']
            || window == ['p', 'q']
            || window == ['x', 'y']
        {
            bad_pattern_found = true;
        }
    }

    (vowel_count >= 3 && good_pattern_found) && !bad_pattern_found
}

fn is_nice_2(input: &str) -> bool {
    let mut repeat_counter = HashMap::new();

    let mut last_window_equal_count = 0;
    let mut last_window = String::new();

    for window in input.chars().collect::<Vec<_>>().windows(2) {
        let hash_key = format!("{}{}", window[0], window[1]);

        if hash_key != last_window || last_window_equal_count >= 1 {
            repeat_counter
                .entry(hash_key.clone())
                .and_modify(|f| *f += 1)
                .or_insert(1);

            last_window_equal_count = 0;
        } else {
            last_window_equal_count += 1;
        }

        last_window = hash_key.clone();
    }

    let double_repeat_found = repeat_counter.values().any(|v| *v > 1);

    let mut repeat_between_found = false;

    for window in input.chars().collect::<Vec<_>>().windows(3) {
        if window[0] == window[2] && window[0] != window[1] {
            repeat_between_found = true;
        }
    }

    double_repeat_found && repeat_between_found
}
