pub fn part1() {
    let input = "vzbxkghb";

    let password = find_next_password(input);

    println!("{password}");
}

pub fn part2() {
    let input = "vzbxkghb";

    let mut password = find_next_password(input);
    password = find_next_password(&password);

    println!("{password}");
}

fn find_next_password(input: &str) -> String {
    let mut next_password = String::from(input);

    next_password = increment_password(&next_password);

    while !is_password_valid(&next_password) {
        next_password = increment_password(&next_password);
    }

    next_password
}

fn increment_password(input: &str) -> String {
    let min = "a".as_bytes()[0];
    let max = "z".as_bytes()[0];

    let mut bytes = String::from(input).into_bytes();

    for i in (0..bytes.len()).rev() {
        let next_val = { if bytes[i] == max { min } else { bytes[i] + 1 } };

        bytes[i] = next_val;

        if next_val != min {
            break;
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn is_password_valid(input: &str) -> bool {
    if input.len() < 8 {
        return false;
    }

    let sequence_exists = input
        .as_bytes()
        .windows(3)
        .any(|x| x[0] == x[1] - 1 && x[1] == x[2] - 1);

    if !sequence_exists {
        return false;
    }

    if input.contains('i') || input.contains('o') || input.contains('l') {
        return false;
    }

    let mut chars = input.chars().peekable();

    let mut repeat_count = 0;
    let mut repeat_char = ' ';

    while let Some(char) = chars.next() {
        if let Some(next_char) = chars.peek()
            && char == *next_char
            && char != repeat_char
        {
            repeat_char = char;
            repeat_count += 1;

            if repeat_count == 2 {
                break;
            }
        }
    }

    if repeat_count < 2 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    fn test_is_password_valid(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_password_valid(input), expected);
    }

    #[rstest]
    #[case("xx", "xy")]
    #[case("xy", "xz")]
    #[case("xz", "ya")]
    #[case("ya", "yb")]
    fn test_increment_password(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(increment_password(input), expected);
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_find_next_password(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(find_next_password(input), expected);
    }
}
