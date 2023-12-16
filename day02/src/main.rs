use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut total_part_one: i32 = 0;
    let mut total_part_two: i32 = 0;
    for line in &lines {
        let game_id = get_game_id(&line);

        let games_played = get_games(&line);
        let mut game_is_possible = true;
        for game in &games_played {
            if !check_game_possible(&game) {
                game_is_possible = false;
                break;
            }
        }

        let minimum_dice_needed = find_minimum_dice_needed(games_played);
        let power = calculate_power(minimum_dice_needed);

        total_part_two += power;

        if game_is_possible {
            total_part_one += game_id;
        }
    }

    println!("Part One: {}", total_part_one);
    println!("Part Two: {}", total_part_two);
}

fn calculate_power(game: HashMap<&str, i32>) -> i32 {
    let mut power = 1;

    for (_dice_type, dice_amount) in game {
        power *= dice_amount;
    }

    power
}

fn find_minimum_dice_needed(games: Vec<HashMap<&str, i32>>) -> HashMap<&str, i32> {
    let mut minimum_dice_needed: HashMap<&str, i32> = HashMap::new();
    minimum_dice_needed.insert("red", 0);
    minimum_dice_needed.insert("green", 0);
    minimum_dice_needed.insert("blue", 0);

    for game in games {
        for (dice_type, dice_amount) in game {
            if dice_amount > minimum_dice_needed[dice_type] {
                minimum_dice_needed.insert(dice_type, dice_amount);
            }
        }
    }

    minimum_dice_needed
}

fn check_game_possible(game: &HashMap<&str, i32>) -> bool {
    let mut part_one = HashMap::new();
    part_one.insert("red", 12);
    part_one.insert("green", 13);
    part_one.insert("blue", 14);

    for (dice_type, dice_amount) in game {
        if dice_amount > &part_one[dice_type] {
            return false;
        }
    }

    true
}

fn get_games(line: &str) -> Vec<HashMap<&str, i32>> {
    let game = line.split(":").collect::<Vec<&str>>();
    let indvidual_games = game[1].split(";");

    let mut games = Vec::new();

    for game in indvidual_games {
        let game_result = to_game_result(&game);
        games.push(game_result);
    }

    games
}

fn to_game_result(game: &str) -> HashMap<&str, i32> {
    let mut game_result: HashMap<&str, i32> = HashMap::new();

    let each_dice = game.split(",").collect::<Vec<&str>>();

    for dice in each_dice {
        let result = dice.split(" ").collect::<Vec<&str>>();
        let dice_amount = result[1].parse::<i32>().unwrap();
        let dice_type = result[2];

        game_result.insert(&dice_type, dice_amount);
    }

    game_result
}

fn get_game_id(line: &str) -> i32 {
    let game_id = line
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(0)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    game_id
}
