use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

fn main() {
    let file_path: &str = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let expansions: (Vec<bool>, Vec<bool>) = create_expansion_lines(&lines);

    let stars = find_all_stars(&lines);
    let pairs: Vec<((usize, usize), (usize, usize))> = create_pairs(&stars);

    let total_part_one = calculate_all_distances(&pairs, &expansions, 1);
    let total_part_two = calculate_all_distances(&pairs, &expansions, 1_000_000);

    println!(" -- ");
    println!("Part One: {total_part_one}");
    println!("Part Two: {total_part_two}");
}

fn calculate_all_distances(
    pairs: &Vec<((usize, usize), (usize, usize))>,
    expansion_lines: &(Vec<bool>, Vec<bool>),
    times: u64,
) -> u64 {
    let mut total = 0;
    let mut correction: u64 = 0;
    if times > 1 {
        correction = 1;
    }

    for pair in pairs {
        let mut add_to_x: u64 = 0;
        let mut add_to_y: u64 = 0;

        if pair.0 .0 < pair.1 .0 {
            for column in pair.0 .0..pair.1 .0 {
                if expansion_lines.0[column] {
                    add_to_x += times - correction;
                }
            }
        } else {
            for column in pair.1 .0..pair.0 .0 {
                if expansion_lines.0[column] {
                    add_to_x += times - correction;
                }
            }
        }

        if pair.0 .1 < pair.1 .1 {
            for row in pair.0 .1..pair.1 .1 {
                if expansion_lines.1[row] {
                    add_to_y += times - correction;
                }
            }
        } else {
            for row in pair.1 .1..pair.0 .1 {
                if expansion_lines.1[row] {
                    add_to_y += times - correction;
                }
            }
        }

        let one_line = (pair.0 .0 as f64 - pair.1 .0 as f64).abs();
        let two_line = (pair.0 .1 as f64 - pair.1 .1 as f64).abs();
        let distance = (one_line + two_line) as u64 + add_to_x + add_to_y;
        total += distance;
    }

    total
}

fn create_pairs(stars: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let temp_stars = stars.clone();
    let mut reversed_stars: Vec<(usize, usize)> = temp_stars
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<(usize, usize)>>();

    let mut pairs: Vec<((usize, usize), (usize, usize))> = Vec::new();

    while let Some(this_star) = reversed_stars.pop() {
        for &other_star in &reversed_stars {
            pairs.push((this_star, other_star));
        }
    }

    pairs
}

fn find_all_stars(lines: &Vec<&str>) -> Vec<(usize, usize)> {
    let mut stars_found: Vec<(usize, usize)> = Vec::new();

    println!(" -- ");
    println!("Finding all stars");

    let bar = ProgressBar::new(lines.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise} / {duration}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );
    for (index_y, line) in lines.iter().enumerate() {
        for (index_x, character) in line.chars().enumerate() {
            if character == '#' {
                stars_found.push((index_x, index_y));
            }
        }
        bar.inc(1);
    }
    bar.finish();

    stars_found
}

fn create_expansion_lines<'a>(lines: &Vec<&'a str>) -> (Vec<bool>, Vec<bool>) {
    let mut final_map: Vec<String> = Vec::new();

    let line_length = lines[0].len();
    let mut expand_bools: (Vec<bool>, Vec<bool>) = (Vec::new(), Vec::new());

    'x: for x in 0..line_length {
        for y in 0..lines.len() {
            let this_line = lines[y].chars().collect::<Vec<char>>();
            let current_character = this_line[x];

            if current_character == '#' {
                expand_bools.0.push(false);
                continue 'x;
            }
        }

        expand_bools.0.push(true);
    }

    'y: for line in lines {
        for character in line.chars() {
            if character == '#' {
                expand_bools.1.push(false);
                continue 'y;
            }
        }
        expand_bools.1.push(true);
    }
    expand_bools

    // println!(" -- ");
    // println!("Expanding x");

    // let mut bar = ProgressBar::new(lines.len() as u64);
    // bar.set_style(
    //     ProgressStyle::with_template(
    //         "[{elapsed_precise} / {duration}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    //     )
    //     .unwrap(),
    // );

    // let mut expanded_x_map: Vec<Vec<char>> = Vec::new();

    // for line in lines {
    //     let mut new_line: Vec<char> = Vec::new();

    //     for (index, character) in line.chars().enumerate() {
    //         new_line.push(character);

    //         if expand_bools[index] {
    //             for _x in 0..times - 1 {
    //                 new_line.push(character);
    //             }
    //         }
    //     }

    //     expanded_x_map.push(new_line);
    //     bar.inc(1);
    // }
    // bar.finish();

    // println!(" -- ");
    // println!("Expanding y");
    // bar = ProgressBar::new(lines.len() as u64 * 2);
    // bar.set_style(
    //     ProgressStyle::with_template(
    //         "[{elapsed_precise} / {duration}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    //     )
    //     .unwrap(),
    // );

    // for line in expanded_x_map {
    //     let new_line: String = line.iter().collect();
    //     final_map.push(new_line.clone());

    //     if !line.contains(&'#') {
    //         for _x in 0..times - 1 {
    //             final_map.push(new_line.clone());
    //         }
    //     }
    //     bar.inc(1);
    // }
    // bar.finish();
}
