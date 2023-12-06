use std::fs;

fn main() {
    let input = fs::read_to_string("assets/test-input.txt").unwrap();
    let matrix = generate_engine_matrix(&input);
    println!("{:?}", matrix);
}

fn generate_engine_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix = vec![];
    let lines = input.split('\n');
    for line in lines {
        if line.len() == 0 { continue }
        let mut row = vec![];
        for char in line.chars() {
            row.push(char);
        }
        matrix.push(row);
    }

    matrix
}
