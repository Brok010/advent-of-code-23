fn main() {
    let input = include_str!("in.txt");
    let (char_list, int_list) = parsing(input);

    if char_list.len() != int_list.len() {
        panic!("Character list and integer list lengths are not equal");
    }

    for i in 0..char_list.len() {
        let conf_num = get_variations(char_list[i],int_list[i]);
    }
}

fn get_variations(chars: Vec<char>, nums: Vec<usize>) -> usize {
    if nums.is_empty() {
        return 0;
    }
    
    let mut result = 0;
    
    //recursive somehow solution - idk



    result
}


fn parsing(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut char_list: Vec<Vec<char>> = Vec::new();
    let mut int_list: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let mut row_chars: Vec<char> = Vec::new();
        let mut row_ints: Vec<usize> = Vec::new();
        let mut current_word = String::new();

        for c in line.chars() {
            if c.is_digit(10) {
                // Add the digit to the current word
                current_word.push(c);
            } else if !current_word.is_empty() {
                // Parse the integer and add to the integer list
                let num: usize = current_word.parse().expect("Failed to parse integer");
                row_ints.push(num);
                current_word.clear();
            }
            if !c.is_whitespace() && !c.is_digit(10) {
                // Add character to the character list
                row_chars.push(c);
            }
        }

        // Check if there's a pending integer at the end of the line
        if !current_word.is_empty() {
            let num: usize = current_word.parse().expect("Failed to parse integer");
            row_ints.push(num);
        }

        char_list.push(row_chars);
        int_list.push(row_ints);
    }

    (char_list, int_list)
}
