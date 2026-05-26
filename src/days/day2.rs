use std::fs::read_to_string;

pub fn part1() {
    let mut square_feet = 0;

    for line in read_to_string("data/day2.txt").unwrap().lines() {
        square_feet += get_square_feet(line);
    }

    println!("{}", square_feet);
}

pub fn part2() {
    let mut ribbon = 0;

    for line in read_to_string("data/day2.txt").unwrap().lines() {
        ribbon += get_ribbon(line);
    }

    println!("{}", ribbon);
}

fn get_square_feet(input: &str) -> i32 {
    let dimensions: Vec<i32> = input.split('x').map(|d| d.parse().unwrap()).collect();

    let side_1 = dimensions[0] * dimensions[1];
    let side_2 = dimensions[1] * dimensions[2];
    let side_3 = dimensions[0] * dimensions[2];

    let sides = [side_1, side_2, side_3];

    let gift_area: i32 = 2 * sides.iter().sum::<i32>();

    gift_area + sides.iter().min().unwrap()
}

fn get_ribbon(input: &str) -> i32 {
    let mut dimensions: Vec<i32> = input.split('x').map(|d| d.parse().unwrap()).collect();

    let gift_volume: i32 = dimensions[0] * dimensions[1] * dimensions[2];

    dimensions.sort_unstable();

    gift_volume + dimensions[0] * 2 + dimensions[1] * 2
}
