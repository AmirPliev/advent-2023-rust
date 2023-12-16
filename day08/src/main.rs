use num::integer::lcm;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let pattern = lines[0].chars().collect::<Vec<char>>();
    let mut directions = lines.clone();
    directions.remove(0);
    directions.remove(0);
    let mapping = create_directions_map(directions);

    let part_one_moves = part_one(&pattern, &mapping);
    let part_two_moves = part_two(&pattern, &mapping);

    println!("Part One: {part_one_moves}");
    println!("Part Two: {part_two_moves}");
}

fn part_two(pattern: &Vec<char>, directions: &HashMap<String, (String, String)>) -> u64 {
    let current_positions = get_all_keys_ending_in('A', &directions);
    let mut moves_necessary: Vec<u64> = Vec::new();

    for position in current_positions {
        let mut moves = 0;
        let mut pattern_pos = 0;
        let mut current_position = position.clone();

        while !current_position.ends_with("Z") {
            if pattern_pos == pattern.len() {
                pattern_pos -= pattern.len();
            }

            if pattern[pattern_pos] == 'R' {
                current_position = directions.get(&current_position).unwrap().1.clone();
            } else {
                current_position = directions.get(&current_position).unwrap().0.clone();
            }

            moves += 1;
            pattern_pos += 1;
        }

        moves_necessary.push(moves);
    }

    let mut current_lcm = 1;
    for this_move in &moves_necessary {
        current_lcm = lcm(current_lcm, this_move.clone());
    }

    current_lcm
}

fn get_all_keys_ending_in(
    character: char,
    directions: &HashMap<String, (String, String)>,
) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();

    for (key, _value) in directions.into_iter() {
        let last_index = key.chars().collect::<Vec<char>>().len() - 1;
        let last_char = key.chars().nth(last_index).unwrap();

        if last_char == character {
            keys.push(key.to_string());
        }
    }

    return keys;
}

fn part_one(pattern: &Vec<char>, directions: &HashMap<String, (String, String)>) -> u64 {
    let mut current_position = "AAA";
    let mut moves = 0;
    let mut pattern_pos = 0;

    while current_position != "ZZZ" {
        if pattern_pos == pattern.len() {
            pattern_pos -= pattern.len();
        }

        if pattern[pattern_pos] == 'R' {
            current_position = &directions[current_position].1;
        } else {
            current_position = &directions[current_position].0;
        }

        moves += 1;
        pattern_pos += 1;
    }
    return moves;
}

fn create_directions_map(lines: Vec<&str>) -> HashMap<String, (String, String)> {
    let mut final_mapping: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let splitted = line.split(" = ").collect::<Vec<&str>>();

        let source = splitted[0];
        let destinations_strings = splitted[1].split(", ").collect::<Vec<&str>>();
        let destination = (
            destinations_strings[0].replace("(", ""),
            destinations_strings[1].replace(")", ""),
        );

        final_mapping.insert(source.to_string(), destination);
    }

    return final_mapping;
}
