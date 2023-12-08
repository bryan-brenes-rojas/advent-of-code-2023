use std::{fs, collections::{HashMap, HashSet}};

type Matrix = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let matrix = generate_engine_matrix(&input);
    let mut symbol_map: HashMap<String, Vec<String>> = HashMap::new();
    for (row_index, row) in matrix.iter().enumerate() {
        let mut number_buffer = String::from("");
        let mut is_part = false;
        let mut symbol_indices_strings: HashSet<String> = HashSet::new();
        for (col_index, cell) in row.iter().enumerate() {
            if cell.is_digit(10) {
                number_buffer.push(*cell);
                let list = get_adjacent_symbols_indices(&matrix, row_index, col_index);
                if !list.is_empty() {
                    symbol_indices_strings.extend(list);
                    is_part = true;
                }
            } else if !number_buffer.is_empty() && is_part {
                for index_str in &symbol_indices_strings {
                    match symbol_map.get(index_str) {
                        Some(value) => { 
                            let mut copy = value.clone();
                            copy.extend_from_slice(&[number_buffer.clone()]);
                            symbol_map.insert(index_str.to_string(), copy);
                        },
                        None => {
                            symbol_map.insert(index_str.to_string(), vec![number_buffer.clone()]);
                        }
                    }
                }
                number_buffer.clear();
                is_part = false;
                symbol_indices_strings.clear();
            } else {
                number_buffer.clear();
                is_part = false;
                symbol_indices_strings.clear();
            }
        }

        if !number_buffer.is_empty() && is_part {
            for index_str in &symbol_indices_strings {
                match symbol_map.get(index_str) {
                    Some(value) => { 
                        let mut copy = value.clone();
                        copy.extend_from_slice(&[number_buffer.clone()]);
                        symbol_map.insert(index_str.to_string(), copy);
                    },
                    None => {
                        symbol_map.insert(index_str.to_string(), vec![number_buffer.clone()]);
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for (_, value) in symbol_map {
        if value.len() == 2 {
            sum += value[0].parse::<u32>().unwrap() * value[1].parse::<u32>().unwrap();
        }
    }
    println!("Sum: {sum}");
}

fn get_adjacent_symbols_indices(matrix: &Matrix, row: usize, col: usize) -> Vec<String> {
    let r_i = row as i32;
    let c_i = col as i32;
    let mut row_index = r_i - 1;
    let mut symbol_indices_strings: Vec<String> = vec![];
    while row_index <= r_i + 1 {
        let mut col_index = c_i - 1;
        while col_index <= c_i + 1 {
            if are_indices_valid(matrix, row_index, col_index) 
                && is_symbol(matrix[row_index as usize][col_index as usize]) {
                    symbol_indices_strings.push(format!("{}_{}", row_index, col_index));
            }
            col_index += 1;
        }
        row_index += 1;
    }
    symbol_indices_strings
}

fn is_symbol(char: char) -> bool {
    !char.is_digit(10) && char != '.'
}

fn are_indices_valid(matrix: &Matrix, row: i32, col: i32) -> bool {
    let row_count = matrix.len() as i32;
    let col_count = matrix.first().unwrap().len() as i32;
    row >= 0 && row < row_count && col >= 0 && col < col_count
}

fn generate_engine_matrix(input: &str) -> Matrix {
    let mut matrix = vec![];
    let mut row = vec![];
    for char in input.chars() {
        if char == '\n' {
            matrix.push(row);
            row = vec![];
        } else {
            row.push(char);
        }
    }
    matrix
}
