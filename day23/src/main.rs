
use std::collections::HashSet;

#[derive(Debug)]
struct Shortcut {
    pos1: (i32, i32),
    pre_pos1: (i32, i32),
    pos2: (i32, i32),
    pre_pos2: (i32, i32),
    steps: i32
}


fn main() {
    let input = include_str!("in.txt");
    let (start, stop, map) = parsing(input);
    let beta_shortcuts = create_shorcuts(&map, start, stop);
    
    // some shortcuts are returned for both ways (form 0,1 to 2,0 && 2,0 to 0,1) im not storing the duplicates
    let shortcuts = filter_shortcuts(beta_shortcuts);

    // part 1
    // let paths = find_paths(&map, start, stop, 1, &shortcuts);
    // if let Some(max_tuple) = paths.iter().max_by_key(|&(_, value)| value) {
    //     println!("Max value1: {}", max_tuple.1);
    // } else {
    //     println!("Vector is empty");
    // }
    
    // part 2
    // find paths that have no conjunctions or whatever - and the lengths in between
    let paths = find_paths(&map, start, stop, 2, &shortcuts);
    if let Some(max_tuple) = paths.iter().max_by_key(|&(_, value)| value) {
        println!("Max value2: {}", max_tuple.1);
    } else {
        println!("Vector is empty");
    }
}

fn filter_shortcuts(original_shortcuts: Vec<Shortcut>) -> Vec<Shortcut> {
    let mut unique_combinations: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    // Filter duplicates from beta_shortcuts and store in shortcuts
    let shortcuts: Vec<Shortcut> = original_shortcuts
        .into_iter()
        .filter(|beta_shortcut| {
            // Get pos1 and pos2 as a tuple, ensuring pos1 <= pos2
            let (pos1, pos2) = if beta_shortcut.pos1 <= beta_shortcut.pos2 {
                (beta_shortcut.pos1, beta_shortcut.pos2)
            } else {
                (beta_shortcut.pos2, beta_shortcut.pos1)
            };

            // Check if the combination is already in the HashSet
            let combination = (pos1, pos2);
            if unique_combinations.contains(&combination) {
                false // Duplicate, skip
            } else {
                // Add the combination to the HashSet and include the shortcut
                unique_combinations.insert(combination);
                true
            }
        })
        .collect();

    shortcuts
}

// bfs
fn create_shorcuts(map: &Vec<Vec<char>>, start: (i32, i32), stop: (i32, i32)) -> Vec<Shortcut> {
    let mut shortcuts: Vec<Shortcut> = Vec::new();
    let mut starts:  Vec<((i32, i32), (i32 ,i32))> = Vec::new();
    let conjunction_pos = (-1, -1); 

    starts.push((start, conjunction_pos));
    let mut new_starts: Vec<((i32, i32), (i32 ,i32))> = Vec::new();
    
    loop {
        for (start, con) in &new_starts{
            
            // filter if its not already in shortcuts or if its a duplicate
            if shortcuts.iter().any(|shortcut| shortcut.pos1 == *start || shortcut.pos2 == *start) ||
            starts.iter().any(|item| item.0 == *start) {
                continue;
            } else {
                starts.push((start.clone(), con.clone()))
            }
        }

        if starts.is_empty() {
            break;

        } else {
            for (start, conjunction_pos) in &starts { 
                
                let (shortcut, update_starts, update_conjunction_pos) = get_shortcut(&map, &start, stop, conjunction_pos);
                shortcuts.push(shortcut);
                for update_start in update_starts {
                    new_starts.push((update_start, update_conjunction_pos));
                }  
            }
            starts.clear();
        }
    }

    shortcuts
}

// takes start and continues the shortcut till it reaches to either stop or a conjunction

// todo: shortcut min length - indexing
fn get_shortcut(map: &Vec<Vec<char>>, start: &(i32, i32), stop: (i32, i32), con_pos: &(i32 ,i32)) -> (Shortcut, Vec<(i32, i32)>, (i32, i32)) {
    let shortcut: Shortcut;
    let mut new_starts: Vec<(i32, i32)> = Vec::new();
    let mut conjunction_pos = con_pos.clone();

    let pos1 = start.clone();
    let mut shortcut_steps_counter = 0;

    let mut path: Vec<(i32, i32)> = Vec::new();
    path.push(con_pos.clone());
    let mut current_pos = pos1.clone();

    loop {
        path.push(current_pos.clone());
        let adjacent = get_neighbours(map, &current_pos, &path, 2);


        if adjacent.len() > 1 {
            conjunction_pos = current_pos;
            new_starts = adjacent.clone();
            let path_len = path.len();

            shortcut = Shortcut {
                pos1,
                pre_pos1: path[2],
                pos2: path[path_len - 2],
                pre_pos2: path[path_len - 3],
                steps: shortcut_steps_counter - 1,
            };
            break;         

        } else { // if the path continues only in one direction
            shortcut_steps_counter += 1;
            current_pos = adjacent[0];

            if current_pos == stop { // the finish edge case
                let path_len = path.len();
                shortcut = Shortcut {
                    pos1,
                    pre_pos1: path[2],
                    pos2: current_pos,
                    pre_pos2: path[path_len - 1],
                    steps: shortcut_steps_counter,
                };
                break;
            }
        }
    }
    
    (shortcut, new_starts, conjunction_pos)
}

fn find_paths(map: &Vec<Vec<char>>, start: (i32, i32), stop: (i32, i32), part: i32, shortcuts: &Vec<Shortcut>) -> Vec<(Vec<(i32, i32)>,i32)> {
    let mut path: Vec<(i32, i32)> = Vec::new();
    let steps = 0;

    // classic step by step in all dir if more possibilities, clone paths and run in other direction
    // rules: can go only down (part 1) and not step on a tile twice

    let current_pos = start;
    path.push(current_pos.clone());
    
    let paths = continue_path(path, steps, &map, stop, part, shortcuts);
    for each in &paths {
        println!("{:?}", each.1)
    }
    paths
}

// dfs 
fn continue_path(last_path: Vec<(i32, i32)>, last_steps: i32, map: &Vec<Vec<char>>, stop: (i32, i32), part: i32, shortcuts: &Vec<Shortcut>) -> Vec<(Vec<(i32, i32)>,i32)> {
    let mut paths: Vec<(Vec<(i32, i32)>,i32)> = Vec::new();
    let mut steps = last_steps.clone();
    let mut path = last_path.clone();

    let mut current_pos = match last_path.last().cloned() {
        Some(pos) => pos,
        None => panic!("empty path")
    };

    
    loop { 
        // applying shortcuts
        
        for shortcut in shortcuts {
            // Check if the current position matches either pos1 or pos2 of the shortcut
            if current_pos == shortcut.pos1 || current_pos == shortcut.pos2 {
                if current_pos == shortcut.pos1 {
                    current_pos = shortcut.pos2.clone();
                    path.push(current_pos.clone());
                    path.push(shortcut.pre_pos2.clone());
                    steps += shortcut.steps;

                } else {
                    current_pos = shortcut.pos1.clone();
                    path.push(current_pos.clone());
                    path.push(shortcut.pre_pos1.clone());
                    steps += shortcut.steps;
                }
                break;
            }
        }

        if current_pos == stop {
            // println!("{}", &steps);
            paths.push((path, steps));
            break;
        }

        let neighbours: Vec<(i32, i32)> = get_neighbours(map, &current_pos, &path, part);
        if neighbours.is_empty() {
            break;
        } else if neighbours.len() == 1 {
            current_pos = neighbours[0];
            steps += 1;
            path.push(current_pos.clone());

        } else { // there is more then 1 possible neighbour
            // first neighbour continues here another ones go on their own
            for (i, neighbour) in neighbours.iter().enumerate() {
                if i == 0 {
                    current_pos = *neighbour;
                    steps += 1;
                    path.push(current_pos.clone());
                    
                } else { // second and following iterations
                    let mut new_path = path.clone();
                    new_path.push(neighbour.clone());
                    let new_paths = continue_path(new_path, steps, map, stop, part, shortcuts);
                    // playout these options
                    if !new_paths.is_empty() {
                        for each in new_paths {
                            // println!("{:?}", &each.1);
                            paths.push(each);
                        }
                    }
                }
            }
        }
    }

    paths
}

fn get_neighbours(map: &Vec<Vec<char>>, pos: &(i32, i32), path: &Vec<(i32, i32)>, part: i32) -> Vec<(i32, i32)> {
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)]; // n s e w

    if part  == 1 {
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

    } else { // part 2
        directions
        .iter()
        .filter_map(|&dir| {
            let new_pos = add_tuple(&pos, &dir);
            if is_in_map(map, new_pos) && !path.contains(&new_pos){
                match map[new_pos.0 as usize][new_pos.1 as usize] {
                    '#' => None,
                    _ => Some(new_pos),
                }
            } else {
                None
            }
        })
        .collect()
    }
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