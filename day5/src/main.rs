use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    println!("Start");
    let seeds = get_defined_seeds(&lines[0]);
    println!("Got seeds");
    let pipeline = collect_pipeline(lines);
    println!("Got pipeilnes");

    let mut lowest = f64::INFINITY;
    for (index, seed) in seeds.iter().enumerate() {
        println!("0");
        let soil = get_value(&pipeline["seed-to-soil"], &*seed);
        println!("1");
        let fert = get_value(&pipeline["soil-to-fertilizer"], &soil);
        println!("2");
        let water = get_value(&pipeline["fertilizer-to-water"], &fert);
        println!("3");
        let light = get_value(&pipeline["water-to-light"], &water);
        println!("4");
        let temp = get_value(&pipeline["light-to-temperature"], &light);
        println!("5");
        let hum = get_value(&pipeline["temperature-to-humidity"], &temp);
        println!("6");
        let location = get_value(&pipeline["humidity-to-location"], &hum);

        let location_number = location.parse::<u64>().unwrap();
        if location_number < (lowest as u64) {
            lowest = location_number as f64;
        }

        println!("{} / {}", index, seeds.len());
    }

    println!("{}", lowest);
}

fn get_value(dictionary: &HashMap<String, String>, value: &str) -> String {
    let one = dictionary.get(value);
    match one {
        Some(dict_value) => dict_value.clone(),
        None => value.to_string(),
    }
}

fn collect_pipeline(lines: Vec<&str>) -> HashMap<String, HashMap<String, String>> {
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();

    let indexes = find_all_titles(&lines);

    for index in 0..indexes.len() - 1 {
        let title_index = indexes[index as usize] as usize + 1;
        let next_index = indexes[index as usize + 1] as usize;
        let slice: Vec<&str> = lines[title_index..next_index].to_vec();
        println!("indexes gerekend");

        let title = lines[title_index - 1].split(" ").collect::<Vec<&str>>()[0].to_string();

        let dictionary = create_mapping(&slice);
        result.insert(title, dictionary);
    }

    result
}

fn create_mapping(lines: &Vec<&str>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    for line in lines {
        if line == &"" {
            continue;
        }
        let numbers = line.split(" ").collect::<Vec<&str>>();

        let destination_start = numbers[0].parse::<u64>().unwrap();
        let source_start = numbers[1].parse::<u64>().unwrap();
        let range = numbers[2].parse::<u64>().unwrap();
        println!("Wat dingen berekend");

        let source_end = source_start + range;

        println!("Dit kan lang duren: {}", source_end - source_start);
        for index in source_start..source_end {
            let mapping_index = index.to_string();
            let mapped_number = destination_start + (index - source_start);
            let mapped_string = mapped_number.to_string();
            result.insert(mapping_index, mapped_string);
        }
        println!("Maar nu wel klaar");
    }

    result
}

fn find_all_titles(lines: &Vec<&str>) -> Vec<u16> {
    let mut indexes: Vec<u16> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if index == 0 || line == &"" {
            continue;
        }
        let first_character = line.chars().nth(0).unwrap();

        if first_character.is_digit(10) {
            continue;
        }

        indexes.push(index as u16);
    }

    indexes.push(lines.len() as u16);
    indexes
}

fn get_defined_seeds(line: &str) -> Vec<String> {
    let seeds_string: String = line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .replace(" ", ",");

    return seeds_string
        .split(",")
        .map(|x| x.trim().to_string())
        .collect();
}
