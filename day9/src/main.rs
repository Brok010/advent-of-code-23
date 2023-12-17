fn main() {
    let input = include_str!("in.txt");
    let doc = parsing(input);
    let mut results_p1: Vec<i32> = Vec::new();
    let mut results_p2: Vec<i32> = Vec::new();

    for list in doc{
        let (returned_num_part1, returned_num_part2) = create_differences_list(list.clone());
        
        let last_element = list.last().clone();
        if let Some(last_value) = last_element {
            let result_num = last_value + returned_num_part1;
            results_p1.push(result_num);
        }

        let first_element = list.first().clone();
        if let Some(first_value) = first_element {
            let result_num = first_value - returned_num_part2;
            results_p2.push(result_num);
        }


    }

    let sum1: i32 = results_p1.iter().sum();
    let sum2: i32 = results_p2.iter().sum();
    println!("{}, {}", sum1, sum2);


}

fn create_differences_list(list: Vec<i32>) -> (i32, i32){
    let mut new_list: Vec<i32> = Vec::new();
    
    for i in 0..list.len() - 1 {
        //creates the list of diferences
        new_list.push(list[i + 1] - list[i]);
    }
    if new_list.iter().any(|&x| x != 0) {
        let (returned_num_p1, returned_num_p2) = create_differences_list(new_list.clone());
        
        let last_element = new_list.last().clone();
        let first_element = new_list.first().clone();

        if let (Some(last_value), Some(first_value)) = (last_element, first_element) {
            let return_p1 = last_value + returned_num_p1;
            let return_p2 = first_value - returned_num_p2;
            return (return_p1, return_p2);
        } else {
            panic!("Error: new_list is empty");
        }
    } else {
        
        return (0, 0)
    }
}

fn parsing(input: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = input.lines().collect();

    // Parse each line into a vector of integers
    let result: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    result
}
