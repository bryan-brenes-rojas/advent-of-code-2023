use std::fs;

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let races = process_input(&input);
    let mut res = 1;
    for (time, distance) in races {
        res *= found_count_of_ways_to_beat(time, distance);
    }
    println!("Result: {:?}", res);
}

fn process_input(input: &str) -> Vec<(u32, u32)> {
    let lines: Vec<&str> = input.lines().collect();
    let times = get_numbers_from_line(lines[0]);
    let distances = get_numbers_from_line(lines[1]);
    let mut races = vec![];
    for i in 0..times.len() {
        races.push((times[i], distances[i]));
    }
    races
}

fn get_numbers_from_line(line: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = vec![];
    let mut num_buffer = String::from("");
    for char in line.chars() {
        if char.is_digit(10) {
            num_buffer += &char.to_string();
        } else if !num_buffer.is_empty() && char == ' ' {
            numbers.push(num_buffer.parse::<u32>().unwrap());
            num_buffer.clear();
        }
    }
    numbers.push(num_buffer.parse::<u32>().unwrap());
    numbers
}

fn found_count_of_ways_to_beat(time: u32, distance: u32) -> u32 {
    let discriminant = (time * time - 4 * distance) as f32;
    let x1: f32 = 0.5 * (time as f32 - discriminant.sqrt());
    let x2: f32 = 0.5 * (time as f32 + discriminant.sqrt());
    let mut result = (x2.ceil() - x1.ceil()) as u32;
    if x1.fract() == 0.0 { result -= 1 }
    result
}
