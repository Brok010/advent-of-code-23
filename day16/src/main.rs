//could be further optimized with memoization of each start and even memoization of each 'mirror'

use core::panic;

struct Directions {
    north: (i32, i32),
    south: (i32, i32),
    east: (i32, i32),
    west: (i32, i32),
}

fn main() {
    let directions = Directions {
        north: (-1, 0),
        south: (1, 0),
        east: (0, 1),
        west: (0, -1),
    };
    let input = include_str!("in.txt");
    let map = parsing(input);
    let mut start_direction = directions.east;
    let mut final_hash_list: Vec<(i32, i32)> = Vec::new();
    let mut hash_list: Vec<(i32, i32)> = Vec::new();
    
    //different starts
    for col in -1..(map[0].len() + 1) as i32 {
        for row in -1..(map.len() + 1) as i32 {
            // for start pos to be valid either row or col has to be outside of the map
            if ((row == -1 || col == -1) || (row == map.len() as i32 || col == map[0].len() as i32)) && row != col {
                if (row == -1 && col == (map[0].len()) as i32) || (row == (map.len()) as i32 && col == -1) {
                    continue;
                }
                let start_pos = (row, col);

                if col == -1 {
                    start_direction = directions.east;
                } else if row == -1 {
                    start_direction = directions.south;
                } else if col == map[0].len() as i32 {
                    start_direction = directions.west;
                } else if row == map.len() as i32 {
                    start_direction = directions.north;
                } 
                hash_list.clear();
                let new_hash_list = beaming(start_pos, start_direction, &mut hash_list, &map, &directions);
                if new_hash_list.len() > final_hash_list.len() {
                    final_hash_list = new_hash_list.clone();
                }
            }   
        }
    }
    println!("energized: {}", final_hash_list.len())
}

fn beaming<'a>(start: (i32, i32), start_direction: (i32, i32), hash_list: &'a mut Vec<(i32, i32)>,
    map: &Vec<Vec<char>>, directions: &Directions) -> &'a mut Vec<(i32, i32)> {
    let mut current_position = start;
    let mut current_direction = start_direction;

    while true {
        current_position = add_touple(current_position, current_direction);
        if out_of_map(current_position, &map){
            return hash_list;  
        }

        let current_char = map[current_position.0 as usize][current_position.1 as usize].clone();
        // println!("{:?}", current_char);
        
        // check if current pos is in hash list
        if !hash_list.contains(&current_position) {
            hash_list.push(current_position.clone());
        } else { // if it already is in hash
            match current_char {
                // we would be entering a loop
                '-' => {
                    return hash_list;
                }
                '|' => {
                    return hash_list;
                }
                // ('/', _) => {}
                // ('\\', _) => {}
                _ => {}
            }
        }
        
        match current_char {
            '.' => {}
            '/' => {
                match current_direction {
                    d if d == directions.north => {
                        current_direction = directions.east;
                    }
                    d if d == directions.south => {
                        current_direction = directions.west;
                    }
                    d if d == directions.east => {
                        current_direction = directions.north;
                    }
                    d if d == directions.west => {
                        current_direction = directions.south;
                    }
                    _ => panic!("Invalid direction"),
                }
            }
            '|' => {
                match current_direction {
                    d if d == directions.north || d == directions.south => {
                        //
                    }
                    d if d == directions.east || d == directions.west => {
                        // continue south and start a new beam to north
                        current_direction = directions.south;
                        let mut cloned_hash_list = hash_list.clone();
                        let new_hash_list = beaming(current_position, directions.north, &mut cloned_hash_list,  &map, &directions);
                        for each in new_hash_list {
                            if !hash_list.contains(each) {
                                hash_list.push(*each);
                            }
                        }
                    }
                    _ => panic!("Invalid direction"),
                }
            }
            '-' => {
                match current_direction {
                    d if d == directions.north || d == directions.south => {
                        // continue east and start a new beam west
                        current_direction = directions.east;
                        let mut cloned_hash_list = hash_list.clone();
                        let new_hash_list = beaming(current_position, directions.west, &mut cloned_hash_list, &map, &directions);
                        for each in new_hash_list {
                            if !hash_list.contains(&each) {
                                hash_list.push(*each);
                            }
                        }
                    }

                    d if d == directions.east || d == directions.west => {
                        //
                    }
                    _ => panic!("Invalid direction"),
                }
            }
            '\\' => {
                match current_direction {
                    d if d == directions.north => {
                        current_direction = directions.west;
                    }
                    d if d == directions.south => {
                        current_direction = directions.east;
                    }
                    d if d == directions.east => {
                        current_direction = directions.south;
                    }
                    d if d == directions.west => {
                        current_direction = directions.north;
                    }
                    _ => panic!("Invalid direction"),
                }
            }
            _ => panic!("Invalid char"),
        }
    }
    hash_list
}

fn out_of_map(pos: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    let row = pos.0;
    let col = pos.1;
    let map_width = map[0].len() -1;
    let map_height = map.len() - 1;
    if row < 0 || row > map_height as i32 || col < 0 || col > map_width as i32 {
        return true;
    }
    false
}

fn add_touple(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    let sum_tuple = (t1.0 + t2.0, t1.1 + t2.1);
    sum_tuple
}

fn parsing (input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for lines in input.lines(){
        let mut line: Vec<char> = Vec::new();

        for char in lines.chars(){
            line.push(char);
        }
        map.push(line);
    }
    map
}
