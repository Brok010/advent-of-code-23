struct Combinations {
    five_a_kind: Vec<(Vec<char>, i32)>,
    four_a_kind: Vec<(Vec<char>, i32)>,
    fullhouse: Vec<(Vec<char>, i32)>,
    three_a_kind: Vec<(Vec<char>, i32)>,
    two_pairs: Vec<(Vec<char>, i32)>,
    two_a_kind: Vec<(Vec<char>, i32)>,
    no_combination: Vec<(Vec<char>, i32)>,
}

fn main() {
    let input = include_str!("in.txt");
    let mut hands: Vec<Vec<char>> = Vec::new();
    let mut values: Vec<i32> = Vec::new();
    (hands, values) = parsing(input);

    let mut ranks: Vec<i32> = Vec::with_capacity(hands.len());
    ranks.resize(hands.len(), 0);

    let mut max_rank = values.len() as i32;

    //dividing
    let mut combinations = Combinations {
    five_a_kind: Vec::new(),
    four_a_kind: Vec::new(), 
    fullhouse: Vec::new(),
    three_a_kind: Vec::new(), 
    two_pairs: Vec::new(),
    two_a_kind: Vec::new(), 
    no_combination: Vec::new()
    };

    for (i, hand) in hands.iter().enumerate() {       
        
        //if hand has all same
        if all_the_same(&hand) {
            combinations.five_a_kind.push((hand.clone(), values[i]));
        }

        // if 4 are the same
        else if contains_x_same_signs(4,&hand) {
            combinations.four_a_kind.push((hand.clone(), values[i]));
        }
        
        //if fullhouse
        else if contains_fullhouse(&hand) {
            combinations.fullhouse.push((hand.clone(), values[i]));
        }

        // if 3 are the same
        else if contains_x_same_signs(3,&hand) {
            combinations.three_a_kind.push((hand.clone(), values[i]));
        }
        
        // if 2 pairs
        else if contains_two_pairs(&hand) {
            combinations.two_pairs.push((hand.clone(), values[i]));
        }

        // if a pair
        else if contains_x_same_signs(2,&hand) {
            combinations.two_a_kind.push((hand.clone(), values[i]));
        }
        else {
            combinations.no_combination.push((hand.clone(), values[i]));
        }
    }
    

    //order and evaluate each list
    let mut part1_result = 0;
    let mut partial_result = 0;
    (max_rank, partial_result) = evaluate_list(&mut combinations.five_a_kind, max_rank);
    part1_result += partial_result as i128;

    (max_rank, partial_result) = evaluate_list(&mut combinations.four_a_kind, max_rank);
    part1_result += partial_result as i128;

    (max_rank, partial_result) = evaluate_list(&mut combinations.fullhouse, max_rank);
    part1_result += partial_result as i128;
    
    (max_rank, partial_result) = evaluate_list(&mut combinations.three_a_kind, max_rank);
    part1_result += partial_result as i128;

    (max_rank, partial_result) = evaluate_list(&mut combinations.two_pairs, max_rank);
    part1_result += partial_result as i128;

    (max_rank, partial_result) = evaluate_list(&mut combinations.two_a_kind, max_rank);
    part1_result += partial_result as i128;

    (max_rank, partial_result) = evaluate_list(&mut combinations.no_combination, max_rank);
    part1_result += partial_result as i128;
    
    println!("{}", part1_result);



}


fn evaluate_list(list: &mut Vec<(Vec<char>, i32)>, max_rank: i32) -> (i32, i128) {
    let mut value_list: Vec<(Vec<i32>, i32)> = Vec::new();
    
    for item in list.iter_mut(){
        // this gives me evaluated card with its bet
       value_list.push(((get_value(&item.0)), item.1.clone()));
    }

    value_list.sort_by(|a, b| {
        for i in 0..a.0.len() {
            match b.0[i].cmp(&a.0[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        // If all elements are equal, compare based on the second element (list[1]) in reverse order
        b.1.cmp(&a.1)
    });

    // Print the sorted tuples
    let mut result: i128 = 0;
    let mut rank = max_rank.clone();
    for tuple in &value_list {
        let hand_result = tuple.1 * rank;
        result += hand_result as i128;
        rank -= 1;
    }
    (rank, result)
}

fn get_value(item: &Vec<char>) -> Vec<i32>{
    let cards: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    let mut values: Vec<i32> = vec![0; 5];

    for (i, char) in item.iter().enumerate() {
        for (j, sign) in (0..cards.len()).rev().enumerate() {
            if *char == cards[sign] {
                values[i] = j as i32;
            }
        }
    }
    values
}

fn contains_two_pairs(hand: &Vec<char>) -> bool{
    let mut counter1 = 1;
    let mut counter2 = 1;
    let mut counter3 = 1;

    for i in 0..hand.len(){
        
        if hand[0] == hand[i] && i != 0{
            counter1 += 1;
        }
        if hand[1] == hand[i] && i != 1{
            counter2 += 1;
        }
        if hand[2] == hand[i] && i != 2{
            counter3 += 1;
        }
    }
    
    if (counter1 == 2 && counter2 == 2) || (counter1 == 2 && counter3 == 2) || (counter2 == 2 && counter3 == 2) {
        return true;
    } else {
        return false;
    }
}

fn contains_fullhouse(hand: &Vec<char>) -> bool{
    if contains_x_same_signs(3, &hand) && contains_x_same_signs(2, &hand){
        return true
    }
    else {
        return false
    }
}

fn all_the_same(hand: &Vec<char>) -> bool {
    let sign = hand[0];
    for i in 1..hand.len(){
        if sign!=hand[i]{
            return false
        }
    }
    return true;
}

fn contains_x_same_signs(num: i32, hand: &Vec<char>) -> bool {

    for i in 0..hand.len(){
        let mut counter = 1;

        for j in 0..hand.len(){
            if i == j {
                continue;
            }
            if hand[i] == hand[j] {
                counter += 1;
            }
        }

        if counter == num {
            return true;
        }
    }
    return false;
}

fn parsing(input: &str) -> (Vec<Vec<char>>, Vec<i32>){
    let (char_lists, int_list): (Vec<Vec<char>>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let chars: Vec<char> = iter.next().unwrap().chars().collect();
            let int_value: i32 = iter.next().unwrap().parse().unwrap();
            (chars, int_value)
        })
        .unzip();

    (char_lists, int_list)
}
