use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut total_part_one = 0;
    let mut total_part_two = 0;

    for line in lines {
        total_part_one += predict_next(line);
        total_part_two += predict_previous(line);
    }

    println!("Part One: {total_part_one}");
    println!("Part Two: {total_part_two}");
}

fn predict_previous(line: &str) -> i32 {
    let starting_values: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut list_of_differences: Vec<Vec<i32>> = Vec::new();
    let mut current_differences = calculate_differences(&starting_values);
    list_of_differences.push(starting_values.clone());
    list_of_differences.push(current_differences.clone());

    loop {
        current_differences = calculate_differences(&current_differences);

        if finished(&current_differences) {
            break;
        }

        list_of_differences.push(current_differences.clone());
    }

    list_of_differences.reverse();

    let mut first_element = 0;

    for difference in &list_of_differences {
        first_element = difference[0] - first_element;
    }

    return first_element;
}

fn predict_next(line: &str) -> i32 {
    let starting_values: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut list_of_differences: Vec<Vec<i32>> = Vec::new();
    let mut current_differences = calculate_differences(&starting_values);
    list_of_differences.push(starting_values.clone());
    list_of_differences.push(current_differences.clone());

    loop {
        current_differences = calculate_differences(&current_differences);

        if finished(&current_differences) {
            break;
        }

        list_of_differences.push(current_differences.clone());
    }

    list_of_differences.reverse();

    let mut last_element = 0;

    for difference in &list_of_differences {
        let last_element_index = difference.len() - 1;
        let new_last_element = difference[last_element_index];
        last_element += new_last_element;

        // println!("{:?} -- {last_element}", difference);
    }

    return last_element;
}

fn finished(numbers: &Vec<i32>) -> bool {
    for number in numbers {
        if *number != 0 {
            return false;
        }
    }

    true
}

fn calculate_differences(numbers: &Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = Vec::new();

    let first_difference = numbers[1] - numbers[0];
    differences.push(first_difference);

    for index in 2..numbers.len() {
        let new_difference = numbers[index] - numbers[index - 1];
        differences.push(new_difference);
    }

    return differences;
}
