

#[derive(Debug)]
struct Module<'a> {
    prefix: char,
    name: &'a str,
    destinations: Vec<&'a str>,
}
#[derive(Debug, Clone)]
struct FlipFlop<'a> { //%
    name: &'a str,
    state: bool,
}
#[derive(Debug, Clone)]
struct Conjunction<'a> { //&
    name: &'a str,
    stack: Vec<(&'a str, bool)>,
}

fn main() {
    let input = include_str!("in.txt");
    let modules = parse_input(input);
    let (mut flipflops, mut conjunctions) = get_stacks_states(&modules);
    // part 1
    let start = "broadcaster";
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    visit(start, false, &modules, &mut flipflops, &mut conjunctions, &mut low_pulses, &mut high_pulses, " ");
    println!("{},{}", low_pulses, high_pulses)

}

fn visit(name_tag: &str, pulse: bool, modules: &Vec<Module>, flipflops: &mut Vec<FlipFlop>, 
    conjunctions: &mut Vec<Conjunction>, low_pulses: &mut i32, high_pulses: &mut i32, last_destination: &str) {
    
    if pulse == false {
        *low_pulses += 1;
    } else {
        *high_pulses += 1;
    }

    let mut new_pulse = pulse;

    for module in modules {
        if module.name == name_tag {
            match module.prefix {
                ' ' => {
                    new_pulse = pulse;
                }
                '&' => {
                    // update stack
                    for i in 0..conjunctions.len() {
                        if conjunctions[i].name == module.name {
                            for stack in conjunctions[i].stack.iter_mut() {
                                if stack.0 == last_destination {
                                    stack.1 = pulse;
                                    break;
                                }
                            }
                            new_pulse = if is_all_stack_high(conjunctions[i].stack.clone()) { false } else { true };
                        }
                    }
                }
                '%' => {
                    if pulse == false {
                        for i in 0..flipflops.len() {

                            if flipflops[i].name == module.name {
                                if flipflops[i].state == false { //off
                                    flipflops[i].state = true;
                                    new_pulse = true;
                                    break;

                                } else { // if flipflop state is true
                                    flipflops[i].state = false;
                                    new_pulse = false;
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => panic!("weird prefix")
            }
            for i in 0..module.destinations.len() {
                let current_destination = module.destinations[i];
                visit(current_destination, new_pulse, modules, &mut flipflops.clone(), &mut conjunctions.clone(), low_pulses, high_pulses, &module.name)
            }
        }
    }
}

fn is_all_stack_high(stack: Vec<(&str, bool)>) -> bool {
    for each in stack {
        if each.1 != true {
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
            let mut stack: Vec<(&str, bool)> = Vec::new();
            for other_module in modules {
                if other_module.destinations.contains(&module.name) {
                    stack.push((other_module.name, false));
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