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

#[derive(Debug, Clone)] 
struct Item {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn main() {
    let input = include_str!("in.txt");
    let (process_list, item_list) = parsing(input);

    // part 1
    let start: String = String::from("in");
    let mut accepted_items : Vec<Item> = Vec::new();

    for i in 0..item_list.len() {
        if visit(start.clone(), &item_list[i], &item_list, &process_list) == 1 {
            accepted_items.push(item_list[i].clone());
        }
    }

    // result part1
    let mut part1 = 0;
    for item in accepted_items {
        part1 += item.x + item.m + item.a + item.s;
    }
    println!("part1 {:?}", part1);
}
    
fn visit(name: String, item: &Item, item_list: &Vec<Item>, process_list: &Vec<Process>) -> i32 {
    let mut process_found_flag = false;

    for process in process_list{
        if process.name == name {
            process_found_flag = true;

            let mut subprocess_found = 0;
            for subprocess in &process.sub_processes {
                subprocess_found = process_subprocess(subprocess, &item, item_list, process_list);
                if subprocess_found != 0 {
                    break;
                }
            }

            if subprocess_found == 0 {
                if process.false_location == "A"{
                    return 1;
                } else if process.false_location == "R" {
                    return 2;
                } else {
                return visit(process.false_location.clone(), &item, item_list, process_list);
                }
            } else {
                return subprocess_found;
            }
        }
    }
    if process_found_flag == false {
        panic!("new process not found")
    }
    return 0; //wrong
}

fn process_subprocess(subprocess: &Subprocess, item: &Item, item_list: &Vec<Item>, process_list: &Vec<Process>) -> i32 {
    
    let x = item.x;
    let m = item.m;
    let a = item.a;
    let s = item.s;
    
    
    match subprocess.category {
        'x' => {
            match subprocess.symbol {
                '<' => {
                    if x < subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
                    }
                }
                '>' => {
                    if x > subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
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
                    if m < subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
                    }
                }
                '>' => {
                    if m > subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
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
                    if a < subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
                    }
                }
                '>' => {
                    if a > subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
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
                    if s < subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
                    }
                }
                '>' => {
                    if s > subprocess.value {
                        return return_what(subprocess, item, item_list, process_list);
                    } else {
                        return 0;
                    }
                }
                _ => {
                    panic!("Unexpected symbol: {}", subprocess.symbol);
                }
            }
        },
        _ => panic!("weird category")
    }
}
fn return_what(subprocess: &Subprocess, item: &Item, item_list: &Vec<Item>, process_list: &Vec<Process>) -> i32 {
    if subprocess.true_location == "A" {
        return 1;
    } else if subprocess.true_location == "R" {
        return 2;
    } else {
        return visit(subprocess.true_location.clone(), item, item_list, process_list);
    }
}
fn parsing(input: &str) -> (Vec<Process>, Vec<Item>) {
    let mut item_list: Vec<Item> = Vec::new();
    let mut process_list: Vec<Process> = Vec::new();

    let blocks: Vec<&str> = input.trim().split("\r\n\r\n").collect();

    // processes
    for line in blocks[0].lines() {
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

    // items
    for line in blocks[1].lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let x: i32 = parts[0][3..].parse().unwrap();
        let m: i32 = parts[1][2..].parse().unwrap();
        let a: i32 = parts[2][2..].parse().unwrap();
        let s: i32 = parts[3][2..parts[3].len()-1].parse().unwrap();

        let item = Item { x, m, a, s };
        item_list.push(item);
    }

    (process_list, item_list)
}