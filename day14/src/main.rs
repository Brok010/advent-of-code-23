
fn main() {
    let input = include_str!("in.txt");
    let field = parse(input);

    //save the positions of rocks
    let mut o_rocks: Vec<(usize, usize)> = Vec::new();
    let mut hash_rocks: Vec<(usize, usize)> = Vec::new();

    for (i, _) in field.iter().enumerate(){
        for (j, _) in field[i].iter().enumerate(){
            let current_symbol = field[i][j];
            
            if current_symbol == 'O' {
                o_rocks.push((i, j))
            }else if current_symbol == '#' {
                hash_rocks.push((i, j))
            }
        }
    }
    
    let mut cycles = 0;
    let field_max_col = field[0].len();
    let field_max_row = field.len();
    let mut memory: Vec<(Vec<(usize, usize)>, usize)> = Vec::new();

    while cycles < 1000 {
        // one cycle
        for i in 0..=3{
            match i {
                0 =>{
                    o_rocks.sort_by(|a, b| a.0.cmp(&b.0));
                    o_rocks = board_titl_north(&o_rocks, &hash_rocks);
                }
                1 =>{
                    o_rocks.sort_by(|a, b| a.1.cmp(&b.1));
                    o_rocks = board_titl_east(&o_rocks, &hash_rocks);
                }
                2 =>{
                    o_rocks.sort_by(|a, b| b.0.cmp(&a.0));
                    o_rocks = board_titl_south(&o_rocks, &hash_rocks, field_max_row);
                }
                3 =>{
                    o_rocks.sort_by(|a, b| b.1.cmp(&a.1));
                    o_rocks = board_titl_west(&o_rocks, &hash_rocks, field_max_col);
                }
                _ => panic!("Unexpected combination of cycle and rock_col values"),
            }
            // println!("{}", i);
            // for each in o_rocks{
            //     println!("{:?}", each);
            // }
        }
        
        // for each in &o_rocks{
            // println!("{:?}", each)
        // }

        //store each result as a string if a repetition is found - cycle
        let mut load = 0;
        for each in &o_rocks{
            load += field_max_row - each.0
        }
        
        if let Some((_index, existing_cycles)) = memory.iter().enumerate().find(|&(_, (ref rocks, _))| *rocks == o_rocks) {
            // Print the cycles of the duplicate state
            println!("Duplicate state found at cycles: {} and existing cycles: {} and load: {}", cycles, existing_cycles.1, load);
            //break;  // Exit the loop or handle as needed
        }

        // Push the final_string and cycles into the strings vector
        memory.push((o_rocks.clone(), cycles));
        cycles += 1;
    }


}

fn board_titl_north(o_rocks: &Vec<(usize, usize)>, hash_rocks: &Vec<(usize, usize)>) ->  Vec<(usize, usize)> {
    let mut new_o_rocks = o_rocks.clone(); 
    
    for i in 0..new_o_rocks.len() {
        let mut new_row = 0;
        let rock_row = new_o_rocks[i].0;
        let rock_col = new_o_rocks[i].1;

        for hash in hash_rocks {
            if hash.0 < rock_row && hash.1 == rock_col && hash.0 >= new_row {
                new_row = hash.0 + 1;
            }
        }
        for o_rock in &new_o_rocks{
            if o_rock == &new_o_rocks[i]{
                continue;
            }

            if o_rock.0 < rock_row && o_rock.1 == rock_col && o_rock.0 >= new_row { // its above the last barrier{
                new_row += 1;
            } 

        }

        new_o_rocks[i].0 = new_row;

    }

    new_o_rocks
}
fn board_titl_east(o_rocks: &Vec<(usize, usize)>, hash_rocks: &Vec<(usize, usize)>) ->  Vec<(usize, usize)> {
    let mut new_o_rocks = o_rocks.clone(); 
    
    for i in 0..new_o_rocks.len() {
        let mut new_col = 0;
        let rock_row = new_o_rocks[i].0;
        let rock_col = new_o_rocks[i].1;

        for hash in hash_rocks {
            if hash.1 < rock_col && hash.0 == rock_row && hash.1 >= new_col {
                new_col = hash.1 + 1;
            }
        }
        for o_rock in &new_o_rocks{
            if o_rock == &new_o_rocks[i]{
                continue;
            }

            if o_rock.1 < rock_col && o_rock.0 == rock_row && o_rock.1 >= new_col { // its above the last barrier{
                new_col += 1;
            } 

        }

        new_o_rocks[i].1 = new_col;

    }

    new_o_rocks
}
fn board_titl_south(o_rocks: &Vec<(usize, usize)>, hash_rocks: &Vec<(usize, usize)>, field_max_row: usize) ->  Vec<(usize, usize)> {
    let mut new_o_rocks = o_rocks.clone(); 
    
    for i in 0..new_o_rocks.len() {
        let mut new_row = field_max_row - 1;
        let rock_row = new_o_rocks[i].0;
        let rock_col = new_o_rocks[i].1;

        for hash in hash_rocks {
            if hash.0 > rock_row && hash.1 == rock_col && hash.0 <= new_row {
                if hash.0 >= 1 {
                    new_row = hash.0 - 1;
                }                
            }
        }
        for o_rock in &new_o_rocks{
            if o_rock == &new_o_rocks[i]{
                continue;
            }

            if o_rock.0 > rock_row && o_rock.1 == rock_col && o_rock.0 <= new_row { // its above the last barrier{
                new_row -= 1;
            } 

        }

        new_o_rocks[i].0 = new_row;

    }

    new_o_rocks
}
fn board_titl_west(o_rocks: &Vec<(usize, usize)>, hash_rocks: &Vec<(usize, usize)>, field_max_col: usize) ->  Vec<(usize, usize)> {
    let mut new_o_rocks = o_rocks.clone(); 
    
    for i in 0..new_o_rocks.len() {
        let mut new_col = field_max_col - 1;
        let rock_row = new_o_rocks[i].0;
        let rock_col = new_o_rocks[i].1;

        for hash in hash_rocks {
            if hash.1 > rock_col && hash.0 == rock_row && hash.1 <= new_col {
                if hash.1 >= 1 {
                    new_col = hash.1 - 1;
                }
            }
        }
        for o_rock in &new_o_rocks{
            if o_rock == &new_o_rocks[i]{
                continue;
            }

            if o_rock.1 > rock_col && o_rock.0 == rock_row && o_rock.1 <= new_col { // its above the last barrier{
                new_col -= 1;
            } 

        }

        new_o_rocks[i].1 = new_col;

    }

    new_o_rocks

}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut field: Vec<Vec<char>> = Vec::new();

    for line in input.lines(){
        let mut list:Vec<char> = Vec::new();

        for char in line.chars(){
            list.push(char.clone());
        }
        field.push(list.clone())
    }

    field
}