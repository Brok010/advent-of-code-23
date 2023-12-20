
fn main() {
    let input = include_str!("in.txt");
    let fields = parse(input);
    let mut part_sum = 0;
    let mut rows_above_mirrage = 0;
    let mut col_left_of_mirage = 0;
    let trans_fields: Vec<Vec<Vec<char>>> = transpose_fields(&fields);
    
    // check rows - for part 2 we only check horizontal lines?
    for field in &fields {
        rows_above_mirrage += field_result(&field);
    }
    
    //cols
    for field in &trans_fields {
        col_left_of_mirage += field_result(&field);
    }

    part_sum += rows_above_mirrage * 100 + col_left_of_mirage;

    println!("{}", part_sum)

    //columns
}


fn field_result(field: &Vec<Vec<char>>) -> usize {
    let mut rows_above_mirrage = 0;

    for (i, row) in field.iter().enumerate() {
        if i <= field.len() - 2{
            let different_symbols = different_symbols(&field[i], &field[i + 1]);
            if different_symbols == 0 { // outside
                if check_row_mirage(i, field, 0) == (1, 1) {
                    rows_above_mirrage += i + 1;
                    break;
                }
            } else if  different_symbols == 1 {
                if check_row_mirage(i, field, 1) == (1, 1) {
                    rows_above_mirrage += i + 1;
                    break;
                }
            }
        }
    }
    if rows_above_mirrage == 0 {
        println!("fuck")
    }
    rows_above_mirrage
}

fn check_row_mirage(i: usize, field: &Vec<Vec<char>>, smudge: usize) -> (i8, usize) {
    // i and i + 1 are the same
    let mut x = i;
    let mut c = 1;
    let mut smudge_check = smudge;
    
    if smudge == 1 {
        while c <= x && x + c < field.len() - 1 { // if x-c would be bellow 0
            if different_symbols(&field[x - c], &field[x + c + 1]) != 0 {
                return (0 , smudge_check);
            }
            c += 1;
        }
        return (1, smudge_check);
    } else { //if smudge == 0 - found smae rows without using the smudge
        while c <= x && x + c < field.len() - 1 { 
            // if we can use smudge we use it
            if different_symbols(&field[x - c], &field[x + c + 1]) == 1 && smudge_check == 0 {
                smudge_check += 1;
            // if there is no more smudge to use we done
            } else if different_symbols(&field[x - c], &field[x + c + 1]) != 0 {
                    return (0, smudge_check);
            }
            c += 1;
        }
        return (1, smudge_check);
    }
}
fn transpose_fields(fields: &Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut transposed = Vec::new();

    
    for field in fields{
        let mut new_field: Vec<Vec<char>> = Vec::new();
        
        for i in 0..field[0].len() { //slow
            let mut new_row: Vec<char> = Vec::new();

            for j in 0..field.len() { //fast
                new_row.push(field[j][i]);
            }

            new_field.push(new_row);
        }

        transposed.push(new_field)
    }

    transposed
}

fn different_symbols(row1: &Vec<char>, row2: &Vec<char>) -> i32 {
    let mut differences = 0;

    for i in 0..row1.len(){
        if row1[i] != row2[i]{
            differences += 1;
        }
    }

    differences
}



fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut fields = Vec::new();
    let mut current_field = Vec::new();
    
    for line in input.lines() {
        if line.trim().is_empty() {
            if !current_field.is_empty() {
                fields.push(current_field);
                current_field = Vec::new();
            }
        } else {
            let chars: Vec<char> = line.chars().collect();
            current_field.push(chars);
        }
    }

    if !current_field.is_empty() {
        fields.push(current_field);
    }

    fields
}
