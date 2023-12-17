extern crate num_integer;
use num_integer::lcm;

#[derive(Debug)]
struct Directions<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn main() {
    let input = include_str!("in.txt");

    let mut turns: Vec<char> = Vec::new();
    let mut structs: Vec<Directions> = Vec::new();
    (structs, turns) = parsing(&input);
    // part1(&structs, &turns);
    part2(&structs, &turns);
}
fn part2 (structs: &Vec<Directions>, turns: &Vec<char>) {
    
    let mut start: Vec<&str> = Vec::new();
    for each in structs{
        if each.name.ends_with('A') {
            start.push(each.name);
        }
    }
    
    let mut current_positions = start;
    let mut counter = 0;
    let mut counter_of_directions = 1;
    let turns_length = turns.len();
    let mut z_repetitions: Vec<i32> = vec![0; current_positions.len()];

    while z_repetitions.contains(&0) {
        
        //ensures looping of turns
        if counter == turns_length {
            counter -= turns_length;
        }
        
        for position in &mut current_positions{
            for direction in structs{
                if direction.name == *position{
    
                    let turn = turns[counter];
                    if turn == 'L'{
                        *position = direction.left;
                    }
                    if turn == 'R' {
                        *position = direction.right;
                    }
                    break;
                }
            }
        }

        let updated_positions = current_positions.clone();
        for updated_position in &updated_positions {
            if updated_position.ends_with('Z') {
                if let Some(index) = current_positions.iter().position(|&x| x == *updated_position) {
                    if z_repetitions[index] == 0 {
                        z_repetitions[index] = counter_of_directions;
                    }
                }
            }
        }
        counter += 1;
        counter_of_directions += 1;
    }

    let result: i64 = z_repetitions.iter().fold(1, |acc, &x| lcm(acc, x.into()));

    println!("LCM: {}", result);
}  

fn part1 (structs: &Vec<Directions>, turns: &Vec<char>) {
    let start = "AAA";
    let stop = "ZZZ";
    let mut current_position: &str = start;
    let mut counter = 0;
    let mut counter_of_directions = 0;
    let turns_length = turns.len();

    while current_position != stop {

        for direction in structs{
            if direction.name == current_position{
                
                //ensures looping of turns
                if counter == turns_length {
                    counter -= turns_length;
                }


                let turn = turns[counter];
                if turn == 'L'{
                    current_position = direction.left;
                }
                if turn == 'R' {
                    current_position = direction.right;
                }

                counter += 1;
                counter_of_directions += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", counter_of_directions)

}

fn parsing(input: &str) -> (Vec<Directions>, Vec<char>){
    let mut turns: Vec<char> = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        turns.extend(line.trim().chars());
    }

    let structs: Vec<Directions> = lines
        .filter(|line| !line.trim().is_empty())
        .map(|turn| {
            let name = &turn[0..3];
            let left = &turn[7..10];
            let right = &turn[12..15];

            Directions {
                name,
                left,
                right,
            }
        })
        .collect();

    // Print the result
    // println!("{:?}", structs);
    // println!("{:?}", turns);

    (structs, turns)
}

