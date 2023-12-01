fn main() {
    let input = include_str!("../input/1.txt");
    let rows = input.split('\n').collect::<Vec<_>>();
    let result = rows.into_iter().map(capture_outer_digits).sum::<u32>();
    println!("Result: {result}");
}

fn capture_outer_digits(input: &str) -> u32 {
    let digits = input.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
    let first = digits.first();
    let last = digits.last();

    format!("{}{}", first.unwrap(), last.unwrap())
        .parse()
        .unwrap()
}
