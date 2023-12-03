use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Game{
    id: i32,
    pulls: Vec<Vec<i32>>,
}
impl Game {
    fn new(id: i32) -> Self {
        Game { id, pulls: Vec::new() }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect(&format!("Error reading file: {}", filename))
        .lines()
        .map(String::from)
        .collect()
}

fn parse_pull(pull: &str) -> Vec<i32> {
    let colors = pull.split(", ");
    let mut sub_pull = vec![0; 3];
    
    for color in colors{
        if color.contains("red"){
            if let Some(red_index) = color.find("red") {
                if let Ok(value) = color[..red_index].trim().parse::<i32>() {
                    sub_pull[0] = value;
                }
            }
        }
        else if color.contains("green"){
            if let Some(green_index) = color.find("green") {
                if let Ok(value) = color[..green_index].trim().parse::<i32>() {
                    sub_pull[1] = value;
                }
            }
        }
        else if color.contains("blue"){
            if let Some(blue_index) = color.find("blue") {
                if let Ok(value) = color[..blue_index].trim().parse::<i32>() {
                    sub_pull[2] = value;
                }
            }
        }
    }
    sub_pull
}

fn main() {
    let filename = "input.txt";
    let lines = read_lines(filename);
    let mut games: Vec<Game> = Vec::new();
    let RED_MAX = 12;
    let GREEN_MAX = 13;
    let BLUE_MAX = 14;

    for line in lines{
        let id = line[5..line.find(':').unwrap()].parse::<i32>().unwrap();
        let mut game = Game::new(id);
        let pulls = line[line.find(':').unwrap() + 2..line.len()].split("; ");

        for pull in pulls{
            let sub_pull = parse_pull(pull);
            game.pulls.push(sub_pull.clone());
        }
        games.push(game)
    }

    let mut games_part1 = games.clone();
    let games_part2 = games.clone();

    games_part1.retain(|game| {
        !game.pulls.iter().any(|sub_pull| {
            sub_pull[0] > RED_MAX
                || sub_pull[1] > GREEN_MAX
                || sub_pull[2] > BLUE_MAX
        })
    });

    let mut min_cubes_list: Vec<[i32; 3]> = Vec::new();
    for game in games_part2{
        let mut min_cubes: [i32; 3] = [0, 0, 0];

        for pull in game.pulls{
            if pull[0] > min_cubes[0]{
                min_cubes[0] = pull[0].clone()

            }if pull[1] > min_cubes[1]{
                min_cubes[1] = pull[1].clone()

            }if pull[2] > min_cubes[2]{
                min_cubes[2] = pull[2].clone()

            }   
        }
        min_cubes_list.push(min_cubes); 
    }

    let min_cubes_list_multiplied: Vec<i32> = min_cubes_list
        .iter()
        .map(|each| each[0] * each[1] * each[2])
        .collect();
    let sum_of_results: i32 = min_cubes_list_multiplied.iter().sum();
    println!("part2: {}", sum_of_results);

    let sum_of_ids: i32 = games_part1.iter().map(|game| game.id).sum();
    println!("part1: {}", sum_of_ids);
}
