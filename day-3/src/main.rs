use std::fs;

type Matrix = Vec<Vec<char>>;

// 534001 is too low

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let matrix = generate_engine_matrix(&input);
    let mut sum = 0;
    for (row_index, row) in matrix.iter().enumerate() {
        let mut number_buffer = String::from("");
        let mut is_part = false;
        for (col_index, cell) in row.iter().enumerate() {
            if cell.is_digit(10) {
                number_buffer.push(*cell);
                if has_adjacent_symbol(&matrix, row_index, col_index) {
                    is_part = true;
                }
            } else if !number_buffer.is_empty() && is_part {
                sum += number_buffer.parse::<u32>().unwrap();
                number_buffer.clear();
                is_part = false;
            } else {
                number_buffer.clear();
                is_part = false;
            }
        }

        if !number_buffer.is_empty() && is_part {
            sum += number_buffer.parse::<u32>().unwrap();
        }
    }

    println!("Sum: {sum}");
}

fn has_adjacent_symbol(matrix: &Matrix, row: usize, col: usize) -> bool {
    let r_i = row as i32;
    let c_i = col as i32;
    let mut row_index = r_i - 1;
    while row_index <= r_i + 1 {
        let mut col_index = c_i - 1;
        while col_index <= c_i + 1 {
            if are_indices_valid(matrix, row_index, col_index) 
                && is_symbol(matrix[row_index as usize][col_index as usize]) {
                    return true;
            }
            col_index += 1;
        }
        row_index += 1;
    }
    false
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
