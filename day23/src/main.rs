
fn main() {
    let input = include_str!("in.txt");
    let (start, stop, map) = parsing(input);
    let paths = find_paths(map, start, stop);
    
    if let Some(max_tuple) = paths.iter().max_by_key(|&(_, value)| value) {
        println!("Max value: {}", max_tuple.1);
    } else {
        println!("Vector is empty");
    }
}

fn find_paths(map: Vec<Vec<char>>, start: (i32, i32), stop: (i32, i32)) -> Vec<(Vec<(i32, i32)>,i32)> {
    let mut path: Vec<(i32, i32)> = Vec::new();
    let mut steps = 0;

    // classic step by step in all dir if more possibilities, clone paths and run in other direction
    // rules: can go only down and not step on a tile twice

    let current_pos = start;
    steps += 1;
    path.push(current_pos.clone());
    
    let paths = continue_path(path, steps, &map);

    paths
}

fn continue_path(last_path: Vec<(i32, i32)>, last_steps: i32, map: &Vec<Vec<char>>) -> Vec<(Vec<(i32, i32)>,i32)> {
    let mut paths: Vec<(Vec<(i32, i32)>,i32)> = Vec::new();
    let mut steps = last_steps.clone();
    let last_pos = last_path.last().unwrap();
    let mut path = last_path.clone();

    let mut current_pos = *last_pos;
    
    loop { // todo: steps counting
        path.push(current_pos.clone());

        let neighbours: Vec<(i32, i32)> = get_neighbours(map, &current_pos, &path);
        if neighbours.is_empty() {
            break;
        } else if neighbours.len() == 1 {
            current_pos = neighbours[0];
            steps += 1;

        } else { // there is more then 1 possible neighbour
            // first neighbour continues here another ones go on their own
            for (i, neighbour) in neighbours.iter().enumerate() {
                if i == 0 {
                    current_pos = *neighbour;
                    steps += 1;
                    continue;
                } else { // second and following iterations
                    let mut new_path = path.clone();
                    new_path.push(neighbour.clone());
                    let new_paths = continue_path(new_path, steps + 1, map);
                    // playout these options
                    if !new_paths.is_empty() {
                        for each in new_paths {
                            paths.push(each);
                        }
                    }
                }
            }
        }
    }


    paths
}

fn get_neighbours(map: &Vec<Vec<char>>, pos: &(i32, i32), path: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)]; // n s e w

    directions
        .iter()
        .filter_map(|&dir| {
            let new_pos = add_tuple(&pos, &dir);
            if is_in_map(map, new_pos) && !path.contains(&new_pos){
                match map[new_pos.0 as usize][new_pos.1 as usize] {
                    '.' => Some(new_pos),
                    '#' => None,
                    '^' if dir == (-1, 0) => Some(new_pos),
                    'v' if dir == (1, 0) => Some(new_pos),
                    '>' if dir == (0, 1) => Some(new_pos),
                    '<' if dir == (0, -1) => Some(new_pos),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}

fn add_tuple(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn is_in_map(map: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < map.len() as i32 && pos.1 >= 0 && pos.1 < map[0].len() as i32
}

fn parsing(input: &str) -> ((i32, i32), (i32, i32), Vec<Vec<char>>) {
    let mut map:Vec<Vec<char>> = Vec::new();

    for line in input.lines() {

        let mut lin: Vec<char> = Vec::new();
        for character in line.chars() {
            lin.push(character);
        }

        map.push(lin)
    }  
    let mut start: (i32, i32) = (0, 0);
    let mut stop: (i32, i32) = (0, 0);

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let current_symbol = map[i][j];
            if i == 0 && current_symbol == '.' {
                start = (i as i32, j as i32);
            }
            if i == map.len() - 1 && current_symbol == '.' {
                stop = (i as i32, j as i32);
            }
        }
    }

    (start, stop, map)
}