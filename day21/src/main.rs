
fn main() {
    let input = include_str!("in.txt");
    let (map, start, rocks) = parsing(input);
    
    //part 1
    // let mut step_counter = 0;
    // let mut steps: Vec<(i32, i32)> = Vec::new();
    // steps.push(start);

    // while step_counter != 64 {
    //     let clonned_steps = steps.clone();
    //     steps = update_steps(clonned_steps, &rocks, &map);
    //     // print_map(&steps, &map);

    //     step_counter += 1;
    // }

    // println!("{}", steps.len());

    //part 2 - each subsequent map starts at the same position and has the same outcome, the diagonal ones would have 2 starts
    let map_width = map[0].len() as i32;
    // let map_height = map.len() as i32;

    let steps = 26501365;
    let n: i64 = ((steps - map_width / 2) / map_width) as i64;

    // make a map of every position and its first distance from the starting position
    let pos_map = make_pos_map(&map, &rocks, &start);

    let mut even_p1 = 0;
    let mut even_corners = 0;
    let mut odd_corners = 0;
    let mut even_full = 0;
    let mut odd_full = 0;

    for ((_, _), distance) in pos_map {
        if distance % 2 == 0 {
            even_full += 1;
            if distance > 64 {
                even_corners += 1;
            } else {
                even_p1 += 1;
            }
        } else {
            odd_full += 1;
            if distance > 64 {
                odd_corners += 1;
            }
        }
    }

    let p2 = ((n+1)*(n+1)) * odd_full + (n*n) * even_full - (n+1) * odd_corners + n * even_corners;
    println!("p2: {}", p2); // this is wrong - the equation should be right so idk why
    println!("p1: {}", even_p1); // this is right
}

fn make_pos_map(map: &Vec<Vec<char>>, rocks: &Vec<(i32, i32)>, start: &(i32 ,i32)) -> Vec<((i32, i32), i32)> { //bfs

    let mut unvisited: Vec<((i32, i32), i32)> = Vec::new();
    unvisited.push((start.clone(), 0));
    let mut visited: Vec<((i32, i32), i32)> = Vec::new();

    while !unvisited.is_empty() {
        let new_pos = unvisited[0];
        unvisited.remove(0);
        visited.push(new_pos.clone());

        let next_steps = get_neighbours(new_pos.0, rocks, map);
        for step in next_steps {
            if !has_same_first_value(&visited, &unvisited, &step) {
                unvisited.push((step, new_pos.1 + 1));
            }
        }
    }
    visited
}

fn has_same_first_value(
    visited: &Vec<((i32, i32), i32)>,
    unvisited: &Vec<((i32, i32), i32)>,
    step: &(i32, i32),
) -> bool {
    visited
        .iter()
        .any(|&(pos, _)| pos == *step)
        || unvisited
            .iter()
            .any(|&(pos, _)| pos == *step)
}

fn update_steps(steps: Vec<(i32, i32)>, rocks: &Vec<(i32, i32)>, map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut new_steps: Vec<(i32, i32)> = Vec::new();

    for each in steps {
        let neighbours = get_neighbours(each, rocks, &map);
        for neighbour in neighbours {
            if !new_steps.contains(&neighbour) {
                new_steps.push(neighbour)
            }
        }
    }

    new_steps
}

fn get_neighbours(position: (i32, i32), rocks: &Vec<(i32, i32)>, map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)]; // n s e w
    let mut neighbours: Vec<(i32, i32)> = Vec::new();

    for dir in directions{
        let new_tuple = add_tuple(position, dir);
        if !rocks.contains(&new_tuple) && !is_out_of_map(new_tuple, map) {
            neighbours.push(new_tuple);
        }
    }
    neighbours
}

fn is_out_of_map(t: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    let map_width = map[0].len() as i32;
    let map_height = map.len() as i32;

    if t.0 < 0 || t.0 > map_height - 1 || t.1 < 0 || t.1 > map_width - 1 {
        return true;
    }
    false
}

fn add_tuple(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    return (t1.0 + t2.0, t1.1 + t2.1)
}

fn print_map(steps: &Vec<(i32, i32)>, map: &Vec<Vec<char>>) {
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for (i, line) in map.iter().enumerate() {
        let mut new_line: Vec<char> = Vec::new();

        for (j, char) in line.iter().enumerate() {
            if !steps.contains(&(i as i32, j as i32)) {
                new_line.push(char.clone());
            }  else {
                new_line.push('O');
            }
        }

        new_map.push(new_line);
    }

    for line in new_map {
        println!("{:?}", line);
    }
    println!("");

}

fn parsing(input: &str) -> (Vec<Vec<char>>, (i32, i32), Vec<(i32, i32)>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    let mut rocks: Vec<(i32, i32)> = Vec::new();
    let mut dot_counter = 0;
    
    for (i, line) in input.lines().enumerate() {
        let mut vec_line: Vec<char> = Vec::new();

        for (j, character) in line.chars().enumerate() {
            vec_line.push(character);
            if character == 'S' {
                start = (i as i32, j as i32);
            }
            if character == '#' {
                rocks.push((i as i32, j as i32));
            }
            if character == '.' {
                dot_counter += 1;
            }
        }
        map.push(vec_line);
    }

    // println!("{}", dot_counter); //15398 + S
    (map, start, rocks)
}
