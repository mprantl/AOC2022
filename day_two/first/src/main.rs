use std::{fs::File, io::Read};

#[derive(Debug)]
struct Game {
    enemy_choice: RPS,
    player_choice: RPS,
    result: Result,
}
//Result enum
#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}
//Rock Paper Scissors enum
#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let games_vec = read_file_into_games_vec();
    let score = calculate_score(&games_vec);
    println!("Score: {:?}", score);
}
// Function to calculate score based on the games vector
fn calculate_score(games_vec: &Vec<Game>) -> u32 {
    let mut player_score = 0;
    for game in games_vec {
        match game.result {
            Result::Win => player_score += 6,
            Result::Lose => player_score += 0,
            Result::Draw => player_score += 3,
        }
    }
    for game in games_vec {
        match game.player_choice {
            RPS::Rock => player_score += 1,
            RPS::Paper => player_score += 2,
            RPS::Scissors => player_score += 3,
        }
    }
    player_score
}

// Function reads text file and returns a vector of Game structs
fn read_file_into_games_vec() -> Vec<Game> {
    let mut file = File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let mut games_vec: Vec<Game> = Vec::new();
    for line in contents.lines() {
        match line {
            "A X" => games_vec.push(Game {
                enemy_choice: RPS::Rock,
                player_choice: RPS::Rock,
                result: Result::Draw,
            }),
            "A Y" => games_vec.push(Game {
                enemy_choice: RPS::Rock,
                player_choice: RPS::Paper,
                result: Result::Win,
            }),
            "A Z" => games_vec.push(Game {
                enemy_choice: RPS::Rock,
                player_choice: RPS::Scissors,
                result: Result::Lose,
            }),
            "B X" => games_vec.push(Game {
                enemy_choice: RPS::Paper,
                player_choice: RPS::Rock,
                result: Result::Lose,
            }),
            "B Y" => games_vec.push(Game {
                enemy_choice: RPS::Paper,
                player_choice: RPS::Paper,
                result: Result::Draw,
            }),
            "B Z" => games_vec.push(Game {
                enemy_choice: RPS::Paper,
                player_choice: RPS::Scissors,
                result: Result::Win,
            }),
            "C X" => games_vec.push(Game {
                enemy_choice: RPS::Scissors,
                player_choice: RPS::Rock,
                result: Result::Win,
            }),
            "C Y" => games_vec.push(Game {
                enemy_choice: RPS::Scissors,
                player_choice: RPS::Paper,
                result: Result::Lose,
            }),
            "C Z" => games_vec.push(Game {
                enemy_choice: RPS::Scissors,
                player_choice: RPS::Scissors,
                result: Result::Draw,
            }),
            _ => println!("Error"),
        }
    }
    games_vec
}
