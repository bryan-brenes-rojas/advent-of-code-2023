use std::fs;

fn main() {
    let input = fs::read_to_string("assets/test-input.txt").unwrap();
    let cards = process_input(&input);
    for card in cards {
        println!("{:?}", card);
    }
}

fn process_input(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut semi_colon_found = false;
    let mut pipe_found = false;
    let mut number_buffer = String::from("");
    let mut cards: Vec<(Vec<u8>, Vec<u8>)> = vec![];
    let mut winning_numbers: Vec<u8> = vec![];
    let mut number_gotten: Vec<u8> = vec![];

    for char in input.chars() {
        if char == ':' { semi_colon_found = true }
        if !semi_colon_found {
            continue;
        }
        if char.is_digit(10) {
            number_buffer = format!("{number_buffer}{char}");
        } else if char == ' ' && !number_buffer.is_empty() {
            if pipe_found {
                number_gotten.push(number_buffer.parse().unwrap());
            } else {
                winning_numbers.push(number_buffer.parse().unwrap());
            }
            number_buffer.clear();
        } else if char == '\n' {
            number_gotten.push(number_buffer.parse().unwrap());
            cards.push((winning_numbers.clone(), number_gotten.clone()));
            number_buffer.clear();
            winning_numbers.clear();
            number_gotten.clear();
            pipe_found = false;
            semi_colon_found = false;
        } else if char == '|' {
            pipe_found = true
        }
    }
    cards
}
