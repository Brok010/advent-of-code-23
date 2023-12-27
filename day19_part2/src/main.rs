// item values x m a s ranged from 1 to 4000

#[derive(Debug)]
struct Process {
    name: String,
    sub_processes: Vec<Subprocess>,
    false_location: String,
}

#[derive(Debug, Clone)]
struct Minmax {
    x: (i32, i32), //min max for each value
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32),
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

    let start: String = String::from("in");
    let minmax = Minmax {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    let results = visit(start, &process_list, minmax);
    
    
    let mut combinations: i128 = 0;
    for each in results{
        let mut partial_comb: i128 = 1;
        partial_comb *= (each.x.1 - each.x.0 + 1) as i128;
        partial_comb *= (each.m.1 - each.m.0 + 1) as i128;
        partial_comb *= (each.a.1 - each.a.0 + 1) as i128;
        partial_comb *= (each.s.1 - each.s.0 + 1) as i128;
        println!("{:?}", each);
        combinations += partial_comb;
    }
    println!("{}, {}", combinations, count_digits(combinations))
}

fn visit(name_tag: String, process_list: &Vec<Process>, minmax: Minmax) -> Vec<Minmax> {
    let mut ranges: Vec<Minmax> = Vec::new();

    for process in process_list {
        if process.name == name_tag {
            let new_ranges = process_process(process, &minmax, process_list);
            for each in new_ranges {
                ranges.push(each);
            }
        }
    }

    ranges
}

fn process_process(process: &Process, minmax: &Minmax, process_list: &Vec<Process>) -> Vec<Minmax> {
    let mut list_of_ranges: Vec<Minmax> = Vec::new();
    let mut current_minmax = minmax.clone();

    for subprocess in &process.sub_processes {
        let (true_minmax, false_minmax) = process_subprocess(&subprocess, &current_minmax);
        // update minmax with false minmax to pass into another subprocess or to false location
        current_minmax = compare_minmax(&current_minmax, &false_minmax);

        if subprocess.true_location == "A" {
            list_of_ranges.push(true_minmax);
        
        } else if subprocess.true_location != "R" {
            let new_minmax = visit(subprocess.true_location.clone(), process_list, true_minmax);
            for each in new_minmax {
                list_of_ranges.push(each);
            }
        }
    }
    if process.false_location == "A" {
        list_of_ranges.push(current_minmax.clone());

    } else if process.false_location != "R" {
        let false_minmax = visit(process.false_location.clone(), process_list, current_minmax);
        for each in false_minmax {
            list_of_ranges.push(each)
        }
    }
    
    list_of_ranges
}

fn compare_minmax(m1: &Minmax, m2: &Minmax) -> Minmax { //m1 - base
    let x = (
        m2.x.0.max(m1.x.0),
        m2.x.1.min(m1.x.1),
    );

    let m = (
        m2.m.0.max(m1.m.0),
        m2.m.1.min(m1.m.1),
    );

    let a = (
        m2.a.0.max(m1.a.0),
        m2.a.1.min(m1.a.1),
    );

    let s = (
        m2.s.0.max(m1.s.0),
        m2.s.1.min(m1.s.1),
    );

    Minmax { x, m, a, s }
}

// return the range that would return true
fn process_subprocess(subprocess: &Subprocess, minmax: &Minmax) -> (Minmax, Minmax) {
    let mut true_minmax = minmax.clone();
    let mut false_minmax = minmax.clone();
    match subprocess.category {
        'x' => {
            match subprocess.symbol {
                '<' => {
                    if true_minmax.x.1 > subprocess.value - 1 { // x < 1000
                        true_minmax.x.1 = subprocess.value - 1;
                    }
                    if false_minmax.x.0 < subprocess.value {
                        false_minmax.x.0 = subprocess.value;
                    }
                }
                '>' => {
                    if true_minmax.x.0 < subprocess.value + 1 { // x > 1000
                        true_minmax.x.0 = subprocess.value + 1;
                    }
                    if false_minmax.x.1 > subprocess.value {
                        false_minmax.x.1 = subprocess.value;
                    }
                }
                _ => {
                    panic!("Unexpected symbol: {}", subprocess.symbol);
                }
            }
        },

        'm' => {
            match subprocess.symbol {
                '<' => {
                    if true_minmax.m.1 > subprocess.value - 1 { 
                        true_minmax.m.1 = subprocess.value - 1;
                    }
                    if false_minmax.m.0 < subprocess.value {
                        false_minmax.m.0 = subprocess.value;
                    }
                }
                '>' => {
                    if true_minmax.m.0 < subprocess.value + 1 { 
                        true_minmax.m.0 = subprocess.value + 1;
                    }
                    if false_minmax.m.1 > subprocess.value {
                        false_minmax.m.1 = subprocess.value;
                    }
                }
                _ => {
                    panic!("Unexpected symbol: {}", subprocess.symbol);
                }
            }
        },
        'a' => {
            match subprocess.symbol {
                '<' => {
                    if true_minmax.a.1 > subprocess.value - 1 { 
                        true_minmax.a.1 = subprocess.value - 1;
                    }
                    if false_minmax.a.0 < subprocess.value {
                        false_minmax.a.0 = subprocess.value;
                    }
                }
                '>' => {
                    if true_minmax.a.0 < subprocess.value + 1 { 
                        true_minmax.a.0 = subprocess.value + 1;
                    }
                    if false_minmax.a.1 > subprocess.value {
                        false_minmax.a.1 = subprocess.value;
                    }
                }
                _ => {
                    panic!("Unexpected symbol: {}", subprocess.symbol);
                }
            }
        },
        's' => {
            match subprocess.symbol {
                '<' => {
                    if true_minmax.s.1 > subprocess.value - 1 { 
                        true_minmax.s.1 = subprocess.value - 1;
                    }
                    if false_minmax.s.0 < subprocess.value {
                        false_minmax.s.0 = subprocess.value;
                    }
                }
                '>' => {
                    if true_minmax.s.0 < subprocess.value + 1 { 
                        true_minmax.s.0 = subprocess.value + 1;
                    }
                    if false_minmax.s.1 > subprocess.value {
                        false_minmax.s.1 = subprocess.value;
                    }
                }
                _ => {
                    panic!("Unexpected symbol: {}", subprocess.symbol);
                }
            }
        },
        _ => panic!("weird category")
    }
    (true_minmax, false_minmax)
}

fn count_digits(number: i128) -> usize {
    let number_str = number.abs().to_string();
    number_str.len()
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