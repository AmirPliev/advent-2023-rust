use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut total_part_one: i32 = 0;
    let mut total_part_two = 0;

    for line in lines {
        let part_one_line_total = find_combined_first_and_last_digits(&line);
        let part_two_line_total = find_combined_first_and_last_digits_words(&line);

        total_part_one += part_one_line_total;
        total_part_two += part_two_line_total
    }

    println!("Part One: {}", total_part_one);
    println!("Part Two: {}", total_part_two);
}

fn find_combined_first_and_last_digits_words(line: &str) -> i32 {
    // Dunno this doesn't work for some reason...
    let mut digits_full: HashMap<&str, char> = HashMap::new();
    digits_full.insert("zero", '0');
    digits_full.insert("one", '1');
    digits_full.insert("two", '2');
    digits_full.insert("three", '3');
    digits_full.insert("four", '4');
    digits_full.insert("five", '5');
    digits_full.insert("six", '6');
    digits_full.insert("seven", '7');
    digits_full.insert("eight", '8');
    digits_full.insert("nine", '9');

    let digits: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut first_digit: char = '0';
    let mut last_digit: char = '0';

    let mut largest_index: i32 = -1;
    let mut smallest_index: i32 = line.len() as i32;
    smallest_index += 1;

    for digit in digits {
        if !line.contains(digit) {
            continue;
        }

        let mut index_digit: i32 = 0;

        match line.find(digit) {
            None => println!("None"),
            Some(index) => index_digit = index as i32,
        }

        if index_digit <= smallest_index {
            smallest_index = index_digit;
            first_digit = digit;
        }

        if index_digit >= largest_index {
            largest_index = index_digit;
            last_digit = digit;
        }
    }

    for (digit, &digit_as_char) in &digits_full {
        if !line.contains(digit) {
            continue;
        }

        let mut index_digit: i32 = 0;

        match line.find(digit) {
            None => println!("None"),
            Some(index) => index_digit = index as i32,
        }

        if index_digit <= smallest_index {
            smallest_index = index_digit;
            first_digit = digit_as_char;
        }

        if index_digit >= largest_index {
            largest_index = index_digit;
            last_digit = digit_as_char;
        }
    }

    let combined_numbers = format!("{}{}", first_digit, last_digit);
    let line_total = combined_numbers.parse::<i32>().unwrap();
    println!("{} -- digit {} test {}", line, combined_numbers, line_total);

    return line_total;
}

fn find_combined_first_and_last_digits(line: &str) -> i32 {
    let mut first_digit: char = 'a';
    let mut first_found: bool = false;
    let mut last_digit: char = 'a';

    for char in line.chars() {
        if char.is_digit(10) {
            if !first_found {
                first_digit = char;
                last_digit = char;
                first_found = true;
            } else {
                last_digit = char;
            }
        }
    }

    let combined_numbers = format!("{}{}", first_digit, last_digit);
    let line_total = combined_numbers.parse::<i32>().unwrap();

    return line_total;
}
