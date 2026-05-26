use std::collections::HashSet;
use std::fs::read_to_string;

struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn hash_key(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

pub fn part1() {
    let input = read_to_string("data/day3.txt").unwrap();

    let mut houses = HashSet::new();

    let mut point = Point { x: 0, y: 0 };

    let mut presents = 1;

    houses.insert(point.hash_key());

    for direction in input.chars() {
        match direction {
            '>' => point.x += 1,
            '<' => point.x -= 1,
            '^' => point.y += 1,
            'v' => point.y -= 1,
            _ => (),
        };

        let hash_key = point.hash_key();

        if !houses.contains(&hash_key) {
            presents += 1;
            houses.insert(hash_key);
        }
    }

    println!("{}", presents);
}

pub fn part2() {
    let input = read_to_string("data/day3.txt").unwrap();

    let mut houses = HashSet::new();

    let mut turn = 0;
    let mut point1 = Point { x: 0, y: 0 };
    let mut point2 = Point { x: 0, y: 0 };

    let mut presents = 1;

    houses.insert(point1.hash_key());

    for direction in input.chars() {
        let mover = match turn {
            0 => {
                turn = 1;
                &mut point1
            }
            1 => {
                turn = 0;
                &mut point2
            }
            _ => &mut point1,
        };

        match direction {
            '>' => mover.x += 1,
            '<' => mover.x -= 1,
            '^' => mover.y += 1,
            'v' => mover.y -= 1,
            _ => (),
        };

        let hash_key = mover.hash_key();

        if !houses.contains(&hash_key) {
            presents += 1;
            houses.insert(hash_key);
        }
    }

    println!("{}", presents);
}
