use std::{fs::read_to_string, iter::Peekable, str::Chars};

pub fn part1() {
    let input = read_to_string("data/day12.txt").unwrap();

    let sum = get_sum(&input);

    println!("{sum}");
}

pub fn part2() {}

fn get_sum(input: &str) -> i32 {
    let mut sum = 0;

    let input = input.replace(" ", "");

    let mut chars = input.chars().peekable();
    
    sum += parse_value(&mut chars);

    sum
}

fn parse_value(chars: &mut Peekable<Chars<'_>>) -> i32 {
    let mut sum = 0;

    println!("Parse value {}", chars.clone().collect::<String>());

    if chars.peek().unwrap() == &'{' {
        sum += parse_object(chars);
    } else if chars.peek().unwrap() == &'[' {
        sum += parse_array(chars);
    } else {
        let mut property_value = String::new();

        while let Some(char) = chars.next() {
            if char == ',' {
                break;
            }

            property_value.push(char);
        }

        if !property_value.starts_with('"') {
            println!("Parse int {}", property_value);
            
            let number_value: i32 = property_value.parse().unwrap();

            sum += number_value;
        }
    }

    sum
}

fn parse_array(chars: &mut Peekable<Chars<'_>>) -> i32 {
    let mut sum = 0;

    let block = parse_block(chars, '[', ']');

    println!("Array {}", block);
     
    let value = &mut String::from(block);
    
    let mut chars = value.chars().peekable();
    
    while let Some(_) = chars.peek() {
        println!("Value {}", chars.clone().collect::<String>());
        
        sum += parse_value(&mut chars);
    }

    sum
}

fn parse_object(chars: &mut Peekable<Chars<'_>>) -> i32 {
    let mut sum = 0;
    
    let block = parse_block(chars, '{', '}');

    println!("Object {}", block);
        
    let value = &mut String::from(block);
    
    let mut chars = value.chars().peekable();
    
    while let Some(char) = chars.next() {
        if char == ':' {
            println!("Value {}", chars.clone().collect::<String>());
            
            sum += parse_value(&mut chars);
        }
    }

    sum
}

fn parse_block(chars: &mut Peekable<Chars<'_>>, start_block: char, end_block: char) -> String {
    let mut current_block = String::new();

    let mut block_depth = 0;

    while let Some(char) = chars.next() {
        current_block.push(char);

        if char == start_block {
            block_depth += 1;

            continue;
        } else if char == end_block {
            block_depth -= 1;

            if block_depth == 0 {
                break;
            }
            
            continue;
        } 
    }

    let current_block = current_block.trim_start_matches(start_block);
    let current_block = current_block.trim_end_matches(end_block);

    String::from(current_block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case("{\"a\":2,\"b\":4}", 6)]
    #[case("[[[3]]]", 3)]
    #[case("{\"a\":{\"b\":4},\"c\":-1}", 3)]
    #[case("{\"a\":[-1,1]}", 0)]
    #[case("[-1,{\"a\":1}]", 0)]
    #[case("{\"a\": 1, \"b\": \"hello\"}", 1)]
    #[case("{\"e\":[[{\"e\":86,\"c\":1}]]}", 87)]
    fn test_get_sum(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(get_sum(input), expected);
    }
}
