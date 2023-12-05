use std::fs;

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
    pub fn new(line: &str) -> Game {
        let game_info: Vec<&str> = line.split(':').collect();
        let id = get_game_id(&game_info);
        let plays = get_game_plays(&game_info);
        Game {id, plays}
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

fn main() {
    let input = fs::read_to_string("assets/input-file.txt").unwrap();
    let lines = input.split('\n');
    let mut games = vec![];

    for line in lines {
        if line.len() == 0 { continue }
        games.push(Game::new(line));
    }

    let mut sum: u32 = 0;
    for game in games {
        let mut max_play = Play { red: 1, green: 1, blue: 1 };

        for play in game.plays {
            if play.red > max_play.red { max_play.red = play.red }
            if play.green > max_play.green { max_play.green = play.green }
            if play.blue > max_play.blue { max_play.blue = play.blue }
        }
        sum += max_play.red as u32 * max_play.green as u32 * max_play.blue as u32;
    }

    println!("Sum: {sum}");
}
