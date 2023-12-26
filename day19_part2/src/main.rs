
#[derive(Debug)]
struct Process {
    name: String,
    sub_processes: Vec<Subprocess>,
    false_location: String,
}

#[derive(Debug)]
struct Subprocess {
    category: char,
    symbol: char,
    value: i32,
    true_location: String,
}

fn main() {
    let input = include_str!("in.txt");
    let process_list = parsing(input);
    // println!("{:?}", process_list)

    for process in process_list {
        
    }
}

fn parsing(input: &str) -> Vec<Process> {
    let mut process_list: Vec<Process> = Vec::new();

    // processes
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let index = parts[0].find('{');
        let name = &parts[0][..index.unwrap()];
        let parts_len = parts.len() - 1; //index
        let last_part_len = parts[parts_len].len() - 1; //index
        let false_location: String = parts[parts_len][..last_part_len].parse().unwrap();
        let mut sub_processes = Vec::new();
        
        for i in 0..parts_len { //last part is not included
            let mut j = 0; //j is the start of the subprocess
            if i == 0 {
                j = index.unwrap() + 1;
            }
            let chared_part: Vec<char> = parts[i].chars().collect();
            let category = chared_part[j];
            let symbol = chared_part[j + 1];
            let double_dot = parts[i].find(':');
            let value: i32 = chared_part[j + 2..double_dot.unwrap()].iter().collect::<String>().parse().unwrap();
            let true_location: String = chared_part[double_dot.unwrap() + 1..].iter().collect();
            let sub_process = Subprocess {
                category,
                symbol,
                value,
                true_location,
            };
    
            sub_processes.push(sub_process);
        }
        
        let process = Process {
            name: name.to_string(),
            sub_processes,
            false_location,
        };
    
        process_list.push(process);
        
    }
    process_list
}