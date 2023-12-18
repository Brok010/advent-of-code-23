use std::collections::HashSet;

fn main() {
    let input = include_str!("in.txt");
    let map = parsing(input);
    let mut starting_pos: (usize, usize) = (0, 0);
    
    // find S
    'outer: for row in 0..map.len() - 1 {
        for col in 0..map[row].len() -1 {
            if map[row][col] == 'S'{
                starting_pos = (row, col);
                break 'outer;
            }
        }
    }

    //logic - part1
    let mut counter = 0;
    let mut pos1 = starting_pos;
    let mut pos2 = starting_pos;
    let mut positions: Vec<(usize, usize)> = Vec::new();
    positions.push(pos1.clone());
    positions.push(pos2.clone());

    let mut last_positions: Vec<Vec<(usize, usize)>> = vec![vec![starting_pos.clone()], vec![starting_pos.clone()]];
    let directions: Vec<(&str, Vec<char>)> = vec![
        ("north", vec!['|', '7', 'F']),
        ("south", vec!['|', 'L', 'J']),
        ("east", vec!['-', 'J', '7']),
        ("west", vec!['-', 'L', 'F']),
    ];

    
    while positions[0] != positions[1] || counter == 0 {
        for i in 0..positions.len() {
            
            let mut found_flag = 0;

            let current_symbol = map[positions[i].0][positions[i].1];
            let possibilities = match current_symbol {
                '|' => vec!["north", "south"],
                '-' => vec!["east", "west"],
                '7' => vec!["south", "west"],
                'J' => vec!["north", "west"],
                'F' => vec!["south", "east"],
                'L' => vec!["north", "east"],
                'S' => vec!["north", "east", "south", "west"],
                _ => panic!("Invalid symbol: {}", current_symbol), // Panics for unknown symbols
            };
    

            // find next position that is not the last position
            for possibility in &possibilities{
                let direction = directions.iter().find(|&&(dir, _)| dir == *possibility).unwrap();
                let (found, new_position) = get_next_position(&positions[i], &last_positions[i], &map, direction);
                found_flag = found;
                
                if found_flag == 1 {
                    
                    // if we are in the 1st iteration - need to make sure that the paths go each its own way
                    if counter == 0 && new_position == positions[0]{
                        continue;

                    }else {
                        // need to push this in before the new asignment but after the function
                        last_positions[i].push(positions[i]);

                        positions[i] = new_position;
                        break;
                    }
                }
            }
        }
        counter += 1;

        // put the last postion of both vecs into the array so i can work with it in part2
        if positions[0] == positions[1] {
            last_positions[0].push(positions[0]);
            last_positions[1].push(positions[1]);
        }
    }

    println!("part1: {}", counter);

    // part 2
    let combined_last_positions: HashSet<_> = last_positions[0].clone().into_iter().chain(last_positions[1].clone().into_iter()).collect();
    
    let mut in_loop_counter = 0;
    for row in 0..map.len()- 1{
        for col in 0..map[row].len() - 2 { // 1 for indexing and another because the last char can never be counter into part2
            let current_position = (row, col);
            if !combined_last_positions.contains(&current_position) {
                let mut checked_for_pos = Vec::new();
                let out = check_surrounding(&current_position, &combined_last_positions, &map, &mut checked_for_pos);
                if out == 1 {
                    println!("{:?}", &current_position);
                    in_loop_counter += 1;
                }
            }
        }
    }

    println!("part2: {}", in_loop_counter);

}

fn check_surrounding(current_position: &(usize, usize),
                     combined_last_positions: &HashSet<(usize, usize)>,
                     map: &Vec<Vec<char>>,
                     checked_for_pos: &mut Vec<(usize, usize)>) -> usize {
                        
    let (row, col) = *current_position;
    checked_for_pos.push(current_position.clone());
    let map_size = (map.len(), map[0].len()); 
    let surrounding_pos = vec![
        (row.wrapping_sub(1), col),     // North
        (row, col.wrapping_add(1)),     // East
        (row, col.wrapping_sub(1)),     // West
        (row.wrapping_add(1), col),     // South
    ];
    // Check if any surrounding position is outside the map boundaries
    for x in &surrounding_pos {
        if x.0 > map_size.0 || x.1 > map_size.1 {
            return 0;
        }
    }

    
    let mut container = [0; 4];

    for position in 0..surrounding_pos.len(){
        if checked_for_pos.contains(&surrounding_pos[position]){
            container[position] += 1;

        } else if combined_last_positions.contains(&surrounding_pos[position]){
            container[position] += 1;

        } else { //the pos is not in the loop - check if it is surrounded
            let new_current_pos = surrounding_pos[position];
            
            let out = check_surrounding(&new_current_pos, combined_last_positions, &map, checked_for_pos);
            if out == 1 {
                container[position] += 1;
            }
        }
    }

    if container.iter().all(|&count| count > 0) {
        return 1;
    } else {
        return 0;
    }
}

fn get_next_position(position: &(usize, usize), last_positions: &Vec<(usize, usize)>, map: &Vec<Vec<char>>, direction: &(&str, Vec<char>)) -> (usize, (usize, usize)) {
    let mut new_position = (0, 0);
    let mut found = 0;

    let next_position = match direction.0 {
        "north" if position.0 > 0 => (position.0 - 1, position.1),
        "south" if position.0 < map.len() - 1 => (position.0 + 1, position.1),
        "east" if position.1 < map[0].len() - 1 => (position.0, position.1 + 1),
        "west" if position.1 > 0 => (position.0, position.1 - 1),
        _ => return (found, new_position), // Default case
    };

    // let current = map[position.0][position.1];
    // let last = map[last_positions.last().unwrap().0][last_positions.last().unwrap().1];
    // let to_be = map[next_position.0][next_position.1];

    if !last_positions.contains(&next_position) {    
        for &each in &direction.1{
            if map[next_position.0][next_position.1] == each {
                new_position = next_position;
                found = 1;
                return (found, new_position);
            }
        }
    }
    (found, new_position)
} 

fn parsing(input: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();

    for line in input.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        map.push(line_chars);
    }

    map
}