use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref GLOBAL_HASHMAP: HashMap<char, i32> = {
        let mut map = HashMap::new();
        // Add initial key-value pairs
        map.insert('j', 0);
        map.insert('2', 1);
        map.insert('3', 2);
        map.insert('4', 3);
        map.insert('5', 4);
        map.insert('6', 5);
        map.insert('7', 6);
        map.insert('8', 7);
        map.insert('9', 8);
        map.insert('T', 9);
        map.insert('J', 10);
        map.insert('Q', 11);
        map.insert('K', 12);
        map.insert('A', 13);
        map
    };
}

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let games: Vec<(&str, u32)> = get_games(&lines);

    let p1_ranked_games = do_logic(&games, false);
    let p2_ranked_games = do_logic(&games, true);

    let total_part_one = calculate_score(p1_ranked_games);
    let total_part_two = calculate_score(p2_ranked_games);

    println!("Part One: {total_part_one}");
    println!("Part One: {total_part_two}");
}

fn do_logic<'a>(games: &Vec<(&'a str, u32)>, part_two: bool) -> Vec<(&'a str, u32)> {
    let mut ranged_games: Vec<(&str, u32)> = Vec::new();

    for new_game in games {
        // First game, just push
        if ranged_games.len() == 0 {
            ranged_games.push(*new_game);
            continue;
        }

        let mut insert_index = 0;
        let mut insert_game: Option<(&str, u32)> = None;

        // Check if the next game is better than any of the previous played games.
        for (index, previous_game) in ranged_games.iter().enumerate() {
            let mut is_better = false;

            if !part_two {
                is_better = compare_p1_game_to(&new_game, previous_game);
            } else {
                is_better = compare_p2_game_to(&new_game, previous_game);
            }

            if is_better {
                insert_index = index;
                insert_game = Some(*new_game);
                break;
            }
        }

        // If there is a better game, place it in the correct position.
        if let Some(game) = insert_game {
            ranged_games = insert_in_place(ranged_games, game, insert_index);
        } else {
            ranged_games.push(*new_game);
        }
    }

    ranged_games.reverse();
    return ranged_games;
}

fn compare_p2_game_to<'a>(game1: &(&'a str, u32), game2: &(&'a str, u32)) -> bool {
    let result_game1 = p2_check_type(*game1);
    let result_game2 = p2_check_type(*game2);

    // Found type is better
    if result_game1 > result_game2 {
        return true;
    }

    // No type, check highest first card
    if result_game1 == result_game2 {
        for index in 0..game1.0.chars().count() {
            let mut game1_char = game1.0.chars().nth(index).unwrap();
            let mut game2_char = game2.0.chars().nth(index).unwrap();

            if game1_char == 'J' {
                game1_char = 'j'
            }

            if game2_char == 'J' {
                game2_char = 'j'
            }

            let game1_char_rank = GLOBAL_HASHMAP.get(&game1_char).unwrap();
            let game2_char_rank = GLOBAL_HASHMAP.get(&game2_char).unwrap();

            if game1_char_rank != game2_char_rank {
                return game1_char_rank > game2_char_rank;
            }
        }
    }

    return false;
}

fn p2_check_type(game: (&str, u32)) -> u32 {
    let mut result: Vec<u32> = Vec::new();
    let mut characters_found: Vec<char> = Vec::new();

    for character in game.0.chars() {
        if characters_found.contains(&character) {
            let index = characters_found
                .iter()
                .position(|&r| r == character)
                .unwrap();
            result[index] += 1;
            continue;
        }

        result.push(1);
        characters_found.push(character);
    }

    if result == [5] {
        return 7;
    }

    if characters_found.contains(&'J') {
        let new_result = find_better_solution(&characters_found, &result).to_vec();
        result = new_result;
    }
    result.sort();
    result.reverse();

    return calculate_hand(result);
}

fn find_better_solution(characters: &Vec<char>, counts: &Vec<u32>) -> Vec<u32> {
    let mut remaining_chars: Vec<char> = Vec::new();
    let mut remaining_counts: Vec<u32> = Vec::new();

    let j_index = characters.iter().position(|&r| r == 'J').unwrap();
    let j_count = counts[j_index];

    let mut best_result = counts.clone();
    let mut best_result_score = calculate_hand(counts.to_vec());

    for (index, character) in characters.iter().enumerate() {
        if character == &'J' {
            continue;
        }
        remaining_chars.push(*character);
        remaining_counts.push(counts[index]);
    }

    for index in 0..remaining_chars.len() {
        let mut new_result = remaining_counts.clone();
        new_result[index] += j_count;

        let mut check_hand_result = new_result.clone();
        check_hand_result.sort();
        check_hand_result.reverse();

        let new_score = calculate_hand(check_hand_result);
        if new_score > best_result_score {
            best_result_score = new_score;
            best_result = new_result.clone();
        }
    }

    best_result
}

fn compare_p1_game_to<'a>(game1: &(&'a str, u32), game2: &(&'a str, u32)) -> bool {
    let result_game1 = check_type(*game1);
    let result_game2 = check_type(*game2);

    // Found type is better
    if result_game1 > result_game2 {
        return true;
    }

    // No type, check highest first card
    if result_game1 == result_game2 {
        for index in 0..game1.0.chars().count() {
            let game1_char = game1.0.chars().nth(index).unwrap();
            let game2_char = game2.0.chars().nth(index).unwrap();

            let game1_char_rank = GLOBAL_HASHMAP.get(&game1_char).unwrap();
            let game2_char_rank = GLOBAL_HASHMAP.get(&game2_char).unwrap();

            if game1_char_rank != game2_char_rank {
                return game1_char_rank > game2_char_rank;
            }
        }
    }

    return false;
}

fn check_type(game: (&str, u32)) -> u32 {
    let mut result: Vec<u32> = Vec::new();
    let mut characters_found: Vec<char> = Vec::new();

    for character in game.0.chars() {
        if characters_found.contains(&character) {
            let index = characters_found
                .iter()
                .position(|&r| r == character)
                .unwrap();
            result[index] += 1;
            continue;
        }

        result.push(1);
        characters_found.push(character);
    }

    result.sort();
    result.reverse();

    return calculate_hand(result);
}

fn insert_in_place<'a>(
    array: Vec<(&'a str, u32)>,
    value: (&'a str, u32),
    index: usize,
) -> Vec<(&'a str, u32)> {
    let mut new_array: Vec<(&str, u32)> = Vec::new();

    for (old_index, element) in array.iter().enumerate() {
        if old_index == index {
            new_array.push(value)
        }

        new_array.push(*element);
    }

    return new_array;
}

fn get_games<'a>(lines: &Vec<&'a str>) -> Vec<(&'a str, u32)> {
    let mut result: Vec<(&str, u32)> = Vec::new();
    for line in lines {
        let thing: Vec<&str> = line.split(" ").collect();
        let card = thing[0];
        let bet = thing[1].parse::<u32>().unwrap();
        result.push((card, bet))
    }

    return result;
}

fn calculate_score(p1_ranked_games: Vec<(&str, u32)>) -> u32 {
    let mut total = 0;
    for (index, game) in p1_ranked_games.iter().enumerate() {
        let rank = index + 1;
        let score = game.1 * rank as u32;
        total += score;
    }

    return total;
}

fn calculate_hand(counts: Vec<u32>) -> u32 {
    if counts == [5] {
        return 7;
    } else if counts == [4, 1] {
        return 6;
    } else if counts == [3, 2] {
        return 5;
    } else if counts == [3, 1, 1] {
        return 4;
    } else if counts == [2, 2, 1] {
        return 3;
    } else if counts == [2, 1, 1, 1] {
        return 2;
    } else if counts == [1, 1, 1, 1, 1] {
        return 1;
    }

    return 0;
}
