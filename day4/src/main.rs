fn main() {
    let mut part1_count = 0;
    let mut card_count: Vec<i32> = Vec::new();

    let content = include_str!("in.txt");
    let (winning_numbers_list, my_numbers_list) = parsing(&content);
    
    // fill the card_count with ones
    for each in 0..winning_numbers_list.len(){
        card_count.push(1);
    }

    //card
    for i in 0..winning_numbers_list.len(){
        let mut line_count_part_1 = 0;
        let mut line_matches = 0;

        // num
        for j in 0..winning_numbers_list[i].len(){
            let checked_for_num = winning_numbers_list[i][j];
            
            //check the same line for the checked for number
            for k in 0..my_numbers_list[i].len(){
                // if we have a match
                if checked_for_num == my_numbers_list[i][k]{
                    
                    //part2
                    line_matches += 1;

                    // part1
                    if line_count_part_1 == 0 {
                        line_count_part_1 += 1;
                    }
                    else {
                        line_count_part_1 = line_count_part_1 * 2;
                    }
                }
            }
        }
        // line matches = times i have to add to next (card * number of cards)
        while  line_matches > 0 {
            
            card_count[i + line_matches] += 1 * card_count[i];

            line_matches -= 1;
        };

        part1_count += line_count_part_1;
    }
    let card_count_sum: i32 = card_count.iter().sum();
    
    println!("Part1: {}", part1_count);
    println!("Part2: {}", card_count_sum);
}

fn parsing(content: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>){
    let mut winning_numbers_list: Vec<Vec<i32>> = Vec::new();
    let mut my_numbers_list: Vec<Vec<i32>> = Vec::new();

    for line in content.lines() {
        let mut parts = line.split('|').map(|part| part.trim());

        if let (Some(left_part), Some(right_part)) = (parts.next(), parts.next()) {
            let left_numbers: Vec<i32> = left_part
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();
            let right_numbers: Vec<i32> = right_part
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            winning_numbers_list.push(left_numbers);
            my_numbers_list.push(right_numbers);
        }
    }

    // Print the lists for demonstration
    println!("Left Numbers List: {:?}", winning_numbers_list);
    println!("Right Numbers List: {:?}", my_numbers_list);

    (winning_numbers_list, my_numbers_list)
}
