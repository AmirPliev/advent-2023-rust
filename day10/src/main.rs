use std::fs;

fn main() {
    let file_path: &str = "test_input.txt";
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let starting_point = find_starting_point(&lines);
    println!("{:?}", starting_point);

    let main_loop = gather_main_loop(&lines, starting_point);
    println!("{:?}", main_loop);
}

fn gather_main_loop(lines: &Vec<&str>, starting_point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut main_loop: Vec<(i32, i32)> = Vec::new();
    main_loop.push(starting_point);

    let mut current_char = '.';

    // while current_char != 'S' {
    for x in 0..50 {
        let last_element = main_loop[main_loop.len() - 1];
        let mut current_x = last_element.0;
        let mut current_y = last_element.1;

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

                if check_connecting(x_diff, y_diff, this_char) {
                    current_char = this_char;
                    main_loop.push((x_index, y_index));
                    println!("adding: ({x_index}, {y_index})");
                    break 'y;
                }
            }
        }
    }

    return main_loop;
}

fn check_connecting(x_diff: i32, y_diff: i32, current_char: char) -> bool {
    return (y_diff == -1 && vec!['F', '|', '7'].contains(&current_char))
        || (y_diff == 1 && vec!['L', '|', 'J'].contains(&current_char))
        || (x_diff == -1 && vec!['F', '-', 'L'].contains(&current_char))
        || (y_diff == 1 && vec!['J', '-', '7'].contains(&current_char));
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
