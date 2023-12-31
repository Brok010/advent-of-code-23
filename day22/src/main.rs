#[derive(Debug, Clone, PartialEq)]
struct Brick {
    positions: Vec<Position>
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let input = include_str!("in.txt");
    let bricks = parsing(input);

    // colapse the bricks then deside which we can remove - part1
    let collapsed_bricks = collapse_bricks(&bricks);
    println!("collapsed bricks made");

    // part 1
    // let deletable = count_deletable(&collapsed_bricks);
    // println!("{}", deletable);

    // part 2 - build a graph of supporting bricks
    // go from top to bottom if any brick above without additional support; weight += 1 + the bricks weight
    let graph = create_graph(&collapsed_bricks);
    let graph_sum: i32 = graph.iter().map(|&(_, _, value)| value).sum();
    println!("p2: {}", graph_sum)
}

fn create_graph(collapsed_bricks: &Vec<Brick>) -> Vec<(Brick, Vec<Brick>, i32)> {
    // |_____| |__x__| |_____|
    //     |_____| |_____|
    //         |_____|

    // make a graph of what is supported by what
    // if all supports gone then i can add weight

    let mut graph: Vec<(Brick, Vec<Brick>, i32)> = Vec::new();
    
    // get supports
    for brick in collapsed_bricks {
        let positions_below = get_below_positions(&brick);
        let bricks_below = get_bricks_in_positions(positions_below, collapsed_bricks);
        graph.push((brick.clone(), bricks_below, 0)); 
    }
    // get weights
    // for each brick - simulate remove, if any brick has all its supports removed, remove and repeat
    // for each removed brick, weight += 1;
    let graph_supports_copy = graph.clone();
    for (current_brick, _, weight) in &mut graph {
        let mut removed: Vec<Brick> = Vec::new();
        removed.push(current_brick.clone());

        let mut found_flag = true;
        while found_flag == true {

            found_flag = false;
            for (following_brick, supports, _) in &graph_supports_copy {
                // println!("{:?}", supports);
                if supports.iter().all(|support| removed.contains(support)) && !supports.is_empty() && !removed.contains(following_brick) { // not working
                    removed.push(following_brick.clone());
                    found_flag = true;
                    break;
                }
            }

        }

        *weight += removed.len() as i32 - 1; // not counting itself
    } 
    
    graph
}

fn count_deletable(collapsed_bricks: &Vec<Brick>) -> i32 {
    let mut deletable = 0;
    for brick in collapsed_bricks {

        let positions_above = get_above_positions(&brick);
        let bricks_above = get_bricks_in_positions(positions_above, &collapsed_bricks);

        //if there is no brick above its eligible for disintegration
        if bricks_above.is_empty() {
            deletable += 1;

        } else {
            let mut found_flag = true;
            for above_brick in bricks_above {
                //get bellow positions and check if there is another brick bellow the current(above) one
                // if there is another brick bellow each above brick we can deletable += 1;
                let positions_below = get_below_positions(&above_brick);
                let bricks_below = get_bricks_in_positions(positions_below, collapsed_bricks);

                if bricks_below.len() < 2 {
                    found_flag = false;
                    break;
                }
            }
            if found_flag == true { // all bricks that are above have more then 1 brick below supporting it
                deletable += 1;
            }
        }
    }

    deletable
}

fn get_bricks_in_positions(positions: Vec<Position>, collapsed_bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();
    for position in &positions {
        'outer: for above_brick in collapsed_bricks {
            for cube in &above_brick.positions {
                if cube == position && !bricks.contains(&above_brick) {
                    bricks.push(above_brick.clone());
                    break 'outer;
                }
            }
        }
    }

    bricks
}

fn get_below_positions(brick: &Brick) ->  Vec<Position> {
    let mut positions_below: Vec<Position> = Vec::new();

    // findout orientation - if min != max - not horizontal
    let min_position = brick
        .positions
        .iter()
        .min_by_key(|pos| pos.z)
        .cloned()
        .unwrap();

    let max_position = brick
        .positions
        .iter()
        .max_by_key(|pos| pos.z)
        .cloned()
        .unwrap();

    if min_position.z != max_position.z { //brick is vertical
        let mut below_pos = min_position.clone();
        below_pos.z -= 1;
        positions_below.push(below_pos);
    
    } else { // brick is horizontal
        for pos in &brick.positions {
            positions_below.push(Position {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            });
        }
    }

    positions_below
}

fn get_above_positions(brick: &Brick) -> Vec<Position> {
    let mut positions_above: Vec<Position> = Vec::new();

    // find out orientation - if min != max - not horizontal
    let min_position = brick
        .positions
        .iter()
        .min_by_key(|pos| pos.z)
        .cloned()
        .unwrap();

    let max_position = brick
        .positions
        .iter()
        .max_by_key(|pos| pos.z)
        .cloned()
        .unwrap();

    if min_position.z != max_position.z { //brick is vertical
        let mut above_pos = max_position.clone();
        above_pos.z += 1;
        positions_above.push(above_pos);

    } else { // brick is horizontal
        for pos in &brick.positions {        
            positions_above.push(Position {
                x: pos.x,
                y: pos.y,
                z: pos.z + 1,
            });
        }
    }

    positions_above
}

// can be optimized by instead of stepping 1 by 1, just finding the lowest pos i can lower the brock to
fn collapse_bricks(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut original_bricks = bricks.clone();
    let mut collapsed_bricks: Vec<Brick> = Vec::new();

    // Sort bricks according to the minimum z value of the first position in each brick
    original_bricks.sort_by_key(|brick| brick.positions.iter().map(|pos| pos.z).min().unwrap_or(0));

    while !original_bricks.is_empty() {
        // Get columns that are going down and change z according to how far down we can go
        let current_brick = original_bricks.remove(0).clone();

        let mut unfold_brick: Vec<Position> = match current_brick {
            Brick { positions } if positions[0] != positions[1] => {
                let (first, second) = (&positions[0], &positions[1]);

                if first.x != second.x {
                    (first.x..=second.x).map(|x| Position { x, y: first.y, z: first.z }).collect()
                } else if first.y != second.y {
                    (first.y..=second.y).map(|y| Position { x: first.x, y, z: first.z }).collect()
                } else { //first.z != second.z
                    (first.z..=second.z).map(|z| Position { x: first.x, y: first.y, z }).collect()
                }
            }
            _ => vec![current_brick.positions[0].clone()]
        };

        let mut lowest_z = unfold_brick.iter().map(|pos| pos.z).min().unwrap_or(1);

        while lowest_z > 1 {
            let new_positions: Vec<Position> = unfold_brick
                .iter()
                .map(|pos| Position { x: pos.x, y: pos.y, z: pos.z - 1 })
                .collect();

            if new_positions.iter().any(|pos| collapsed_bricks.iter().any(|b| b.positions.contains(pos))) {
                break;
            } else {
                unfold_brick = new_positions;
                lowest_z -= 1;
            }
        }

        // Add the unfold brick to the collapsed bricks
        collapsed_bricks.push(Brick { positions: unfold_brick });
    }

    collapsed_bricks
}


fn parsing(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();

    for line in input.lines() {
        let sides: Vec<&str> = line.split('~').collect();
        let side1: Vec<i32> = sides[0].split(',').map(|s| s.parse().unwrap()).collect();
        let side2: Vec<i32> = sides[1].split(',').map(|s| s.parse().unwrap()).collect();

        let p1 = Position {
            x: side1[0],
            y: side1[1],
            z: side1[2],
        };

        let p2 = Position {
            x: side2[0],
            y: side2[1],
            z: side2[2],
        };

        let positions = vec![p1, p2];
        let new_brick = Brick { positions };
        bricks.push(new_brick);
    }
    bricks
}