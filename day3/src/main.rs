
#[derive(Debug)]
struct SymbolInfo {
    value: char,
    coordinates: (usize, usize),
}
#[derive(Debug)]
struct DigitInfo {
    value: i32,
    coordinates: Vec<Vec<i32>>,
}

fn main() {
    let content = include_str!("in.txt");
    let mut digit_list: Vec<DigitInfo> = Vec::new();
    let mut symbol_list: Vec<SymbolInfo> = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let mut j = 0;  // Initialize j outside the inner loop to make it mutable

        while j < line.len() {
            let c = line.chars().nth(j).unwrap();

            if c.is_digit(10) {
                let (num, coordinates_list) = get_the_number(&line, i, j);

                let num_str = num.to_string();
                let num_length = num_str.len();

                let dig_info = DigitInfo {
                    value: num,
                    coordinates: coordinates_list,
                };

                digit_list.push(dig_info);

                j += num_length;

            } else if !c.is_digit(10) && c != '.' {
                let sym_info = SymbolInfo {
                    value: c,
                    coordinates: (i, j),
                };
                symbol_list.push(sym_info);

                // Increment j only if a symbol is found
                j += 1;
            } else {
                // Increment j for other characters
                j += 1;
            }
        }
    }

    let adjacent_locations: [(i32, i32); 8] = [ 
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];

    let mut part1_sum: i32 = 0;
    let mut part2_sum: i32 = 0;
    
    for each in &symbol_list {
        let symbol_coordinates = (each.coordinates.0 as i32, each.coordinates.1 as i32);
        let symbol = each.value;
        let  mut itter_skip: i32 = 0;
        let mut adjacent_nums: i32 = 0;

        for loc in adjacent_locations{
            let new_loc = (loc.0 + symbol_coordinates.0, loc.1 + symbol_coordinates.1);
            if itter_skip > 0 {
                itter_skip -= 1;
                continue;
            }

            'outer: for num in &digit_list {
                'inner: for coordinates in &num.coordinates {

                    // there is num around the symbol
                    if new_loc.0 == coordinates[0] && new_loc.1 == coordinates[1] {
                        if symbol == '*' {
                            if adjacent_nums == 0{
                                adjacent_nums += num.value;
                            }
                            else if adjacent_nums != 0{
                                adjacent_nums *= num.value;
                                part2_sum += adjacent_nums;
                            }
                            
                        }
                        
                        part1_sum += num.value;

                        // if the num is in -1 pos and has a digit in 0 means that there is no possible other num on the line
                        // we skip the line
                        if loc.1 == -1 {
                            for other_coordinates in &num.coordinates {
                                if other_coordinates[1] == coordinates[1] + 1 {
                                    itter_skip += 2;
                                }
                            }   
                        }
                        
                        // if the num is in 0 pos there is no way another num is on the line - skip last char
                        if loc.1 == 0 {
                            itter_skip += 1;
                        }

                        break 'outer;
                    }
                }
            }
        }
    }

    println!("part1: {}", part1_sum);
    println!("part2: {}", part2_sum);
}

fn get_the_number(line: &str, i: usize, j: usize) -> (i32, Vec<Vec<i32>>) {
    let mut num = 0;
    let mut coordinates_list: Vec<Vec<i32>> = Vec::new();

    let mut current_position = j;
    let mut current_digit_positions: Vec<Vec<i32>> = Vec::new();

    while current_position < line.len() {
        match line.chars().nth(current_position) {
            Some(c) if c.is_digit(10) => {
                let digit = c.to_digit(10).unwrap() as i32;
                num = num * 10 + digit;

                // Store the position of the current digit as a pair of (i, j)
                current_digit_positions.push(vec![i as i32, current_position as i32]);
            }
            _ => break,
        }
        current_position += 1;
    }

    // If at least one digit was found, store the positions in the coordinates list
    if !current_digit_positions.is_empty() {
        coordinates_list.extend(current_digit_positions);
    }

    (num, coordinates_list)
}
