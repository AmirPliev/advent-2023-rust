use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let part_one = do_part_one(&lines);
    let part_two = do_part_two(&lines);

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}

fn do_part_two(lines: &Vec<&str>) -> u64 {
    let races: Vec<u64> = get_races(&lines);
    let record_distances: Vec<u64> = get_record_distances(&lines);
    let race = to_single_race(&races, &record_distances);

    let mut final_answer = 0;

    let milliseconds_total = race[0];
    let record_to_beat = race[1];

    for press_time in 0..milliseconds_total {
        let remaining_time = milliseconds_total - press_time;
        let distance = remaining_time * press_time;

        if distance > record_to_beat {
            final_answer += 1;
        }
    }

    return final_answer;
}

fn to_single_race(races: &Vec<u64>, records: &Vec<u64>) -> Vec<u64> {
    let time = races
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let record = records
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    return vec![time, record];
}

fn do_part_one(lines: &Vec<&str>) -> u64 {
    let races: Vec<u64> = get_races(&lines);
    let possible_races: Vec<Vec<u64>> = calculate_possible_distances(&races);

    let record_distances: Vec<u64> = get_record_distances(&lines);

    let winning_distances = calculate_winning_distances(&record_distances, &possible_races);
    let result = calculate_final_answer(winning_distances);
    return result;
}

fn calculate_final_answer(records: Vec<u64>) -> u64 {
    let mut result = 0;

    for record in records {
        if result == 0 {
            result = 1;
        }

        result = result * record;
    }

    return result;
}

fn calculate_winning_distances(
    record_distances: &Vec<u64>,
    possible_races: &Vec<Vec<u64>>,
) -> Vec<u64> {
    let mut result = Vec::new();

    for (index, record) in record_distances.iter().enumerate() {
        let possible_races_for_this_race: &Vec<u64> = &possible_races[index];
        let mut beating_races: Vec<u64> = Vec::new();

        for possible_race in possible_races_for_this_race {
            if possible_race > record {
                beating_races.push(possible_race.clone());
            }
        }

        result.push(beating_races.len() as u64);
    }

    return result;
}

fn get_record_distances(lines: &Vec<&str>) -> Vec<u64> {
    let distances: Vec<&str> = lines[1].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect();

    let mut filtered_distances = distances.to_vec(); // Create a new vector
    filtered_distances.retain(|&x| x != ""); // Filter out empty strings

    let final_distances: Vec<u64> = filtered_distances
        .iter()
        .map(|&s| s.parse().expect("Failed to parse to u64"))
        .collect();

    return final_distances;
}

fn calculate_possible_distances(races: &Vec<u64>) -> Vec<Vec<u64>> {
    let mut possible_results: Vec<Vec<u64>> = Vec::new();

    for race in races {
        let mut new_race_possibilities: Vec<u64> = Vec::new();
        for press_time in 0..*race {
            let resulting_time = race - press_time;
            new_race_possibilities.push(resulting_time * press_time);
        }

        possible_results.push(new_race_possibilities);
    }

    return possible_results;
}

fn get_races(lines: &Vec<&str>) -> Vec<u64> {
    let races: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect();

    let mut filtered_races = races.to_vec(); // Create a new vector
    filtered_races.retain(|&x| x != ""); // Filter out empty strings

    let final_races: Vec<u64> = filtered_races
        .iter()
        .map(|&s| s.parse().expect("Failed to parse to u64"))
        .collect();

    return final_races;
}
