fn main() {
    let input = include_str!("../input/3.txt");
    let rows = input.lines().collect::<Vec<_>>();
    let (parts, symbols) = get_parts_and_symbols(rows);

    let result = parts
        .into_iter()
        .filter(|part| {
            symbols
                .clone()
                .into_iter()
                .any(|symbol| is_adjacent(part, &symbol))
        })
        .map(|part| part.number)
        .sum::<u32>();
    println!("Result: {result}");
}

fn get_parts_and_symbols(rows: Vec<&str>) -> (Vec<Part>, Vec<Symbol>) {
    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    for (row_index, row) in rows.iter().enumerate() {
        let mut part_buffer = String::new();

        for (column_index, character) in row.chars().enumerate() {
            if character.is_digit(10) {
                part_buffer.push(character);
            } else {
                if !part_buffer.is_empty() {
                    parts.push(capture_part(&part_buffer, row_index, column_index));
                    part_buffer = String::new(); // clear buffer
                }

                if character == '.' {
                    continue; // ignore empty spots
                } else {
                    symbols.push(Symbol {
                        character,
                        position: Position {
                            row: row_index,
                            column: column_index,
                        },
                    });
                }
            }
        }

        // see if we ended in a part
        if !part_buffer.is_empty() {
            parts.push(capture_part(&part_buffer, row_index, row.len()));
        }
    }

    (parts, symbols)
}

fn capture_part(buffer: &str, row_index: usize, column_index: usize) -> Part {
    let number = buffer.parse().unwrap();
    let start = Position {
        row: row_index,
        column: column_index - buffer.len(),
    };
    let end = Position {
        row: row_index,
        column: column_index - 1, // minus one because we're closing _after_ the digits are done
    };
    Part { number, start, end }
}

fn is_adjacent(part: &Part, symbol: &Symbol) -> bool {
    let is_row_adjacent = (symbol.position.row as i32 - part.start.row as i32).abs() <= 1;
    let is_column_adjacent = (symbol.position.column
        >= part.start.column.checked_sub(1).unwrap_or(0))
        && (symbol.position.column <= part.end.column + 1);

    return is_row_adjacent && is_column_adjacent;
}

#[derive(Debug)]
struct Part {
    number: u32,
    start: Position,
    end: Position,
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    character: char,
    position: Position,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}
