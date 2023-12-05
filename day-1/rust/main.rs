use std::fs;

#[derive(Debug)]
struct NumberMatch {
    name: String,
    num: u8,
}


fn main() {
    let number_names_table: [NumberMatch; 10] = [
        NumberMatch {name: String::from("zero"), num: 0},
        NumberMatch {name: String::from("one"), num: 1},
        NumberMatch {name: String::from("two"), num: 2},
        NumberMatch {name: String::from("three"), num: 3},
        NumberMatch {name: String::from("four"), num: 4},
        NumberMatch {name: String::from("five"), num: 5},
        NumberMatch {name: String::from("six"), num: 6},
        NumberMatch {name: String::from("seven"), num: 7},
        NumberMatch {name: String::from("eight"), num: 8},
        NumberMatch {name: String::from("nine"), num: 9},
    ];

    let input = fs::read_to_string("../input-file.txt").expect("Cannot open file");
    let lines = input.split('\n');
    let mut sum: u32 = 0;
    let mut left_number: u8 = 255;
    let mut right_number: u8 = 255;

    for line in lines {
        if line.len() == 0 { continue }

        for (i, char) in line.chars().enumerate() {
            let slice = &line[i..line.len()];
            for possible_match in &number_names_table {
                if left_number == 255 && slice.starts_with(&possible_match.name){
                    left_number = possible_match.num;
                }

                if left_number != 255 && slice.starts_with(&possible_match.name) {
                    right_number = possible_match.num;
                }
            }

            if left_number != 255 && char.is_digit(10) {
                right_number = char.to_digit(10).expect("Cannot convert to u8") as u8;
            }

            if left_number == 255 && char.is_digit(10) {
                left_number = char.to_digit(10).expect("Cannot convert to u8") as u8;
            }
        }

        sum += (left_number * 10) as u32;
        sum += (if right_number != 255 { right_number } else { left_number }) as u32;
        right_number = 255;
        left_number = 255;
    }

    println!("Code: {sum}");
}

