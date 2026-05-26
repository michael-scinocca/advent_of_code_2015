use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("data/day1.txt").unwrap();

    let mut floor = 0;

    for command in input.chars() {
        floor += match command {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }

    println!("{}", floor);
}

pub fn part2() {
    let input = read_to_string("data/day1.txt").unwrap();

    let mut floor = 0;

    let mut position = 0;

    for command in input.chars() {
        floor += match command {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        position += 1;

        if floor == -1 {
            break;
        }
    }

    println!("{}", position);
}
