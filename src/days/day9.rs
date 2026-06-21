use std::{collections::HashMap, fs::read_to_string, str::FromStr};

struct Trip {
    pub from: String,
    pub to: String,
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

fn process_data() -> HashMap<String, HashMap<String, u32>> {
    let mut hash_map = HashMap::new();

    for line in read_to_string("data/day9.txt").unwrap().lines() {
        let trip = line.parse::<Trip>().unwrap();

        let from_location = hash_map.entry(trip.from.clone()).or_insert(HashMap::new());

        from_location
            .entry(trip.to.clone())
            .or_insert(trip.distance);

        let to_location = hash_map.entry(trip.to.clone()).or_insert(HashMap::new());

        to_location
            .entry(trip.from.clone())
            .or_insert(trip.distance);
    }

    hash_map
}

pub fn part1() {
    let hash_map = process_data();

    let mut trips = Vec::new();

    let mut journies = Vec::new();

    for city in hash_map.keys() {
        let mut journey = Journey {
            cities: Vec::new(),
            distance: 0,
        };

        println!("Find journey for {city}");

        journey.cities.push(city.clone());

        let mut city_journies = Vec::new();

        find_journey(&hash_map, city, &mut journey, &mut city_journies);

        journies.push(city_journies);
    }

    for journey in journies.iter().flatten() {
        println!("{} = {}", journey.cities.join(" -> "), journey.distance);

        trips.push(journey.distance);
    }
    
    let min_trip = trips.into_iter().min().unwrap();

    println!("{}", min_trip);
}

pub fn part2() {
    let hash_map = process_data();

    let mut trips = Vec::new();

    let mut journies = Vec::new();

    for city in hash_map.keys() {
        let mut journey = Journey {
            cities: Vec::new(),
            distance: 0,
        };

        journey.cities.push(city.clone());

        let mut city_journies = Vec::new();

        find_journey(&hash_map, city, &mut journey, &mut city_journies);

        journies.push(city_journies);
    }

    for journey in journies.iter().flatten() {
        println!("{} = {}", journey.cities.join(" -> "), journey.distance);

        trips.push(journey.distance);
    }
    
    let max_trip = trips.into_iter().max().unwrap();

    println!("{}", max_trip);
}

fn find_journey(
    hash_map: &HashMap<String, HashMap<String, u32>>,
    city: &String,
    journey: &mut Journey,
    journies: &mut Vec<Journey>,
) {
    let visitable_links = hash_map
        .get(city)
        .unwrap()
        .iter()
        .filter(|x| !journey.cities.contains(x.0))
        .collect::<Vec<(&String, &u32)>>();

    if visitable_links.len() == 0 {
        journies.push(journey.clone());

        return;
    }

    for link in visitable_links {
        journey.cities.push(link.0.clone());
        journey.distance += link.1;

        find_journey(hash_map, link.0, journey, journies);

        journey.cities.pop();
        journey.distance -= link.1;
    }
}