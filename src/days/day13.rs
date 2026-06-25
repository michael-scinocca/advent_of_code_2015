use std::{cmp::Reverse, collections::HashMap, fs::read_to_string};

#[derive(Clone)]
struct PersonLink {
    name: String,
    other_name: String,
    score: i32,
    score_other: i32,
    total_score: i32,
}

pub fn part1() {
    let mut people = parse_input(&read_to_string("data/day13.txt").unwrap());
    people.sort_unstable_by_key(|b| Reverse(b.score));

    let mut score = 0;

    let mut partner_map: HashMap<String, Vec<String>> = HashMap::new();

    for person_link in people {
        let mut partner_count = 0;
        let mut other_partner_count = 0;

        if partner_map.contains_key(&person_link.name) {
            let partners = partner_map.get(&person_link.name).unwrap().clone();
            partner_count = partners.len();

            if partner_count >= 2 && !partners.contains(&person_link.other_name) {
                continue;
            }
        }

        if partner_map.contains_key(&person_link.other_name) {
            let partners = partner_map.get(&person_link.other_name).unwrap().clone();
            other_partner_count = partners.len();

            if other_partner_count >= 2 && !partners.contains(&person_link.name) {
                continue;
            }
        }

        println!(
            "Link {} {} {} {} {} {}",
            person_link.name,
            person_link.other_name,
            person_link.score,
            person_link.total_score,
            partner_count,
            other_partner_count
        );

        partner_map
            .entry(person_link.name.clone())
            .and_modify(|links| {
                if !links.contains(&person_link.other_name.clone()) {
                    links.push(person_link.other_name.clone());
                }
            })
            .or_insert(vec![person_link.other_name.clone()]);

        partner_map
            .entry(person_link.other_name.clone())
            .and_modify(|links| {
                if !links.contains(&person_link.name.clone()) {
                    links.push(person_link.name.clone());
                }
            })
            .or_insert(vec![person_link.name.clone()]);

        score += person_link.score;
    }

    println!("{score}");
}

pub fn part2() {}

fn parse_input(input: &str) -> Vec<PersonLink> {
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
            score_other: 0,
            total_score: 0,
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

        person_link.score_other = person_link_other.score;
        person_link.total_score = person_link.score + person_link.score_other;
    }

    people
}
