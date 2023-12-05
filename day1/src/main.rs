use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut total: i32 = 0;
    // let mut last_digit;

    for line in lines {
        let line_total = find_combined_first_and_last_digits(&line);

        total += line_total;
    }

    println!("Part One: {}", total);
}

fn find_combined_first_and_last_digits_words(line: &str) -> i32 {
    let 


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
