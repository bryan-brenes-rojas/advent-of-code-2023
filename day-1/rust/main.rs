use std::fs;
use std::time::SystemTime;

fn main() {
    let input = fs::read_to_string("../input-file.txt").expect("Cannot open file");
    let init_time = SystemTime::now();
    let mut sum: i32 = 0;
    let mut left_number: i32 = -1;
    let mut right_number: i32 = -1;

    for byte in input.bytes() {
        if byte != 10 {
            if left_number != -1 && is_number(byte) {
                right_number = (byte - 48) as i32;
            }

            if left_number == -1 && is_number(byte) {
                left_number = (byte - 48) as i32;
            }
        } else {
            sum += left_number * 10;
            sum += if right_number != -1 { right_number } else { left_number };
            right_number = -1;
            left_number = -1;
        }
    }

    println!("Code: {sum}");
    match init_time.elapsed() {
        Ok(elapsed) => println!("Time: {}", elapsed.as_secs_f32()),
        Err(e) => println!("Cannot get time elapsed {e}"),
    }
}

fn is_number(byte: u8) -> bool {
    return byte >= 48 && byte <= 57;
}
