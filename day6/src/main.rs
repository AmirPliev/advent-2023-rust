use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let races: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .retain(|&x| x % 2 == 0);

    println!("{:?}", races);
}
