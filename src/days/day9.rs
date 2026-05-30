use std::{collections::{HashMap, HashSet}, fs::read_to_string, str::FromStr};

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
        let mut cities = Vec::new();
        let mut trip: u32 = 0;

        println!("Main {}", key);

        cities.push(key.clone());
        
        run_trip(&hash_map, &key, &mut cities, &mut trip);

        println!("{} = {}", cities.join(" -> "), trip);
        
        trips.push(trip);
    }

    let min_trip = trips.into_iter().min().unwrap();

    println!("{}", min_trip);
}

fn run_trip(hash_map: &HashMap<String, Location>, key: &String, cities: &mut Vec<String>, trip: &mut u32) {
    println!("Run trip for {key}");
    
    for link in &hash_map.get(key).unwrap().links {
        if !cities.contains(&link.1.location_name) {
            println!("Moving to {}", link.1.location_name);
            
            cities.push(link.1.location_name.clone());

            *trip += link.1.distance;

            run_trip(hash_map, &link.1.location_name, cities, trip);
        }
    }
}

pub fn part2() {}
