use std::fs::read_to_string;

pub fn part1() {
    let mut total_chars = 0;
    let mut total_in_memory_chars = 0;
    
    for line in read_to_string("data/day8.txt").unwrap().lines() {
        let (chars, in_memory_chars) = calculate_characters(line);

        total_chars += chars;
        total_in_memory_chars += in_memory_chars;
    }

    println!("{}", total_chars - total_in_memory_chars);
}

pub fn part2() {
    let mut total_chars = 0;
    let mut total_escaped_chars = 0;
    
    for line in read_to_string("data/day8.txt").unwrap().lines() {
        let (chars, escaped_chars) = calculate_characters_2(line);

        total_chars += chars;
        total_escaped_chars += escaped_chars;
    }

    println!("{}", total_escaped_chars - total_chars);
}

fn calculate_characters(input: &str) -> (u32, u32) {
    let mut count = 0;

    let mut chars = input.chars();
    while let Some(char) = chars.next() {
        if char == '\\' {
            let next = chars.next().unwrap();

            if next == '\"' || next == '\\' {
                count += 1;
            }

            if next == 'x' {
                chars.next();
                chars.next();

                count += 1;
            }

            continue;
        }
        
        if char != '\"' {
            count += 1;
        }
    }

    (input.chars().count() as u32, count)
}

fn calculate_characters_2(input: &str) -> (u32, u32) {
    let mut count = 0;

    let mut chars = input.chars();
    while let Some(char) = chars.next() {
        if char == '\"' {
            count += 3;

            continue;
        }
        
        if char == '\\' {
            let next = chars.next().unwrap();

            count += 2;

            if next == '\"' || next == '\\' {
                count += 2;
            } else {
                count += 1;
            }
            
            continue;
        }
        
        count += 1;
    }

    (input.chars().count() as u32, count)
}