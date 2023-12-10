use std::{fs, collections::HashMap};

/**
 * Format: destination_range_start source_range_start range_length (a b n)
 * source_min => b
 * source_max => b + n - 1 (inclusive)
 * offset => a - b
 * destination = source + offset
 */

type ProcessMap = HashMap<u8, Vec<RangeData>>;

#[derive(Debug)]
struct RangeData {
    min: u32,
    max: u32,
    offset: i32,
}

impl RangeData {
    pub fn new(destination_start: u32, source_start: u32, range_length: u32) -> Self {
        RangeData {
            min: source_start,
            max: source_start + range_length - 1,
            offset: destination_start as i32 - source_start as i32,
        }
    }

    // pub fn destination(self, source: u32) -> u32 {
        // (source as i32 + self.offset) as u32
     // }
}

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let (seeds, process_map) = process_input(&input);
    let mut min_location = u32::MAX;
    for seed in seeds {
        let mut source = seed;
        for process_index in 0..7 {
            source = get_destination(&process_map, process_index, source);
        }
        if source < min_location { min_location = source; }
    }

    println!("{:?}", min_location);
}

fn process_input(input: &str) -> (Vec<u32>, ProcessMap) {
    let mut process_map: HashMap<u8, Vec<RangeData>> = HashMap::new();
    let mut current_index: i8 = -1;
    let mut seeds: Vec<u32> = vec![];

    for line in input.lines() {
        if line == "" { 
            current_index += 1;
            process_map.insert(current_index as u8, vec![]);
            continue; 
        }

        if current_index == -1 {
            seeds = get_numbers_from_line(line);
        } else {
            if line.chars().last().unwrap() == ':' { continue; }
            let numbers = get_numbers_from_line(line);
            let range_config = RangeData::new(numbers[0], numbers[1], numbers[2]);
            let range_vector = process_map.get_mut(&(current_index as u8)).unwrap();
            range_vector.push(range_config);
        }
    }

    (seeds, process_map)
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

fn get_destination(process_map: &ProcessMap, process_index: u8, source: u32) -> u32 {
    let ranges = process_map.get(&process_index).unwrap();
    for range in ranges {
        if  source >= range.min && source <= range.max {
            // TODO: use destination fn for this
            return (source as i32 + range.offset) as u32;
        }
    }
    return source;
}
