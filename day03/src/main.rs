use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let total_part_one = part_one(&lines);
    let total_part_two = part_two(&lines);

    println!("Part One: {}", total_part_one);
    println!("Part Two: {}", total_part_two);
}

fn part_two(lines: &Vec<&str>) -> i32 {
    let mut total = 0;

    for line_index in 0..lines.len() {
        for char_index in 0..lines[line_index].len() {
            let line = lines[line_index];
            let char: char = line.chars().nth(char_index).unwrap();

            if !check_gear(char) {
                continue;
            }

            let left_index: i32 = char_index as i32 - 1;
            let right_index: i32 = char_index as i32 + 2;
            let top_index: i32 = line_index as i32 - 1;
            let bottom_index: i32 = line_index as i32 + 1;

            let top_line = &lines[top_index as usize][left_index as usize..right_index as usize];
            let current_line = &lines[line_index][left_index as usize..right_index as usize];
            let bottom_line =
                &lines[bottom_index as usize][left_index as usize..right_index as usize];

            let full_string = [top_line, current_line, bottom_line].join("");

            let mut count = 0;
            let mut on_number = false;

            let mut first_index = 0;
            let mut second_index = 0;
            let full_string_chars: Vec<char> = full_string.chars().collect();

            for index in 0..full_string_chars.len() {
                let char = full_string_chars[index];

                if on_number && (index == 3 || index == 6) {
                    count += 1;
                    if count == 1 {
                        first_index = index - 1;
                    } else if count == 2 {
                        second_index = index - 1;
                    }
                    on_number = false;
                }

                if char.is_digit(10) {
                    on_number = true;

                    if index == full_string.len() - 1 {
                        count += 1;
                        second_index = index;
                    }
                } else {
                    if on_number {
                        count += 1;
                        on_number = false;

                        if count == 1 {
                            first_index = index - 1;
                        } else if count == 2 {
                            second_index = index - 1;
                        }
                    }
                }
            }

            if count < 2 {
                continue;
            }

            let mut first_line_index: usize = 0;
            let mut second_line_index: usize = 0;

            if first_index < 3 {
                first_line_index = top_index as usize;
            } else if first_index > 2 && first_index < 6 {
                first_line_index = line_index;
                first_index -= 3;
            } else {
                first_line_index = bottom_index as usize;
                first_index -= 6;
            }

            if second_index < 3 {
                second_line_index = top_index as usize;
            } else if second_index > 2 && second_index < 6 {
                second_line_index = line_index;
                second_index -= 3;
            } else {
                second_line_index = bottom_index as usize;
                second_index -= 6;
            }

            let number1 =
                find_number_on_line(&lines[first_line_index], char_index - 1 + first_index);

            let number2 =
                find_number_on_line(&lines[second_line_index], char_index - 1 + second_index);

            total += number1 * number2;
        }
    }

    return total;
}

fn find_number_on_line(line: &str, index: usize) -> i32 {
    let mut collected_number: String = String::new();
    let mut right_side: String = String::new();
    let mut left_side: String = String::new();

    for char_index in index..line.chars().count() {
        let char: char = line.chars().nth(char_index as usize).unwrap();
        if char.is_digit(10) {
            right_side.push(char);
        } else {
            break;
        }
    }

    for char_index in (0..index).rev() {
        let char: char = line.chars().nth(char_index as usize).unwrap();
        if char.is_digit(10) {
            left_side.push(char);
        } else {
            break;
        }
    }

    for char in left_side.chars().rev() {
        collected_number.push(char);
    }
    for char in right_side.chars() {
        collected_number.push(char);
    }

    return collected_number.parse::<i32>().unwrap();
}

fn check_gear(char: char) -> bool {
    return char == '*';
}

fn part_one(lines: &Vec<&str>) -> i32 {
    let mut line_index = 0;
    let mut total = 0;

    for line in lines {
        let mut collected_number = String::new();

        for char_index in 0..line.len() {
            let char: char = line.chars().nth(char_index).unwrap();
            let mut minus = collected_number.len() + 1;
            if char.is_digit(10) {
                collected_number.push(char);

                if char_index == line.len() - 1 {
                    minus = collected_number.len();
                }

                if char_index != line.len() - 1 {
                    continue;
                }
            }

            if collected_number.len() <= 0 {
                collected_number.clear();
                continue;
            }

            if char_index < minus {
                minus = char_index
            }

            let left_index = char_index - minus;
            let right_index = char_index;

            let mut left_char = line.chars().nth(left_index).unwrap();
            if char_index == collected_number.len() {
                left_char = '.';
            }

            let mut right_char = line.chars().nth(char_index).unwrap();
            if char_index == line.len() - 1 {
                right_char = '.';
            }

            let mut top_line_range: &str = "";
            let mut bottom_line_range: &str = "";

            if line_index != 0 {
                let topline_index = line_index - 1;
                top_line_range = &lines[topline_index][left_index..right_index + 1];
            }

            if line_index != lines.len() - 1 {
                let bottomline_index = line_index + 1;
                bottom_line_range = &lines[bottomline_index][left_index..right_index + 1];
            }

            let found = check_symbol(left_char)
                || check_symbol(right_char)
                || check_symbols_in_string(top_line_range)
                || check_symbols_in_string(bottom_line_range);

            let actual_number = collected_number.parse::<i32>().unwrap();
            if found {
                total += actual_number;
            }

            collected_number.clear();
        }

        line_index += 1;
    }

    return total;
}

fn check_symbols_in_string(string: &str) -> bool {
    for char in string.chars() {
        if check_symbol(char) {
            return true;
        }
    }
    return false;
}

fn check_symbol(symbol: char) -> bool {
    let thing = !symbol.is_digit(10) && symbol != '.';
    return thing;
}
