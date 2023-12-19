use std::fs;

// get every empty col and row
// if the path goes trough these the path num + 1 000 000

fn main() {
    let input = fs::read_to_string("src/in.txt").expect("Failed to read file");
    let mut rows: Vec<Vec<char>> = Vec::new();

    let mut coordinates: Vec<(usize, usize)> = Vec::new();
    let list = parse(&input, &mut rows);
    let (coordinates, empty_rows_num, empty_columns_num) = make_lists(&list);
    
    // count the pair distances
    let mut sums: Vec<usize> = Vec::new(); 
    for (i, (a, b)) in coordinates.iter().enumerate(){
        for (j, (x, y)) in coordinates.iter().enumerate(){
            
            if i == j {
                continue;
            }

            let mut x_c = 0;
            let mut y_c = 0;
            let mut add = 0;

            // Calculate path length and check for empty rows
            if a > x {
                x_c = a - x;
                for i in *x..*a {
                    if empty_rows_num.contains(&i) {
                        add += 999_999;
                    }
                }
            } else {
                x_c = x - a;
                for i in *a..*x {
                    if empty_rows_num.contains(&i) {
                        add += 999_999;
                    }
                }
            }

            // Calculate path length and check for empty columns
            if b > y {
                y_c = b - y;
                for j in *y..*b {
                    if empty_columns_num.contains(&j) {
                        add += 999_999;
                    }
                }
            } else {
                y_c = y - b;
                for j in *b..*y {
                    if empty_columns_num.contains(&j) {
                        add += 999_999;
                    }
                }
            }
            let pair_sum = x_c + y_c + add;

            sums.push(pair_sum);
            }
    }
    let total_sum: usize = sums.iter().sum();
    println!("{}", total_sum / 2)
}

fn make_lists(list: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>){
    let mut coordinates: Vec<(usize, usize)> = Vec::new();
    let mut empty_rows_num: Vec<(usize)> = Vec::new();
    let mut empty_columns_num: Vec<(usize)> = Vec::new();

    // coordinates
    for (i, _) in list.iter().enumerate(){
        for (j, _) in list[i].iter().enumerate(){
            if list[i][j] != '.' {
                let cor = (i, j);
                coordinates.push(cor);
            }
        }
    }

    // Rows
    for (i, row) in list.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows_num.push(i);
        }
    }

    // Columns
    for j in 0..list[0].len() {
        if list.iter().all(|row| row[j] == '.') {
            empty_columns_num.push(j);
        }
    }

    (coordinates, empty_rows_num, empty_columns_num)
}

fn parse<'a>(input: &'a str, rows: &'a mut Vec<Vec<char>>) -> &'a mut Vec<Vec<char>> {
    //rows
    for line in input.lines() {
        let row_chars: Vec<char> = line.chars().collect();
        rows.push(row_chars.clone());

        // if row_chars.iter().all(|&c| c == '.') {
        //     rows.push(row_chars);
        // }
    }

    rows
    // //columns
    // let mut columns: Vec<Vec<char>> = Vec::new();
    // for i in 0..rows[0].len()  { // slow iter == col iter == row length

    //     let mut col_vec: Vec<char> = Vec::new();
    //     for j in 0..rows.len()  { //fast iter  == row iter == map size
    //         col_vec.push(rows[j][i])
    //     }
    //     if col_vec.iter().all(|&c| c == '.') {
    //         columns.push(col_vec.clone());
    //     }

    //     columns.push(col_vec);
    // }

    // // Transpose the columns
    // let transposed_columns: Vec<Vec<char>> = (0..columns[0].len())
    //     .map(|i| columns.iter().map(|col| col[i]).collect())
    //     .collect();

    // // Do something with the transposed columns
    // transposed_columns
}
