use std::{fs, thread::park_timeout};

#[derive(Debug)]
struct Play {
    red: u8,
    green: u8,
    blue: u8,
}

impl Play {
    pub fn new() -> Play {
        Play { red: 0, green: 0, blue: 0 }
    }
}

#[derive(Debug)]
struct Game {
    id: u8,
    plays: Vec<Play>,
}

impl Game {
    pub fn new(id: u8, plays: Vec<Play>) -> Game {
        Game {id, plays}
    }
}

/**
 * Configuration: 
 * - 12 RED
 * - 13 GREEN
 * - 14 BLUE
 */
fn main() {
    let input = fs::read_to_string("assets/input-file.txt").expect("Cannot read input file");
    let lines = input.split('\n');
    for line in lines {
        if line.len() == 0 { continue }

        let game = get_game(line);
        println!("{:?}", game);
    }
}

fn get_game_id(game_info: &Vec<&str>) -> u8 {
    match game_info.first() {
        None => panic!("The game has no name"),
        Some(name) => match name.split(' ').collect::<Vec<&str>>().last() {
            None => panic!("The game has no id"),
            Some(id) => id.parse::<u8>().unwrap(),
        }
    }
}

fn get_game_plays(game_info: &Vec<&str>) -> Vec<Play> {
    let game_plays_strings = match game_info.last() {
        None => panic!("Game has no plays"),
        Some(plays_string) => plays_string.split(';'),
    };

    let mut plays = vec![];
    for play_string in game_plays_strings {
        let cubes = play_string.split(',');
        let mut play = Play::new();

        for cube in cubes {
            let cube_info = cube.trim().split(' ').collect::<Vec<&str>>();
            let amount = cube_info.first().unwrap().parse::<u8>().unwrap();
            let color = cube_info.last().unwrap();
            
            if *color == "RED" {
                play.red = amount;
            } else if *color == "GREEN" {
                play.green = amount;
            } else {
                play.blue = amount;
            }
        }
        plays.push(play);
    }
    plays
}

fn get_game(line: &str) -> Game {
    let game_info: Vec<&str> = line.split(':').collect();
    let game_id = get_game_id(&game_info);
    let plays = get_game_plays(&game_info);
    Game::new(game_id, plays)
}


