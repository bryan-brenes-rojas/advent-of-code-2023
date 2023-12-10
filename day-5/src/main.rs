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
    min: u64,
    max: u64,
    offset: i64,
}

impl RangeData {
    pub fn new(destination_start: u64, source_start: u64, range_length: u64) -> Self {
        RangeData {
            min: source_start,
            max: source_start + range_length - 1,
            offset: destination_start as i64 - source_start as i64,
        }
    }

    // pub fn destination(self, source: u64) -> u64 {
        // (source as i64 + self.offset) as u64
     // }
}

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let (seed_ranges, process_map) = process_input(&input);
    let mut min_location = u64::MAX;
    for (min, max) in seed_ranges {
        for seed in min..max {
            let mut source = seed;
            for process_index in 0..7 {
                source = get_destination(&process_map, process_index, source);
            }
            if source < min_location { min_location = source; }
        }
    }

    println!("{:?}", min_location);
}

fn process_input(input: &str) -> (Vec<(u64, u64)>, ProcessMap) {
    let mut process_map: HashMap<u8, Vec<RangeData>> = HashMap::new();
    let mut current_index: i8 = -1;
    let mut seeds: Vec<(u64, u64)> = vec![];

    for line in input.lines() {
        if line == "" { 
            current_index += 1;
            process_map.insert(current_index as u8, vec![]);
            continue; 
        }

        if current_index == -1 {
            let seeds_ranges = get_numbers_from_line(line);
            let mut index = 0;
            while index < seeds_ranges.len() {
                seeds.push((seeds_ranges[index], seeds_ranges[index] + seeds_ranges[index + 1]));
                index += 2;
            }
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

fn get_numbers_from_line(line: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = vec![];
    let mut num_buffer = String::from("");
    for char in line.chars() {
        if char.is_digit(10) {
            num_buffer += &char.to_string();
        } else if !num_buffer.is_empty() && char == ' ' {
            numbers.push(num_buffer.parse::<u64>().unwrap());
            num_buffer.clear();
        }
    }
    numbers.push(num_buffer.parse::<u64>().unwrap());
    numbers
}

fn get_destination(process_map: &ProcessMap, process_index: u8, source: u64) -> u64 {
    let ranges = process_map.get(&process_index).unwrap();
    for range in ranges {
        if  source >= range.min && source <= range.max {
            // TODO: use destination fn for this
            return (source as i64 + range.offset) as u64;
        }
    }
    return source;
}
