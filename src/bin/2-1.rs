fn main() {
    let input = include_str!("../input/2.txt");
    let rows = input.lines().collect::<Vec<_>>();
    let games = rows.into_iter().map(into_game);

    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let limit = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = games
        .filter(|game| {
            game.sets.iter().all(|set| {
                // each color must be <= limit
                set.red <= limit.red
                    && set.green <= limit.green
                    && set.blue <= limit.blue
                // total amount must be less than limit
                    && (set.red + set.green + set.blue) <= (limit.red + limit.green + limit.blue)
            })
        })
        .fold(0, |acc, game| acc + game.id);
    println!("Result: {result}");
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

// Input is like `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
fn into_game(input: &str) -> Game {
    let buffer = input.clone();

    // Trim "Game " from the start
    let parts = buffer[5..].split(':').collect::<Vec<_>>();

    let id = parts.get(0).unwrap().parse().unwrap();
    let sets = parts
        .get(1)
        .unwrap()
        .split(";")
        .into_iter()
        .map(into_set)
        .collect::<Vec<_>>();

    Game { id, sets }
}

// Input is like `3 blue, 4 red`, in any order
fn into_set(input: &str) -> Set {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for part in input.split(',') {
        let part = part.trim();
        // get number after trimming " {color}" from the end
        if part.ends_with("red") {
            red = part[0..part.len() - 4].parse().unwrap();
        } else if part.ends_with("green") {
            green = part[0..part.len() - 6].parse().unwrap();
        } else if part.ends_with("blue") {
            blue = part[0..part.len() - 5].parse().unwrap();
        }
    }

    Set { red, green, blue }
}
