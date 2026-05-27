use std::fs::read_to_string;

pub fn part1() {
    let mut grid: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    for line in read_to_string("data/day6.txt").unwrap().lines() {
        process_input(&mut grid, line);
    }

    let count = grid.iter().flatten().filter(|x| **x == 1).count();

    println!("{}", count);
}

pub fn part2() {
    let mut grid: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    for line in read_to_string("data/day6.txt").unwrap().lines() {
        process_input_2(&mut grid, line);
    }

    let count: u32 = grid.iter().flatten().sum();

    println!("{}", count);
}

#[derive(Debug)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

fn process_input(grid: &mut [[u8; 1000]; 1000], input: &str) {
    let mut input_parts = input.split(' ');

    let input_part = input_parts.next().unwrap();

    let operation = {
        if input_part == "turn" {
            match input_parts.next().unwrap() {
                "on" => Some(Operation::TurnOn),
                "off" => Some(Operation::TurnOff),
                _ => None,
            }
        } else if input_part == "toggle" {
            Some(Operation::Toggle)
        } else {
            None
        }
    };

    let start = {
        let xy = input_parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (xy[0], xy[1])
    };

    input_parts.next();

    let end = {
        let xy = input_parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (xy[0], xy[1])
    };

    let Some(operation) = operation else {
        return;
    };

    for row in grid.iter_mut().take(end.0 + 1).skip(start.0) {
        for square in row.iter_mut().take(end.1 + 1).skip(start.1) {
            match operation {
                Operation::TurnOn => *square = 1,
                Operation::TurnOff => *square = 0,
                Operation::Toggle => {
                    *square = if *square == 1 { 0 } else { 1 };
                }
            }
        }
    }
}

fn process_input_2(grid: &mut [[u32; 1000]; 1000], input: &str) {
    let mut input_parts = input.split(' ');

    let input_part = input_parts.next().unwrap();

    let operation = {
        if input_part == "turn" {
            match input_parts.next().unwrap() {
                "on" => Some(Operation::TurnOn),
                "off" => Some(Operation::TurnOff),
                _ => None,
            }
        } else if input_part == "toggle" {
            Some(Operation::Toggle)
        } else {
            None
        }
    };

    let start = {
        let xy = input_parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (xy[0], xy[1])
    };

    input_parts.next();

    let end = {
        let xy = input_parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (xy[0], xy[1])
    };

    let Some(operation) = operation else {
        return;
    };

    for row in grid.iter_mut().take(end.0 + 1).skip(start.0) {
        for square in row.iter_mut().take(end.1 + 1).skip(start.1) {
            match operation {
                Operation::TurnOn => *square += 1,
                Operation::TurnOff => *square = square.saturating_sub(1),
                Operation::Toggle => *square += 2,
            }
        }
    }
}
