use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
use std::{fs, vec};

fn main() {
    let file_path: &str = "input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let starting_point = find_starting_point(&lines);

    let main_loop = gather_main_loop(&lines, starting_point);

    let result = part_two(&lines, &main_loop);

    draw(&lines, &main_loop, &result);
    println!("Part One: {}", (main_loop.len() - 1) / 2);
    println!("Part Two: {}", result.len());
}

fn part_two(lines: &Vec<&str>, main_loop: &Vec<(i32, i32)>) -> Vec<(u32, u32)> {
    let mut collected_points: Vec<(u32, u32)> = Vec::new();

    let bar = ProgressBar::new(lines.len() as u64);

    for (line_index, line) in lines.iter().enumerate() {
        for char_index in 0..line.len() {
            if main_loop.contains(&(char_index as i32, line_index as i32)) {
                continue;
            }

            let horizontal =
                check_horizontal_encapsulated((char_index, line_index), &line, &main_loop);
            if !horizontal {
                continue;
            }

            let vertical =
                check_vertical_encapsulated((char_index, line_index), &lines, &main_loop);
            if !vertical {
                continue;
            }

            collected_points.push((char_index as u32, line_index as u32));
        }
        bar.inc(1);
    }
    bar.finish();

    return collected_points;
}

fn check_vertical_encapsulated(
    coordinate: (usize, usize),
    lines: &Vec<&str>,
    main_loop: &Vec<(i32, i32)>,
) -> bool {
    let ranges = vec![
        (0..coordinate.1).rev().collect::<Vec<_>>(),
        (coordinate.1..lines.len()).collect::<Vec<_>>(),
    ];

    for range in ranges {
        let mut side = "";
        let mut count = 0;
        for x in range {
            let current_character: char = lines[x].chars().collect::<Vec<char>>()[coordinate.0];

            if !main_loop.contains(&(coordinate.0 as i32, x as i32)) {
                continue;
            }

            if vec!['-', 'S'].contains(&current_character) {
                count += 1;
                side = "";
                continue;
            }

            if current_character == '|' && side != "" {
                continue;
            }

            if vec!['7', 'J'].contains(&current_character) {
                if side == "" {
                    side = "left";
                } else if side == "left" {
                    side = "";
                } else if side == "right" {
                    side = "";
                    count += 1;
                }
                continue;
            }

            if vec!['F', 'L'].contains(&current_character) {
                if side == "" {
                    side = "right";
                } else if side == "right" {
                    side = "";
                } else if side == "left" {
                    side = "";
                    count += 1;
                }
                continue;
            }
        }

        if count % 2 == 0 {
            return false;
        }
    }

    return true;
}

fn check_horizontal_encapsulated(
    coordinate: (usize, usize),
    line: &str,
    main_loop: &Vec<(i32, i32)>,
) -> bool {
    let ranges = vec![
        (0..coordinate.0).rev().collect::<Vec<_>>(),
        (coordinate.0..line.len()).collect::<Vec<_>>(),
    ];

    // Traverse left
    for range in ranges {
        let mut side = "";
        let mut count = 0;
        for x in range {
            let current_character: char = line.chars().collect::<Vec<char>>()[x];

            if !main_loop.contains(&(x as i32, coordinate.1 as i32)) {
                continue;
            }

            if current_character == '|' {
                count += 1;
                side = "";
                continue;
            }

            if vec!['-', 'S'].contains(&current_character) && side != "" {
                continue;
            }

            if vec!['J', 'L'].contains(&current_character) {
                if side == "" {
                    side = "up";
                } else if side == "up" {
                    side = "";
                } else if side == "down" {
                    side = "";
                    count += 1;
                }
                continue;
            }

            if vec!['F', '7'].contains(&current_character) {
                if side == "" {
                    side = "down";
                } else if side == "down" {
                    side = "";
                } else if side == "up" {
                    side = "";
                    count += 1;
                }
                continue;
            }
        }

        if count % 2 == 0 {
            return false;
        }
    }

    return true;
}

fn draw(lines: &Vec<&str>, main_loop: &Vec<(i32, i32)>, insides: &Vec<(u32, u32)>) {
    let mut final_strings: Vec<String> = Vec::new();

    let bar = ProgressBar::new(lines.len() as u64);
    for (index_y, line) in lines.iter().enumerate() {
        let mut current_characters: Vec<char> = Vec::new();
        for (index_x, character) in line.chars().enumerate() {
            let mut current_character = '.';

            if main_loop.contains(&(index_x as i32, index_y as i32)) {
                current_character = character;
            }

            if insides.contains(&(index_x as u32, index_y as u32)) {
                current_character = 'I';
            }

            current_characters.push(current_character);
        }
        final_strings.push(current_characters.into_iter().collect::<String>());
        bar.inc(1);
    }
    bar.finish();

    let mut file = File::create("output.txt").expect("Unable to create file");
    for line in final_strings {
        writeln!(file, "{}", line).expect("Unable to write data");
    }
}

fn gather_main_loop(lines: &Vec<&str>, starting_point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut main_loop: Vec<(i32, i32)> = Vec::new();
    main_loop.push(starting_point);

    let mut current_char = 's';

    while current_char != 'S' {
        let last_element = main_loop[main_loop.len() - 1];
        let current_x = last_element.0;
        let current_y = last_element.1;

        let mut adjacent_pipes: Vec<(char, (i32, i32))> = Vec::new();

        'y: for y_diff in -1 as i32..2 {
            'x: for x_diff in -1 as i32..2 {
                let y_index = current_y + y_diff;
                let x_index = current_x + x_diff;

                if y_diff.abs() == x_diff.abs()
                    || (main_loop.len() >= 2
                        && (x_index, y_index) == main_loop[main_loop.len() - 2])
                {
                    continue;
                }

                if y_index >= lines.len() as i32 || y_index < 0 {
                    continue 'y;
                }

                let this_line = lines[y_index as usize].chars().collect::<Vec<char>>();

                if x_index >= this_line.len() as i32 || x_index < 0 {
                    continue 'x;
                }

                let this_char = this_line[x_index as usize];
                adjacent_pipes.push((this_char, (x_index, y_index)));
            }
        }

        for pipe in &adjacent_pipes {
            let pipe_coordinate = pipe.1;

            if check_connecting(
                pipe_coordinate.0 - current_x,
                pipe_coordinate.1 - current_y,
                current_char,
                pipe.0,
            ) {
                current_char = pipe.0;
                main_loop.push(pipe_coordinate);
                break;
            }
        }
    }

    return main_loop;
}

fn check_connecting(x_diff: i32, y_diff: i32, current_char: char, next_char: char) -> bool {
    if y_diff == -1
        && vec!['|', 'L', 'J', 's'].contains(&current_char)
        && vec!['F', '|', '7', 'S'].contains(&next_char)
    {
        return true;
    }

    if y_diff == 1
        && vec!['|', 'F', '7', 's'].contains(&current_char)
        && vec!['L', '|', 'J', 'S'].contains(&next_char)
    {
        return true;
    }

    if x_diff == -1
        && vec!['-', 'J', '7', 's'].contains(&current_char)
        && vec!['F', '-', 'L', 'S'].contains(&next_char)
    {
        return true;
    }

    if x_diff == 1
        && vec!['-', 'L', 'F', 's'].contains(&current_char)
        && vec!['J', '-', '7', 'S'].contains(&next_char)
    {
        return true;
    }

    false
}

fn find_starting_point(lines: &Vec<&str>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    'y: for (index_y, line) in lines.iter().enumerate() {
        for (index_x, character) in line.chars().enumerate() {
            if character == 'S' {
                x = index_x as i32;
                y = index_y as i32;
                break 'y;
            }
        }
    }

    return (x, y);
}
