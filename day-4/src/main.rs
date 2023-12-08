use std::{fs, collections::HashSet};

type Card = (HashSet<u8>, HashSet<u8>);

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let cards = process_input(&input);
    let mut sum = 0;
    for (winning_numbers, numbers_gotten) in cards {
        let intersection = winning_numbers.intersection(&numbers_gotten);
        let size = &intersection.count();
        if *size > 0 {
            sum += 2u32.pow((*size - 1) as u32);
        }
    }
    println!("Sum: {sum}");
}

fn process_input(input: &str) -> Vec<Card> {
    let mut semi_colon_found = false;
    let mut pipe_found = false;
    let mut number_buffer = String::from("");
    let mut cards: Vec<Card> = vec![];
    let mut winning_numbers: HashSet<u8> = HashSet::new();
    let mut number_gotten: HashSet<u8> = HashSet::new();

    for char in input.chars() {
        if char == ':' { semi_colon_found = true }
        if !semi_colon_found {
            continue;
        }
        if char.is_digit(10) {
            number_buffer = format!("{number_buffer}{char}");
        } else if char == ' ' && !number_buffer.is_empty() {
            if pipe_found {
                number_gotten.insert(number_buffer.parse().unwrap());
            } else {
                winning_numbers.insert(number_buffer.parse().unwrap());
            }
            number_buffer.clear();
        } else if char == '\n' {
            number_gotten.insert(number_buffer.parse().unwrap());
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
