use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Mode {
    Moving,
    Resting,
}

#[derive(Debug)]
struct Reindeer {
    #[allow(dead_code)]
    name: String,
    speed: u32,
    duration: u32,
    rest: u32,
    location: u32,
    mode: Mode,
    mode_tick: u32,
    points: u32,
}

pub fn part1() {
    let mut reindeers = get_reindeers();

    for _ in 0..2503 {
        for reindeer in &mut reindeers {
            if reindeer.mode == Mode::Moving {
                reindeer.location += reindeer.speed;
                reindeer.mode_tick += 1;

                if reindeer.mode_tick == reindeer.duration {
                    reindeer.mode = Mode::Resting;
                    reindeer.mode_tick = 0;

                    continue;
                }
            }

            if reindeer.mode == Mode::Resting {
                reindeer.mode_tick += 1;

                if reindeer.mode_tick == reindeer.rest {
                    reindeer.mode = Mode::Moving;
                    reindeer.mode_tick = 0;

                    continue;
                }
            }
        }
    }

    reindeers.iter().for_each(|x| println!("{:?}", x));

    let max_distance = reindeers.into_iter().map(|x| x.location).max().unwrap();

    println!("{max_distance}");
}

pub fn part2() {
    let mut reindeers = get_reindeers();

    for _ in 0..2503 {
        for reindeer in &mut reindeers {
            if reindeer.mode == Mode::Moving {
                reindeer.location += reindeer.speed;
                reindeer.mode_tick += 1;

                if reindeer.mode_tick == reindeer.duration {
                    reindeer.mode = Mode::Resting;
                    reindeer.mode_tick = 0;

                    continue;
                }
            }

            if reindeer.mode == Mode::Resting {
                reindeer.mode_tick += 1;

                if reindeer.mode_tick == reindeer.rest {
                    reindeer.mode = Mode::Moving;
                    reindeer.mode_tick = 0;

                    continue;
                }
            }
        }

        let max_distance = reindeers.iter().map(|r| r.location).max().unwrap();

        reindeers
            .iter_mut()
            .filter(|r| r.location == max_distance)
            .for_each(|r| r.points += 1);
    }

    reindeers.iter().for_each(|x| println!("{:?}", x));

    let max_points = reindeers.into_iter().map(|x| x.points).max().unwrap();

    println!("{max_points}");
}

fn get_reindeers() -> Vec<Reindeer> {
    let mut reindeers = Vec::new();

    for line in read_to_string("data/day14.txt").unwrap().lines() {
        let mut parts = line.split(' ');

        let name = parts.next().unwrap().to_string();
        let mut speed = 0;
        let mut duration = 0;
        let mut rest = 0;

        while let Some(part) = parts.next() {
            if part == "fly" {
                speed = parts.next().unwrap().parse().unwrap();
            }

            if part == "for" {
                if duration == 0 {
                    duration = parts.next().unwrap().parse().unwrap();
                } else {
                    rest = parts.next().unwrap().parse().unwrap();
                }
            }
        }

        let reindeer = Reindeer {
            name,
            speed,
            duration,
            rest,
            location: 0,
            mode: Mode::Moving,
            mode_tick: 0,
            points: 0,
        };

        reindeers.push(reindeer);
    }

    reindeers
}
