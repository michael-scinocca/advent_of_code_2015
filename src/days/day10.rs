pub fn part1() {
    let mut input = "1113122113";

    let mut output = String::new();

    for _ in 0..40 {
        output = transform(input);
        input = output.as_str();
    }

    println!("{}", output.len());
}

pub fn part2() {
    let mut input = "1113122113";

    let mut output = String::new();

    for _ in 0..50 {
        output = transform(input);
        input = output.as_str();
    }

    println!("{}", output.len());
}

fn transform(input: &str) -> String {
    let mut output = String::new();

    let mut count = 0;
    let mut current_digit = ' ';

    for digit in input.chars() {
        if current_digit == ' ' {
            current_digit = digit;
        }

        if digit == current_digit {
            count += 1;
        } else {
            output.push_str(&count.to_string());
            output.push(current_digit);

            count = 1;
            current_digit = digit;
        }
    }

    output.push_str(&count.to_string());
    output.push(current_digit);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(transform("1"), "11");
    }

    #[test]
    fn test_11() {
        assert_eq!(transform("11"), "21");
    }

    #[test]
    fn test_21() {
        assert_eq!(transform("21"), "1211");
    }

    #[test]
    fn test_1211() {
        assert_eq!(transform("1211"), "111221");
    }

    #[test]
    fn test_111221() {
        assert_eq!(transform("111221"), "312211");
    }
}
