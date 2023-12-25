// dijkstra alg
use std::collections::HashMap;

fn main() {
    let input = include_str!("in.txt");
    let map = parsing(input);
    let part1 = path(&map);

    println!("{}", part1);

}
fn path(map: &Vec<Vec<char>>) -> i32 {
    
    let map_length = map.len() - 1;
    let map_width = map[0].len() -1 ; 
    let start_pos = (0, 0);

    //create list of positions
    let mut distance_map = HashMap::new(); //capacity?
    distance_map.insert(start_pos, 0);
    
    // get the positions_distance chart
    for i in 0..=map_length {
        for j in 0..=map_width {
            if i == 0 && j == 0 { //start pos
                continue;
            }

            let pos = (i as i32, j as i32);
            distance_map.insert(pos, i32::MAX); //initialize these to big num
        }
    }

    let mut unvisited_map = distance_map.clone();
    
    //initialize steps for each distance
    let mut step_map = HashMap::new();
    for (key, _) in &distance_map {
        step_map.insert(key.clone(), [0, 0, 0, 0]);
    }


    while !unvisited_map.is_empty() {
        // visit the place with lowest in distance and unvisited
        //check if there are places to go to - with value != max and unvisited
        
        let mut common_keys = distance_map.keys().filter(|key| unvisited_map.contains_key(key));

        // Check if all keys have values equal to i32::MAX
        if common_keys.all(|key| *distance_map.get(key).unwrap() == i32::MAX) {
            // If all keys have values equal to i32::MAX, panic
            panic!("All common keys have values equal to i32::MAX");
        }

        let mut lowest_distance = i32::MAX;
        let mut current_pos: ((i32, i32), i32) = ((0, 0), i32::MAX);
        for each in &distance_map{
            if *each.1 < lowest_distance && unvisited_map.contains_key(&each.0) {
                lowest_distance = each.1.clone();
                current_pos = ((each.0.clone()), lowest_distance)
            }   // what if they are ==
        }

        // delete the current_pos from unvisited
        if current_pos.1 != i32::MAX { // if found
            unvisited_map.remove(&(current_pos.0));
        }
        // get current's steps
        let mut current_steps = [0,0,0,0];
        for (key, value) in &step_map {
            if *key == current_pos.0 {
                current_steps = value.clone();
                break;
            }
        }

        //check adjacent positions - update the distance map if already in distance map keep the lower
        let neighbors = get_adjacent(current_pos, &map, map_width, map_length);
        for neighbor in neighbors {
            
            // if the neighbor is valid new value is smaller then original value
            for (key, value) in distance_map.iter_mut() {
                let new_value = neighbor.1 + current_pos.1;
                if neighbor.0 == *key && new_value < *value {
                    // check for if it is possible - direction wise to go to current neighbor
                    
                    for (step_key, step_value) in step_map.iter_mut() {

                        //if neighbor wouldnt be an unvalid continuation of path
                        if neighbor.0 == *step_key && contains_less_four(neighbor.2.clone(), current_steps.clone()) {               
                             
                            *value = new_value;
                            //sum the arrays and if there are not 3 zeros...
                            let new_array = sum_arrays(neighbor.2.clone(), current_steps);
                            println!("{:?}, {:?}, {:?}", new_array, neighbor.2, current_steps);
        
                            if new_array.iter().filter(|&x| *x == 0).count() == 3 {
                                // we continue in the direction
                                *step_value = new_array;
                                break;
                            } else { // we have changed direction
                                *step_value = neighbor.2;
                                break;
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
    // sort distance_map
    let mut sorted_distance_map: Vec<((i32, i32), i32)> = distance_map.clone().into_iter().collect();
    sorted_distance_map.sort_by(|a, b| {
        // Sort first by key.0 and then by key.1
        a.0 .0.cmp(&b.0 .0).then_with(|| a.0 .1.cmp(&b.0 .1))
    });

    let result = distance_map.get(&(map_length as i32, map_width as i32)).cloned().unwrap();
    result
}

fn sum_arrays (a1: [i32;4], a2:[i32;4]) -> [i32;4]{
    let mut new_array = [0,0,0,0];
    for i in 0..a1.len() {
        new_array[i] = a1[i] + a2[i];
    }
    new_array
}

// sum arrays and if it contains more than 2 return false
fn contains_less_four(a1: [i32;4], a2:[i32;4]) -> bool {
    let new_array = sum_arrays(a1, a2);
    if new_array.iter().any(|&x| x > 3) {
        false
    } else {
        true
    }
}

fn get_adjacent(pos: ((i32, i32), i32), map: &Vec<Vec<char>>, map_width: usize, map_length: usize) -> Vec<((i32, i32), i32, [i32; 4])> {
    let mut neighbors: Vec<((i32, i32), i32, [i32; 4])> = Vec::new();
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)]; // n s e w
    let arrays = [[1, 0, 0, 0],[0, 1, 0, 0],[0, 0, 1, 0],[0, 0, 0, 1]];
    
    for i in 0..directions.len() {
        let new_pos = sum_tuple(pos.0, directions[i]);
        // if new position in map add its values into result
        if (new_pos.0 >= 0 && new_pos.0 <= map_length as i32) && (new_pos.1 >= 0 && new_pos.1 <= map_width as i32) {
            
            if let Some(digit) = map[new_pos.0 as usize][new_pos.1 as usize].to_digit(10) {
                let new_neighbor = ((new_pos), digit as i32, arrays[i]);
                neighbors.push(new_neighbor);
            }
        }
    }
    neighbors
}

fn sum_tuple(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32){
    return (t1.0 + t2.0, t1.1 + t2.1)
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
