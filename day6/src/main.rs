fn main() {
    let input = include_str!("in.txt");
    let (time, distance) = parsing(input);

    println!("Time: {:?}", time);
    println!("Distance: {:?}", distance);

    let mut multiply_beaten: i128 = 0;

    for (i, milisec) in time.iter().enumerate() {
        let mut speed_mm = 0;
        let mut time_left = milisec.clone();
        let mut beat_counter = 0;

        while speed_mm <= *milisec {
            let race_distance = speed_mm * time_left;
            // println!("{}", race_distance);
            
            if race_distance > distance[i]{
                beat_counter += 1;
            }

            time_left -= 1;
            speed_mm += 1;
        }   
        println!("beaten: {}", beat_counter);
        if multiply_beaten == 0 {
            multiply_beaten += beat_counter;
        } else {
            multiply_beaten *= beat_counter;
        }
    }
    println!("multiplied beaten {}", multiply_beaten)
}

fn parsing(input: &str) -> (Vec<i128>, Vec<i128>) {
    let mut time_list: Vec<Vec<i128>> = Vec::new();
    let mut distance_list: Vec<Vec<i128>> = Vec::new();

    for line in input.lines() {
        // Split the line by whitespace
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Skip empty lines
        if parts.is_empty() {
            continue;
        }

        // Determine if it's a time or distance line
        match parts[0] {
            "Time:" => {
                // Parse the remaining elements as integers
                let values: Vec<i128> = parts[1..].iter().map(|s| s.parse().unwrap()).collect();
                time_list.push(values);
            }
            "Distance:" => {
                // Parse the remaining elements as integers
                let values: Vec<i128> = parts[1..].iter().map(|s| s.parse().unwrap()).collect();
                distance_list.push(values);
            }
            _ => {
                // Handle unexpected lines if necessary
            }
        }
    }

    (time_list[0].clone(), distance_list[0].clone())
}