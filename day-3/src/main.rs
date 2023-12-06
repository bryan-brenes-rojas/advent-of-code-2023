use std::fs;

fn main() {
    let input = fs::read_to_string("assets/test-input.txt").unwrap();
    let matrix = generate_engine_matrix(&input);
    println!("{:?}", matrix);
}

fn generate_engine_matrix(input: &str) -> Vec<Vec<char>> {
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
