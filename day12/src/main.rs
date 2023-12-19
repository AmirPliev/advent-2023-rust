use std::fs;

fn main() {
    let file_path: &str = "test_input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let result = separate_conditions(lines);
    println!("{:?}", result);
}

fn separate_conditions(lines: Vec<&str>) -> Vec<(Vec<char>, Vec<u32>)> {
    let mut result: Vec<(Vec<char>, Vec<u32>)> = Vec::new();

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();

        let characters = split[0].chars().collect::<Vec<char>>();
        let numbers: Vec<u32> = split[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        result.push((characters, numbers));
    }

    result
}
