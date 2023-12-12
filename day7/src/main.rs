use std::fs;

fn main() {
    let file_path = "test_input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let games = get_games(&lines);

    

    println!("{:?}", result);
}

fn get_games<'a>(lines: &Vec<&'a str>) -> Vec<(&'a str, u32)> {
    let mut result: Vec<(&str, u32)> = Vec::new();
    for line in lines {
        let thing: Vec<&str> = line.split(" ").collect();
        let card = thing[0];
        let bet = thing[1].parse::<u32>().unwrap();
        result.push((card, bet))
    }

    return result;
}
