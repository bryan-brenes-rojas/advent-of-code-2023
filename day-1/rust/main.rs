use std::fs;
use std::time::SystemTime;

fn main() {
    let input = fs::read_to_string("../input-file.txt").expect("Cannot open file");
    let init_time = SystemTime::now();
    let mut sum: i32 = 0;
    let mut left_number: u8 = 255;
    let mut right_number: u8 = 255;

    for byte in input.bytes() {
        if byte != 10 {
            if left_number != 255 && is_number(byte) {
                right_number = byte - 48;
            }

            if left_number == 255 && is_number(byte) {
                left_number = byte - 48;
            }
        } else {
            sum += (left_number * 10) as i32;
            sum += (if right_number != 255 { right_number } else { left_number }) as i32;
            right_number = 255;
            left_number = 255;
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
