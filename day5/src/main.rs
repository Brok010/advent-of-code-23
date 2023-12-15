fn main() {
    // Your input data
    let input = include_str!("in.txt");
    let list = parsing(input).unwrap();

    let mut seeds = list[0][1..].to_vec();
    let mut seeds_part2: Vec<i128> = Vec::new();

    let mut c = 1;
    let mut seed: i128 = 0;

    for num in seeds.clone(){
        if c % 2 == 1{
            // seed
            seed = num.clone();
            seeds_part2.push(seed);
            c += 1;

        } else {
            //range
            for i in 1..num{
                seeds_part2.push(seed + i);
            }
            c += 1;
        }
    }  
    println!("seeds done");

    let maps = list[3..].to_vec();
    logic(&mut seeds, &maps);  
    println!("part1 done");
    logic(&mut seeds_part2, &maps);
    println!("part2 done")


}

fn logic(seeds: &mut Vec<i128>, maps: &Vec<Vec<i128>>) {
    let mut lock = vec![0; seeds.len()];
    let mut counter = 0;
    for each in maps{
        
        //skip empty lists - new lines
        if each.is_empty() || each == &[0, 0] {
            for value in &mut lock {
                *value = 0;
            }
            counter += 1;
            if counter % 2 == 0 {
                // println!("{:?}", &seeds);
            }
            continue;
        }

        let push = each[0] - each[1];

        for i in 0..seeds.len() {
            let num = seeds[i];
            if num >= each[1] && num < each[1] + each[2] && lock[i] == 0 {
                lock[i] = 1;
                seeds[i] += push;
            }
        }
    }
    // println!("{:?}", &seeds);

    if let Some(min_value) = seeds.iter().cloned().min() {
        println!("{}", min_value);
    }
}


fn parsing(input: &str) -> Option<Vec<Vec<i128>>> {
    let maps: Vec<Vec<Vec<i128>>> = input
        .split("\n\n")
        .filter_map(|s| {
            let lines: Vec<Vec<i128>> = s
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse().unwrap_or_default())
                        .collect()
                })
                .collect();

            if lines.is_empty() {
                None
            } else {
                Some(lines)
            }
        })
        .collect();

    // Access the first element of the outer vector
    maps.get(0).cloned()
}
