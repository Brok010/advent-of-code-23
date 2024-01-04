use num_integer::lcm;

#[derive(Debug, Clone)]
struct Module<'a> {
    prefix: char,
    name: &'a str,
    destinations: Vec<&'a str>,
}
#[derive(Debug, Clone, PartialEq)]
struct FlipFlop<'a> { //%
    name: &'a str,
    state: bool,
}
#[derive(Debug, Clone, PartialEq)]
struct Conjunction<'a> { //&
    name: &'a str,
    stack: Vec<(&'a str, i32)>,
}

fn main() {
    let input = include_str!("in.txt");
    let modules = parse_input(input);
    let (mut flipflops, mut conjunctions) = get_stacks_states(&modules);


    // part 1
    //inicialization
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    
    let cycle_count_simulation = 1000;
    let mut cycle_count = 1; // not incrementing on the inicialization
    let mut count_differences = vec![(low_pulses, high_pulses)];
    let original_state = (flipflops.clone(), conjunctions.clone());

    bfs(&modules, &mut flipflops, &mut conjunctions, &mut low_pulses, &mut high_pulses);
    let (low_pulses_dif, high_pulses_dif) = calculate_pulse_differences(low_pulses, high_pulses,
        count_differences.last().unwrap().0,
        count_differences.last().unwrap().1);
    count_differences.push((low_pulses_dif, high_pulses_dif));

    // cycle simulation
    while (!are_vectors_equal(&flipflops, &original_state.0) || !are_vectors_equal(&conjunctions, &original_state.1)) && cycle_count < cycle_count_simulation {
        cycle_count += 1;
        bfs(&modules, &mut flipflops, &mut conjunctions, &mut low_pulses, &mut high_pulses);
        let (low_pulses_dif, high_pulses_dif) = calculate_pulse_differences(low_pulses, high_pulses,
            count_differences.last().unwrap().0,
            count_differences.last().unwrap().1);
        count_differences.push((low_pulses_dif, high_pulses_dif));
    }

    // result calc
    let offset = cycle_count_simulation % cycle_count;
    let last_division_result = cycle_count_simulation / cycle_count;
    let partial_low = last_division_result * low_pulses;
    let partial_high = last_division_result * high_pulses;
    let partial_offset = count_differences[offset as usize].0 + count_differences[offset as usize].1;
    println!("part1: {}", partial_high * partial_low + partial_offset);

    // part 2 - its wrong but rx is the pulse im looking for and ql is the conjunction leading to it
    // reset
    flipflops = original_state.0.clone();
    conjunctions = original_state.1.clone();
    low_pulses = 0;
    high_pulses = 0;
    cycle_count = 1;
    let mut memory = [0; 4];

    loop {
        bfs(&modules, &mut flipflops, &mut conjunctions, &mut low_pulses, &mut high_pulses);
        cycle_count += 1;
        

        for con in &conjunctions { // this never changes? - prob inside the cycle
            if con.name == "ql" {
                for i in 0..con.stack.len() {
                    if con.stack[i].1 != 0 && memory[i] == 0 {
                        memory[i] = cycle_count;
                    }
                }
            }
        }
        if memory.iter().all(|&x| x != 0) {
            let lcm_result = memory.iter().fold(1, |acc, &x| lcm(acc, x));
            // Print the result
            println!("LCM of memory: {}", lcm_result);
            break;
        }
    }
    
}


fn calculate_pulse_differences(current_low_pulses: i32, current_high_pulses: i32, last_low_pulses: i32, last_high_pulses: i32) -> (i32, i32) {
    (current_low_pulses - last_low_pulses, current_high_pulses - last_high_pulses)
}

fn are_vectors_equal<T: PartialEq>(vec1: &[T], vec2: &[T]) -> bool {
    vec1.iter().zip(vec2.iter()).all(|(a, b)| a == b) && vec1.len() == vec2.len()
}

fn bfs(modules: &Vec<Module>, flipflops: &mut Vec<FlipFlop>, conjunctions: &mut Vec<Conjunction>, low_pulses: &mut i32, high_pulses: &mut i32) {
   
    let mut new_destinations: Vec<(&str, i32, &str)> = Vec::new();
    new_destinations.push(("broadcaster", 0, "")); //dest name, pulse going into the destination, last destination
            
    while !new_destinations.is_empty() {
        let (current_name, current_pulse, current_last_destination) = new_destinations.remove(0);

        if current_pulse == 0 {
            *low_pulses += 1;
        } else {
            *high_pulses += 1;
        }

        for module in modules {
            if current_name == module.name {
                let new_pulse = process_module(&module, current_pulse, flipflops, conjunctions, current_last_destination);

                if new_pulse != 2 {
                    for destination in &module.destinations {
                        new_destinations.push((destination, new_pulse, current_name));
                    }
                    break;
                }
            } 
        }
    }
}

fn process_module(module: &Module, pulse: i32, flipflops: &mut Vec<FlipFlop>, conjunctions: &mut Vec<Conjunction>, last_destination: &str) -> i32 {
    
    // in low pulse - flipflop switches states from on or off, if on - lowpulse, if off - highpulse
    //              - conjunction - remembers each last input for each of their connections - on pulse updates its stack and
    //                              if all in stack are high sends low pulse, if some are low sends highpulse
    // on high pulse - flipflop ignores - end of line
    
    let mut new_pulse = 2;
    match module.prefix {
        ' ' => {
            new_pulse = pulse;
        }
        '&' => {
            // update stack
            for i in 0..conjunctions.len() {
                if conjunctions[i].name == module.name {
                    for (stack_name, stack_state) in conjunctions[i].stack.iter_mut() {
                        if *stack_name == last_destination {
                            *stack_state = pulse;
                            break;
                        }
                    }
                    new_pulse = if is_all_stack_high(conjunctions[i].stack.clone()) { 0 } else { 1 };
                }
            }
        }
        '%' => {
            if pulse == 0 {
                for i in 0..flipflops.len() {

                    if flipflops[i].name == module.name {
                        if flipflops[i].state == false { //off
                            flipflops[i].state = true;
                            new_pulse = 1;
                            break;

                        } else { // if flipflop state is true
                            flipflops[i].state = false;
                            new_pulse = 0;
                            break;
                        }
                    }
                }
            }
        }
        _ => panic!("weird prefix")
    }
    new_pulse
}

fn is_all_stack_high(stack: Vec<(&str, i32)>) -> bool {
    for (_, state) in stack {
        if state != 1 {
            return false;
        }
    }
    true
}

fn get_stacks_states<'a>(modules: &'a Vec<Module<'a>>) -> (Vec<FlipFlop<'a>>, Vec<Conjunction<'a>>) {
    let mut flip_flops: Vec<FlipFlop> = Vec::new();
    let mut conjunctions: Vec<Conjunction> = Vec::new();

    // Part 1: Set the states and stacks
    for module in modules {
        if module.prefix == '%' {
            // Make a new flip-flop instance and set the name to the module.name and the state to false (off)
            let flip_flop = FlipFlop {
                name: module.name,
                state: false,
            };
            flip_flops.push(flip_flop);

        } else if module.prefix == '&' {
            // Go through all modules again and see if any of those module.destinations end in this module.name
            // If so, put the name of that module into the stack of this module
            let mut stack: Vec<(&str, i32)> = Vec::new();
            for other_module in modules {
                if other_module.destinations.contains(&module.name) {
                    stack.push((other_module.name, 0));
                }
            }
            let conjunction = Conjunction {
                name: module.name,
                stack,
            };
            conjunctions.push(conjunction);
        }
    }

    (flip_flops, conjunctions)
}

fn parse_input(input: &str) -> Vec<Module> {
    let mut modules = Vec::new();

    for line in input.lines() {

        let mut words = line.split(" ");
        let mut name = words.next().unwrap();
        let mut prefix: char = ' ';
        if name == "broadcaster" {
            prefix = ' ';
        } else {
            prefix = name.chars().next().unwrap();
            name = &name[1..];
        }
        let destinations: Vec<&str> = words
            .skip(1)
            .flat_map(|s| s.split(','))
            .filter(|s| !s.is_empty())
            .collect();
    
        let module = Module {
            prefix,
            name,
            destinations,
        };

        modules.push(module);
    }

    modules
}