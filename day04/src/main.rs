use std::fs;
use std::io::Write;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let total_part_one = do_part1(&lines);
    let total_part_two = do_part2(&lines);

    println!("Part One: {}", total_part_one);
    println!("Part Two: {}", total_part_two);
}

fn do_part2(lines: &Vec<&str>) -> i32 {
    let cards = transform_to_arrays(&lines);
    let mut total_cards: Vec<Vec<Vec<i32>>> = Vec::new();
    println!("Starting part two");

    for (index, card) in cards.iter().enumerate() {
        let matches = get_number_matches(&card[1], &card[0]);

        total_cards.push(card.clone());

        get_next_cards_by_matches(&cards, &mut total_cards, index as u32 + 1, matches as u32);

        let percentage = index as f64 / cards.len() as f64 * 100.0;
        print!("{:.2}% -- ", percentage);
        std::io::stdout().flush().unwrap();
    }

    println!("Finished, now calculating");

    return total_cards.len() as i32;
}

fn get_next_cards_by_matches(
    cards: &Vec<Vec<Vec<i32>>>,
    total_cards: &mut Vec<Vec<Vec<i32>>>,
    current_index: u32,
    matches: u32,
) {
    let top_index = current_index + matches;
    for index in current_index..top_index {
        let matches = get_number_matches(&cards[index as usize][1], &cards[index as usize][0]);

        total_cards.push(cards[index as usize].clone());

        if matches == 0 {
            continue;
        }

        get_next_cards_by_matches(&cards, total_cards, index as u32 + 1, matches as u32);
    }
}

fn do_part1(lines: &Vec<&str>) -> i32 {
    let mut total: i32 = 0;

    let cards = transform_to_arrays(&lines);

    for card in cards {
        let matches = get_number_matches(&card[1], &card[0]);
        let score = calculate_score(matches);
        total += score;
    }

    return total;
}

fn transform_to_arrays(lines: &Vec<&str>) -> Vec<Vec<Vec<i32>>> {
    let mut result: Vec<Vec<Vec<i32>>> = Vec::new();

    for line in lines {
        let actual_line = line.split(":").collect::<Vec<&str>>()[1];

        let numbers = split_number_types(actual_line);
        let winning_numbers = &numbers[0];
        let given_numbers = &numbers[1];

        result.push(vec![(&winning_numbers).to_vec(), (&given_numbers).to_vec()]);
    }

    return result;
}

fn split_number_types(line: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let winning_line = line.split("|").collect::<Vec<&str>>()[0];
    let i_have_line = line.split("|").collect::<Vec<&str>>()[1];
    let winning_numbers = parse_numbers(winning_line);
    let given_numbers = parse_numbers(i_have_line);

    result.push(winning_numbers);
    result.push(given_numbers);

    return result;
}

fn get_number_matches(given_numbers: &Vec<i32>, winning_numbers: &Vec<i32>) -> u32 {
    let mut count: u32 = 0;
    for given_number in given_numbers {
        if winning_numbers.contains(&given_number) {
            count += 1;
        }
    }

    return count;
}

fn calculate_score(matches: u32) -> i32 {
    if matches == 0 {
        return 0;
    } else if matches == 1 {
        return 1;
    }

    2_i32.pow(matches - 1)
}

fn parse_numbers(numbers_string: &str) -> Vec<i32> {
    let mut results_array: Vec<i32> = Vec::new();
    let numbers = numbers_string.trim().split(" ").collect::<Vec<&str>>();
    for number in numbers {
        if number == "" {
            continue;
        }
        let number_as_int = number.parse::<i32>().unwrap();
        results_array.push(number_as_int);
    }
    return results_array;
}
