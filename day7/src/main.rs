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

    divede_list(1, &hands, &values);
    divede_list(2, &hands, &values);
}

fn divede_list(part: i32, hands: &Vec<Vec<char>>, values: &Vec<i32>){
    //dividing
    
    let mut max_rank = values.len() as i32;
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
        if contains_x_same_signs(5, &hand, part) {
            combinations.five_a_kind.push((hand.clone(), values[i]));
        }

        // if 4 are the same
        else if contains_x_same_signs(4, &hand, part) {
            combinations.four_a_kind.push((hand.clone(), values[i]));
        }
        
        //if fullhouse
        else if contains_fullhouse(&hand, part) {
            combinations.fullhouse.push((hand.clone(), values[i]));
        }

        // if 3 are the same
        else if contains_x_same_signs(3,&hand, part) {
            combinations.three_a_kind.push((hand.clone(), values[i]));
        }
        
        // if 2 pairs
        else if contains_two_pairs(&hand) {
            combinations.two_pairs.push((hand.clone(), values[i]));
        }

        // if a pair
        else if contains_x_same_signs(2,&hand, part) {
            combinations.two_a_kind.push((hand.clone(), values[i]));
        }
        else {
            combinations.no_combination.push((hand.clone(), values[i]));
        }
    }

    process_lists(part, max_rank, &mut combinations);
}

fn process_lists(part: i32, mut max_rank: i32, combinations: &mut Combinations) {
        //order and evaluate each list
        let mut part_result = 0;
        let mut partial_result = 0;
        (max_rank, partial_result) = evaluate_list(&mut combinations.five_a_kind, max_rank, part);
        part_result += partial_result;
    
        (max_rank, partial_result) = evaluate_list(&mut combinations.four_a_kind, max_rank, part);
        part_result += partial_result;
    
        (max_rank, partial_result) = evaluate_list(&mut combinations.fullhouse, max_rank, part);
        part_result += partial_result;
        
        (max_rank, partial_result) = evaluate_list(&mut combinations.three_a_kind, max_rank, part);
        part_result += partial_result;
    
        (max_rank, partial_result) = evaluate_list(&mut combinations.two_pairs, max_rank, part);
        part_result += partial_result;
    
        (max_rank, partial_result) = evaluate_list(&mut combinations.two_a_kind, max_rank, part);
        part_result += partial_result;
    
        (max_rank, partial_result) = evaluate_list(&mut combinations.no_combination, max_rank, part);
        part_result += partial_result;
        
        println!("{}", part_result);
}


fn evaluate_list(list: &mut Vec<(Vec<char>, i32)>, max_rank: i32, part: i32) -> (i32, i128) {
    let mut value_list: Vec<(Vec<i32>, i32)> = Vec::new();
    
    for item in list.iter_mut(){
        // this gives me evaluated card with its bet
       value_list.push(((get_value(&item.0, part)), item.1.clone()));
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

fn get_value(item: &Vec<char>, part: i32) -> Vec<i32>{
    let cards_part1: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    let cards_part2: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    let mut values: Vec<i32> = vec![0; 5];
    let mut cards: Vec<char> = Vec::new();

    if part == 1 {
        cards = cards_part1;
    } else {
        cards = cards_part2;
    }

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
    let mut repetitions = vec![1; 5];
    for i in 0..hand.len(){
        for j in 0..hand.len(){
            if i == j {
                continue;
            }
            if hand[i] == hand[j]{
                repetitions[i] += 1;
            }
        }
    }

    let count_of_2_repetitions = repetitions.iter().filter(|&&x| x == 2).count();
    count_of_2_repetitions == 4
}

fn contains_fullhouse(hand: &Vec<char>, part: i32) -> bool{
    if part == 1 {
        if contains_x_same_signs(3, &hand, part) && contains_x_same_signs(2, &hand, part){
            return true
        }
        else {
            return false
        }
    }
    if part == 2 && contains_two_pairs(hand) && contains_Js(hand) == 1 {
        return true;
    }
    if part == 2 && contains_Js(hand) == 0 {
        if contains_x_same_signs(3, &hand, part) && contains_x_same_signs(2, &hand, part){
            return true
        }
        else {
            return false
        }
    }
    return  false;
}

fn contains_x_same_signs(num: i32, hand: &Vec<char>, part: i32) -> bool {

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

        if counter == num{
            return true;
        }
        if part == 2 {
            let js_count = contains_Js(&hand);
            // if the evaluated sign is not J and the card contains other Js we can promote it
            if counter + js_count == num  && hand[i] != 'J'{
                return true;
            }
        }
    }
    return false;
}

fn contains_Js(hand: &Vec<char>) -> i32 {
    let mut result = 0;
    for &each in hand{
        if each == 'J' {
            result += 1;
        }
    }
    result
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
