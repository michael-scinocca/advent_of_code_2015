use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
struct PersonLink {
    name: String,
    other_name: String,
    score: i32,
}

#[derive(Clone)]
struct Arrangement {
    pub people: Vec<String>,
    pub score: i32,
}

pub fn part1() {
    let hash_map = parse_input(&read_to_string("data/day13.txt").unwrap());

    let mut arrangements = Vec::new();

    for person in hash_map.keys() {
        let mut arrangement = Arrangement {
            people: Vec::new(),
            score: 0,
        };

        arrangement.people.push(person.clone());

        find_arrangements(&hash_map, person, &mut arrangement, &mut arrangements);
    }

    let max_score = arrangements.into_iter().map(|a| a.score).max().unwrap();

    println!("{max_score}");
}

pub fn part2() {
    let mut hash_map = parse_input(&read_to_string("data/day13.txt").unwrap());

    let me = String::from("Me");

    for person in hash_map.values_mut() {
        person.insert(me.clone(), 0);
    }

    hash_map.insert(me.clone(), {
        let mut map = HashMap::new();

        for person in hash_map.keys() {
            map.insert(person.clone(), 0);
        }

        map
    });

    let mut arrangements = Vec::new();

    for person in hash_map.keys() {
        let mut arrangement = Arrangement {
            people: Vec::new(),
            score: 0,
        };

        arrangement.people.push(person.clone());

        find_arrangements(&hash_map, person, &mut arrangement, &mut arrangements);
    }

    let max_score = arrangements.into_iter().map(|a| a.score).max().unwrap();

    println!("{max_score}");
}

fn find_arrangements(
    hash_map: &HashMap<String, HashMap<String, i32>>,
    person: &String,
    arrangement: &mut Arrangement,
    arrangements: &mut Vec<Arrangement>,
) {
    let visitable_people: Vec<_> = hash_map
        .get(person)
        .unwrap()
        .iter()
        .filter(|x| !arrangement.people.contains(x.0))
        .collect();

    if visitable_people.is_empty() {
        let first_last_score = hash_map
            .get(&arrangement.people[0])
            .unwrap()
            .get(&arrangement.people[arrangement.people.len() - 1])
            .unwrap();

        arrangement.score += first_last_score;
        arrangements.push(arrangement.clone());
        arrangement.score -= first_last_score;

        return;
    }

    for link in visitable_people {
        arrangement.people.push(link.0.clone());
        arrangement.score += link.1;

        find_arrangements(hash_map, link.0, arrangement, arrangements);

        arrangement.people.pop();
        arrangement.score -= link.1;
    }
}

fn parse_input(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut people = Vec::new();

    for mut line in input.lines() {
        line = line.trim_matches('.');

        let mut words = line.split(' ');

        let person = String::from(words.next().unwrap());
        let mut happiness = 0;
        let mut next_person = "";

        while let Some(word) = words.next() {
            if word == "gain" {
                happiness = words.next().unwrap().parse::<i32>().unwrap();
            } else if word == "lose" {
                happiness = -words.next().unwrap().parse::<i32>().unwrap();
            }

            next_person = word;
        }

        let person_link = PersonLink {
            name: person,
            other_name: String::from(next_person),
            score: happiness,
        };

        people.push(person_link);
    }

    let people_clone = people.clone();

    for person_link in people.iter_mut() {
        let others = people_clone
            .clone()
            .into_iter()
            .filter(|x| x.other_name == person_link.name && x.name == person_link.other_name)
            .collect::<Vec<_>>();

        let person_link_other = others.first().unwrap();

        person_link.score += person_link_other.score;
    }

    let mut names = people
        .clone()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>();
    names.dedup();

    let mut hash_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for person in names {
        let person_map = hash_map.entry(person.clone()).or_default();

        let person_links = people.iter().filter(|x| x.name == person);

        for person_link in person_links {
            person_map
                .entry(person_link.other_name.clone())
                .or_insert(person_link.score);
        }
    }

    hash_map
}
