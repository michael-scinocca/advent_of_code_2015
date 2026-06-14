use std::{collections::HashMap, fs::read_to_string, str::FromStr};

struct Trip {
    pub from: String,
    pub to: String,
    pub distance: u32,
}

struct Location {
    pub name: String,
    pub links: HashMap<String, Link>,
}

struct Link {
    pub location_name: String,
    pub distance: u32,
}

#[derive(Clone)]
struct Journey {
    pub cities: Vec<String>,
    pub distance: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTripError;

impl FromStr for Trip {
    type Err = ParseTripError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('=');

        let from_to = parts.next().unwrap();
        let distance = parts.next().unwrap();

        let mut from_to_parts = from_to.split("to");

        let from = from_to_parts.next().unwrap();
        let to = from_to_parts.next().unwrap();

        Ok(Trip {
            from: String::from(from.trim()),
            to: String::from(to.trim()),
            distance: distance.trim().parse().unwrap(),
        })
    }
}

pub fn part1() {
    let mut hash_map = HashMap::new();

    for line in read_to_string("data/day9.txt").unwrap().lines() {
        let trip = line.parse::<Trip>().unwrap();

        let from_location = hash_map.entry(trip.from.clone()).or_insert(Location {
            name: trip.from.clone(),
            links: HashMap::new(),
        });

        from_location.links.entry(trip.to.clone()).or_insert(Link {
            location_name: trip.to.clone(),
            distance: trip.distance,
        });

        let to_location = hash_map.entry(trip.to.clone()).or_insert(Location {
            name: trip.to.clone(),
            links: HashMap::new(),
        });

        to_location.links.entry(trip.from.clone()).or_insert(Link {
            location_name: trip.from.clone(),
            distance: trip.distance,
        });
    }

    let mut trips = Vec::new();

    for key in hash_map.keys() {
        println!("Main {}", key);

        let journies = find_journies(&hash_map, key);

        for journey in journies {
            println!("{} = {}", journey.cities.join(" -> "), journey.distance);

            trips.push(journey.distance);
        }
    }

    let min_trip = trips.into_iter().min().unwrap();

    println!("{}", min_trip);
}

fn find_journies(hash_map: &HashMap<String, Location>, key: &String) -> Vec<Journey> {
    println!("Find journies for {key}");

    let mut journies = Vec::new();

    for link in &hash_map.get(key).unwrap().links {
        let mut journey = Journey {
            cities: Vec::new(),
            distance: 0,
        };

        println!("Moving to {}", link.1.location_name);

        journey.cities.push(key.clone());

        journey.cities.push(link.1.location_name.clone());

        journey.distance += link.1.distance;

        continue_journey(hash_map, &link.1.location_name, &mut journey, &mut journies);
    }

    journies
}

fn continue_journey(
    hash_map: &HashMap<String, Location>,
    key: &String,
    journey: &mut Journey,
    journies: &mut Vec<Journey>,
) {
    println!("Continue journey for {key}");

    for link in &hash_map.get(key).unwrap().links {
        if journey.cities.contains(&link.1.location_name) {
            if journey.cities.len() == hash_map.len() {
                println!("Add jouney ending at {}", link.1.location_name);
                
                journies.push(journey.clone());
            }
            
            continue;
        }

        println!("Moving to {}", link.1.location_name);

        journey.cities.push(link.1.location_name.clone());

        journey.distance += link.1.distance;

        continue_journey(hash_map, &link.1.location_name, journey, journies);
    }
}

pub fn part2() {}
