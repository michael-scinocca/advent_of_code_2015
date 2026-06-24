use std::{cmp::Reverse, collections::HashMap, fs::read_to_string};

#[derive(Clone)]
struct PersonLink {
    name: String,
    score: i32,
}

pub fn part1() {
    let people = parse_input(&read_to_string("data/day13.txt").unwrap());

    let mut score = 0;

    let mut partner_map: HashMap<String, Vec<String>> = HashMap::new();

    for key in people.keys() {
        println!("Person {key}");

        let mut links = people.get(key).unwrap().clone();
        links.sort_unstable_by_key(|b| Reverse(b.score));

        let mut partners = Vec::new();

        if partner_map.contains_key(key) {
            let locked_partners = partner_map.get(key).unwrap().clone();

            for locked_partner in locked_partners {
                let filtered = links.iter().filter(|x| x.name == *locked_partner).collect::<Vec<_>>();
                
                let partner = *filtered.first().unwrap();
                
                partners.push(partner);
            }
        }

        if partners.len() < 2 {
            for link in &links {
                let link_full = partner_map.contains_key(&link.name) && partner_map.get(&link.name).unwrap().len() >= 2;
                
                if !link_full && partners.iter().filter(|x| x.name == link.name).count() == 0 {
                    partners.push(link);
                }

                if partners.len() >= 2 {
                    break;
                }
            }
        }

        for link in partners {
            println!("Link {} {}", link.name, link.score);

            partner_map
                .entry(key.clone())
                .and_modify(|links| links.push(link.name.clone()))
                .or_insert(vec![link.name.clone()]);

            partner_map
                .entry(link.name.clone())
                .and_modify(|links| links.push(key.clone()))
                .or_insert(vec![key.clone()]);
            
            score += link.score;
        }
    }

    println!("{score}");
}

pub fn part2() {}

fn parse_input(input: &str) -> HashMap<String, Vec<PersonLink>> {
    let mut people: HashMap<String, Vec<PersonLink>> = HashMap::new();

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
            name: String::from(next_person),
            score: happiness,
        };

        people
            .entry(person)
            .and_modify(|links| links.push(person_link.clone()))
            .or_insert(vec![person_link.clone()]);
    }

    people
}
