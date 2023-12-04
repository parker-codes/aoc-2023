fn main() {
    let input = include_str!("../input/4.txt");
    let result: usize = input
        .lines()
        .map(into_card)
        .inspect(|card| {
            dbg!(&card);
            return ();
        })
        .filter_map(count_winning_numbers)
        .inspect(|w| {
            dbg!(&w);
            return ();
        })
        .map(|n| usize::pow(2, (n - 1) as u32))
        .inspect(|c| {
            dbg!(&c);
            return ();
        })
        .sum();
    println!("Result: {result}");
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    set: Vec<usize>,
}

// Input is like `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`
fn into_card(input: &str) -> Card {
    let buffer = input.clone();

    // Trim "Card " from the start
    let parts = buffer[5..].split(':').collect::<Vec<_>>();

    let id = parts.get(0).unwrap().trim().parse().unwrap();

    let mut data = parts.get(1).unwrap().split('|');
    let winning = data
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
    let set = data
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    Card { id, winning, set }
}

// count how many of the winning numbers are in the set
fn count_winning_numbers(card: Card) -> Option<usize> {
    let count = card.winning.iter().filter(|n| card.set.contains(n)).count();
    if count > 0 {
        Some(count)
    } else {
        None
    }
}
