fn main() {
    let input = include_str!("../input/1.txt");
    let rows = input.split('\n').collect::<Vec<_>>();
    let result = rows.into_iter().map(capture_outer_digits).sum::<u32>();
    println!("Result: {result}");
}

fn capture_outer_digits(input: &str) -> u32 {
    let first = find_first_digit_from(Side::Start, input);
    let last = find_first_digit_from(Side::End, input);

    format!("{}{}", first, last).parse().unwrap_or(0)
}

#[derive(PartialEq)]
enum Side {
    Start,
    End,
}

// NOTE: Turns out this can't be a buffer consumer - it needs to search from the start
// and end (without reversing the string) so that "eighthree" resolves to 83 even though
// there is no extra "T" to match both.
fn find_first_digit_from(side: Side, input: &str) -> u32 {
    let mut buffer = input.clone();
    while !buffer.is_empty() {
        let next_character = match side {
            Side::Start => buffer.chars().next().unwrap(),
            Side::End => buffer.chars().last().unwrap(),
        };
        if next_character.is_digit(10) {
            return next_character.to_digit(10).unwrap();
        } else if let Some(digit) = find_digit_word(&side, buffer) {
            return digit;
        } else {
            // not found, so trim first or last character
            if side == Side::Start {
                buffer = &buffer[1..];
            } else {
                buffer = &buffer[..buffer.len() - 1];
            }
        }
    }

    return 0;
}

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn find_digit_word(side: &Side, input: &str) -> Option<u32> {
    for (word, digit) in DIGIT_WORDS.iter().zip(1..) {
        if side == &Side::Start && input.starts_with(word) {
            return Some(digit);
        } else if side == &Side::End && input.ends_with(word) {
            return Some(digit);
        }
    }

    None
}
