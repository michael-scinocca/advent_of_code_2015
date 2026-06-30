use std::collections::HashMap;

pub fn part1() {
    let total = 150;
    let buckets = [
        43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38,
    ];

    let (combinations, _) = find_combinations(total, &buckets);

    println!("{}", combinations);
}

pub fn part2() {
    let total = 150;
    let buckets = [
        43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38,
    ];

    let (_, min_bucket_combinations) = find_combinations(total, &buckets);

    println!("{}", min_bucket_combinations);
}

fn find_combinations(total: i32, buckets: &[i32]) -> (i32, i32) {
    let mut combinations = 0;

    let mut combination = Vec::new();
    let mut combo_hashmap = HashMap::new();

    find_working_combination(
        total,
        buckets,
        &mut combination,
        &mut combinations,
        &mut combo_hashmap,
    );

    let min_buckets = combo_hashmap.keys().min().unwrap();

    (combinations, combo_hashmap[min_buckets])
}

fn find_working_combination(
    total: i32,
    buckets: &[i32],
    combination: &mut Vec<i32>,
    combinations: &mut i32,
    combo_hashmap: &mut HashMap<usize, i32>,
) {
    for i in 0..=1 {
        let bucket = &buckets[0];

        if i == 1 {
            combination.push(*bucket);
        } else {
            let position = combination.iter().position(|x| x == bucket);

            if let Some(position) = position {
                combination.remove(position);
            }
        }

        if buckets.len() > 1 {
            find_working_combination(
                total,
                &buckets[1..],
                combination,
                combinations,
                combo_hashmap,
            );
        } else {
            let working_total: i32 = combination.iter().sum();

            if working_total == total {
                *combinations += 1;

                combo_hashmap
                    .entry(combination.len())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }
}
