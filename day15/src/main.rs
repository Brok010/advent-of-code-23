
fn main() {
    let input = include_str!("in.txt");
    let parts = input.split(',');
    let mut hashed_list: Vec<i32> = Vec::new();


    // part 1

    for part in parts.clone(){
        let mut hash_value = 0;

        for character in part.chars() {
            hash_value += character as i32;
            hash_value = hash_value * 17 % 256;

        }

        hashed_list.push(hash_value);
    }


    // part2

    let mut boxes: Vec<Vec<(String, i32)>> = vec![Vec::new(); 256];
    for part in parts{
        let mut label_value = 0; // box number
        let mut label = String::new();
        let mut focal_length = 0;

        for character in part.chars() {
            
            if character.is_alphabetic(){
                label_value += character.clone() as i32;
                label_value = label_value * 17 % 256;
                label.push(character);

            } else if character == '-' {
                let box_num = label_value as usize;
                if boxes[box_num].len() > 0 {
                    for i in 0..boxes[box_num].len() {
                        let current_label = boxes[box_num][i].0.clone();
                        if current_label == label {
                            boxes[box_num].remove(i);
                            break;
                        }
                    }
                }

            } else if character.is_numeric() { // if there is number there is =, seems like there are just 1 digit nums
                let box_num = label_value as usize;
                focal_length = character.to_digit(10).unwrap() as i32;
                
                // if there is something in
                let mut found_flag = 0;
                if boxes[box_num].len() > 0 {
                    for i in 0..boxes[box_num].len(){
                        if boxes[box_num][i].0 == label {
                            found_flag = 1;
                            boxes[box_num][i].0 = label.clone();
                            boxes[box_num][i].1 = focal_length.clone();

                        }
                    }
                }
                if found_flag != 1 {
                    boxes[box_num].push((label.clone(), focal_length.clone()))
                }
            }
        }
    }

    let mut part2 = 0;

    for i in 0..boxes.len(){
        if boxes[i].len() > 0 {
            for j in 0..boxes[i].len(){
                let focal_length =  boxes[i][j].1;
                part2 += (i + 1) as i32 * (j + 1) as i32 * focal_length;
            }
        }
    }

    let part1: i32 = hashed_list.iter().sum();
    println!("Result Value: {:?}", part1);
    println!("Result Value2: {}", part2);
}
