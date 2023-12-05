use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut line_index = 0;
    for line in lines {
        let mut char_index = 0;
        let mut collected_number = String::new();
        for char in line.chars() {
            if char.is_digit(10) {
                collected_number.push(char);
            } else {
                if collected_number.len() > 0 {
                    println!("collected_number: {}", collected_number);
                    collected_number.clear();
                }
            }

            char_index += 1;
        }

        line_index += 1;
    }

    println!("Part One: {}", '.'.is_digit(10));
}
