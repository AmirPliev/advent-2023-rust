// use indicatif::ProgressBar;
use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let seeds = get_defined_seeds(&lines[0]);
    let pipeline = collect_pipeline(lines);

    let mut lowest_part_one: f64 = f64::INFINITY;
    for seed in &seeds {
        let location = get_seed_location(&seed, &pipeline);

        if location < lowest_part_one {
            lowest_part_one = location;
        }
    }

    let mut part_two_seeds: Vec<Vec<u64>> = Vec::new();
    for index in (0..seeds.len()).step_by(2) {
        let start_seed = seeds[index].parse::<u64>().unwrap();
        let end_seed = seeds[index + 1].parse::<u64>().unwrap() + start_seed;

        part_two_seeds.push(vec![start_seed, end_seed]);
    }

    let pipeline_keys = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    for key in pipeline_keys {
        let block: &Vec<HashMap<String, u32>> = &pipeline[key];
        let mut new: Vec<Vec<u64>> = Vec::new();

        while part_two_seeds.len() > 0 {
            let range: Vec<u64> = part_two_seeds.pop().unwrap();
            let s = range[0] as u64;
            let e = range[1] as u64;

            let mut append_thing = true;

            for line in block {
                let a = line["destination"] as u64;
                let b = line["start"] as u64;
                let c = line["end"] as u64;

                let os = cmp::max(s, b);
                let oe = cmp::min(e, c);

                if os < oe {
                    new.push(vec![os - b + a, oe - b + a]);

                    if os > s {
                        part_two_seeds.push(vec![s, os]);
                    }

                    if e > oe {
                        part_two_seeds.push(vec![oe, e]);
                    }
                    append_thing = false;
                    break;
                }
            }

            if append_thing {
                new.push(vec![s, e]);
            }
        }

        part_two_seeds = new;
    }

    let mut lowest_part_two: f64 = f64::INFINITY;
    for range in part_two_seeds {
        if range[0] < lowest_part_two as u64 {
            lowest_part_two = range[0] as f64;
        }
    }

    println!("Part One: {}", lowest_part_one);
    println!("Part Two: {}", lowest_part_two as u64);
}

fn get_seed_location(seed: &String, pipeline: &HashMap<String, Vec<HashMap<String, u32>>>) -> f64 {
    let first_seed = seed.parse::<u32>().unwrap();

    let soil = get_value(&pipeline["seed-to-soil"], first_seed);
    let fert = get_value(&pipeline["soil-to-fertilizer"], soil);
    let water = get_value(&pipeline["fertilizer-to-water"], fert);
    let light = get_value(&pipeline["water-to-light"], water);
    let temp = get_value(&pipeline["light-to-temperature"], light);
    let hum = get_value(&pipeline["temperature-to-humidity"], temp);
    let location = get_value(&pipeline["humidity-to-location"], hum);

    return location as f64;
}

fn get_value(mappings: &Vec<HashMap<String, u32>>, value: u32) -> u32 {
    for mapping in mappings {
        if value >= mapping["start"] && value < mapping["end"] {
            let destination = mapping["destination"];

            if destination < mapping["start"] {
                let delta = mapping["start"] - destination;
                return value - delta;
            } else {
                let delta = destination - mapping["start"];
                return value + delta;
            }
        }
    }

    return value;
}

fn collect_pipeline(lines: Vec<&str>) -> HashMap<String, Vec<HashMap<String, u32>>> {
    let mut result: HashMap<String, Vec<HashMap<String, u32>>> = HashMap::new();

    let indexes = find_all_titles(&lines);

    for index in 0..indexes.len() - 1 {
        let title_index = indexes[index as usize] as usize + 1;
        let next_index = indexes[index as usize + 1] as usize;
        let slice: Vec<&str> = lines[title_index..next_index].to_vec();

        let title = lines[title_index - 1].split(" ").collect::<Vec<&str>>()[0].to_string();

        let dictionary = create_mapping(&slice);
        result.insert(title, dictionary);
    }

    result
}

fn create_mapping(lines: &Vec<&str>) -> Vec<HashMap<String, u32>> {
    let mut result: Vec<HashMap<String, u32>> = Vec::new();

    for line in lines {
        if line == &"" {
            continue;
        }
        let numbers = line.split(" ").collect::<Vec<&str>>();

        let destination_start = numbers[0].parse::<u64>().unwrap();
        let source_start = numbers[1].parse::<u64>().unwrap();
        let range = numbers[2].parse::<u64>().unwrap();

        let source_end = source_start + range;
        let mut new_mapping: HashMap<String, u32> = HashMap::new();
        new_mapping.insert("start".to_owned(), source_start as u32);
        new_mapping.insert("end".to_owned(), source_end as u32);
        new_mapping.insert("destination".to_owned(), destination_start as u32);
        result.push(new_mapping);
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
