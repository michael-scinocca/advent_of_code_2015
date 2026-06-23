use std::{fs::read_to_string, iter::Peekable, str::Chars};

pub fn part1() {
    let input = read_to_string("data/day12.txt").unwrap();

    let sum = get_sum(&input);

    println!("{sum}");
}

pub fn part2() {
    let input = read_to_string("data/day12.txt").unwrap();

    let sum = get_sum_no_red(&input);

    println!("{sum}");
}

fn get_sum(input: &str) -> i32 {
    let mut sum = 0;

    let input = input.replace(" ", "");

    let mut chars = input.chars().peekable();

    sum += parse_value(&mut chars, &vec![]).0;

    sum
}

fn get_sum_no_red(input: &str) -> i32 {
    let mut sum = 0;

    let input = input.replace(" ", "");

    let mut chars = input.chars().peekable();

    sum += parse_value(&mut chars, &vec!["\"red\""]).0;

    sum
}

fn parse_value(chars: &mut Peekable<Chars<'_>>, blacklist: &Vec<&str>) -> (i32, String) {
    let mut sum = 0;
    let mut property_value = String::new();
    
    if chars.peek().unwrap() == &'{' {
        sum += parse_object(chars, blacklist);
    } else if chars.peek().unwrap() == &'[' {
        sum += parse_array(chars, blacklist);
    } else {
        for char in chars {
            if char == ',' {
                break;
            }

            property_value.push(char);
        }

        if !property_value.starts_with('"') {
            let number_value: i32 = property_value.parse().unwrap();

            sum += number_value;
        }
    }

    (sum, property_value)
}

fn parse_array(chars: &mut Peekable<Chars<'_>>, blacklist: &Vec<&str>) -> i32 {
    let mut sum = 0;

    let block = parse_block(chars, '[', ']');

    let mut chars = block.chars().peekable();

    while let Some(char) = chars.peek() {
        if char == &',' {
            chars.next();
        }

        sum += parse_value(&mut chars, blacklist).0;
    }

    sum
}

fn parse_object(chars: &mut Peekable<Chars<'_>>, blacklist: &Vec<&str>) -> i32 {
    let mut sum = 0;

    let block = parse_block(chars, '{', '}');
    
    let mut chars = block.chars().peekable();

    while let Some(char) = chars.next() {
        if char == ':' {
            let result = parse_value(&mut chars, blacklist);

            if blacklist.contains(&result.1.as_str()) {
                return 0;
            }

            sum += result.0;
        }
    }

    sum
}

fn parse_block(chars: &mut Peekable<Chars<'_>>, start_block: char, end_block: char) -> String {
    let mut current_block = String::new();

    let mut block_depth = 0;

    for char in chars {
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

    let current_block = current_block.strip_prefix(start_block).unwrap();
    let current_block = current_block.strip_suffix(end_block).unwrap();

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
    #[case(
        "[[\"orange\",\"green\",\"green\",\"red\",-25],-16,104,177,\"red\"],",
        240
    )]
    fn test_get_sum(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(get_sum(input), expected);
    }

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case("[1,{\"c\":\"red\",\"b\":2},3]", 4)]
    #[case("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", 0)]
    #[case("[1,\"red\",5]", 6)]
    fn test_get_sum_no_red(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(get_sum_no_red(input), expected);
    }
}
