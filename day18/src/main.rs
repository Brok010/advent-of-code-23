use std::fs;

#[derive(Debug)]
struct Dig{
    direction: char,
    value: i32,
    color: String,

}

fn main() {
    let input = include_str!("in.txt");
    let list = parsing(input);
    let list2 = make_list2(&list);
    // println!("{:?}", list2);

    let hole_list = make_hole_list(&list, 1);
    let circumference_length = make_hole_list(&list2, 1).len();
    let point_list2 = make_hole_list(&list2, 2);
    println!("{:?}, {}", point_list2, circumference_length);
    
    // part 2
    let result = shoelace_method(&point_list2) + (circumference_length as i64 / 2) + 1; //idk where did the 1 came from
    println!("part2 result {}", result);

    // part 1
    // find a point that is inside the shape
    let loc_inside = get_inside_pos(&hole_list); //since 0,0 is edge for sure it cant be inside
    if loc_inside != (0, 0) {
        let result = count_inside_blocks(loc_inside, &hole_list) + hole_list.len() as i32;
        println!("{}", result)
    }
}

fn shoelace_method(points: &Vec<(i32, i32)>) -> i64 {
    let mut result = 0;
    let n = points.len();

    for i in 0..n - 1 {
        let pos1 = points[i];
        let pos2 = points[i + 1];
        result += (pos1.0 as i64 * pos2.1 as i64) - (pos2.0 as i64 * pos1.1 as i64);
    }

    // Add the contribution of the last and first points
    let first_pos = points[0];
    let last_pos = points[n - 1];
    result += (last_pos.0 as i64 * first_pos.1 as i64) - (first_pos.0 as i64 * last_pos.1 as i64);

    // Take the absolute value and divide by 2
    result.abs() / 2
}

fn make_list2(list: &Vec<Dig>) -> Vec<Dig> {
    let mut new_list: Vec<Dig> = Vec::new();
    for item in list {
        let hex_color = item.color.clone();
        let value = parse_hex_to_decimal(&hex_color[1..6]) as i32;
        let last_number = hex_color.chars().last().unwrap().to_digit(10).unwrap();
        let mut direction = ' ';
        match last_number {
            0 => direction = 'R',
            1 => direction = 'D',
            2 => direction = 'L',
            3 => direction = 'U',
            _ => println!("weird last digit")
        };
        let color: String = String::new();
        let new_item = Dig {
            direction,
            value,
            color,
        };
        new_list.push(new_item);
    }

    new_list
}

fn parse_hex_to_decimal(hex_string: &str) -> u32 {
    u32::from_str_radix(hex_string, 16).unwrap()
}

fn get_inside_pos(hole_list: &Vec<(i32, i32)>) -> (i32, i32){
    //get map edges
    let mut loc_inside = (0, 0);
    let (min_first, max_first) = min_max_tuple_values(hole_list.iter().map(|&(x, _)| x)); // row
    let (min_second, max_second) = min_max_tuple_values(hole_list.iter().map(|&(_, y)| y)); // col

    'outer: for i in min_first..max_first + 1 { //isnt optimal but works
        for j in min_second..max_second - 1 {
            if !hole_list.contains(&(i, j)) && hole_list.contains(&(i, j + 1)) && !hole_list.contains(&(i, j + 2)) {
                loc_inside = (i, j + 2);
                break 'outer;
            }
        }
    }
    // draw_map(min_first, max_first, min_second, max_second, loc_inside.clone(), &hole_list);
    println!("found loc_inside {:?}", loc_inside);
    loc_inside
}

fn draw_map(min_first: i32, max_first: i32, min_second: i32, max_second: i32, loc_inside: (i32, i32), hole_list: &Vec<(i32, i32)>) {
    // draw map
    let mut draw_map: Vec<Vec<char>> = Vec::new();
    for i in min_first..max_first + 1 {
        let mut line: Vec<char> = Vec::new();
        for j in min_second..max_second + 1 {
            if (i, j) == loc_inside {
                line.push('O')

            } else if hole_list.contains(&(i, j)) {
                line.push('#');
            } else {
                line.push('.');

            }
        }
        draw_map.push(line)
    }

    let map_string: String = draw_map
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    // Write the map to a text file
    if let Err(err) = fs::write("output.txt", map_string) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("Map written to output.txt");
    }
}

fn count_inside_blocks(pos1: (i32, i32), hole_list: &Vec<(i32, i32)>) -> i32 {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut unvisited: Vec<(i32, i32)> = Vec::new();
    unvisited.push(pos1);

    while !unvisited.is_empty() {
        let current_pos = unvisited.remove(0);
        visited.push(current_pos.clone());
        let neighbors = get_adjacent(current_pos, hole_list, &mut visited);
        for n in neighbors {
            if !unvisited.contains(&n) {
                unvisited.push(n);
            }
        }
    }
    return visited.len() as i32;
}

fn get_adjacent(pos1: (i32, i32), hole_list: &Vec<(i32, i32)>, visited: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let dirs = [(1, 0),(-1, 0),(0, 1),(0, -1)]; // n s e w
    let mut available_neighbors: Vec<(i32, i32)> = Vec::new();
    for dir in dirs.iter() {
        let new_pos = add_tuple(pos1, *dir);

        if !hole_list.contains(&new_pos) && !visited.contains(&new_pos) {
            available_neighbors.push(new_pos);
        }
    }
    available_neighbors
}

fn min_max_tuple_values<I>(iter: I) -> (i32, i32)
where
    I: Iterator<Item = i32>,
{
    let mut iter = iter.peekable();
    if iter.peek().is_none() {
        // Handle empty iterator case
        return (0, 0);
    }

    let mut min_val = i32::MAX;
    let mut max_val = i32::MIN;

    for val in iter {
        if val < min_val {
            min_val = val;
        }
        if val > max_val {
            max_val = val;
        }
    }

    (min_val, max_val)
}

fn make_hole_list(list: &Vec<Dig>, part: i32) -> Vec<(i32, i32)> {
    let mut current_pos = (0, 0);
    let mut list_of_holes: Vec<(i32, i32)> = Vec::new();
    if part == 2 {
        list_of_holes.push(current_pos);
    }

    for each in list{
        let dir = each.direction;
        let value = each.value;
        let mut current_dir = (0, 0);
        
        match dir {
            'U' => current_dir = (-1, 0),
            'D' => current_dir = (1, 0),
            'R' => current_dir = (0, 1),
            'L' => current_dir = (0, -1),
            _ => panic!("wrong direction"),
        }
        if part == 1 {
            for _ in 0..value {
                current_pos = add_tuple(current_pos,current_dir);
                list_of_holes.push(current_pos);
            }
        } else if part == 2 {
            current_dir = multiply_tuple(current_dir, value);
            current_pos = add_tuple(current_pos, current_dir);
            list_of_holes.push(current_pos);
        }
        
    }

    list_of_holes
}

fn multiply_tuple(t1: (i32, i32), value: i32) -> (i32, i32) {
    return (t1.0 * value, t1.1 * value);
}

fn add_tuple(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    return (t1.0 + t2.0, t1.1 + t2.1);
}


fn parsing(input: &str) -> Vec<Dig> {
    let mut list: Vec<Dig> = Vec::new();

    for line in input.lines() {
        // Split the line into components
        let mut iter = line.split_whitespace();

        // Extract the direction (first letter)
        let direction = iter.next().unwrap().chars().next().unwrap();

        // Extract the value
        let value: i32 = iter.next().unwrap().parse().unwrap();

        // Extract the color
        let color_with_parentheses = iter.next().unwrap();
        let color = color_with_parentheses[1..color_with_parentheses.len() - 1].to_string();

        // Create a Dig struct and push it to the list
        let dig = Dig {
            direction,
            value,
            color,
        };
        list.push(dig);
    }
    list
}