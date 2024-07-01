pub mod day20 {
    
    use std::collections::VecDeque;
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        const LOW: bool = false;
        const HIGH: bool = true;
        
        const BROADCAST: u8 = 0;
        const FLIPFLOP: u8 = 1;
        const CONJUNCTION: u8 = 2;
        
        let mut modules: HashMap<&str, (u8, Vec<&str>, HashMap<&str, bool>)> = HashMap::new();
        
        let mut rx_present = false;
        
        for line in &lines {
            let mut split1 = line.split(" -> ");
            let mut mod_name = split1.next().unwrap();
            let mod_type;
            let targets = split1.next().unwrap().split(", ").collect::<Vec<_>>();
            let mut storage = HashMap::new();
            if mod_name.chars().next().unwrap() == '%' {
                // Flip-flop
                mod_type = FLIPFLOP;
                mod_name = &mod_name[1..];
                storage.insert("state", false);
            } else if mod_name.chars().next().unwrap() == '&' {
                // Conjunction
                mod_type = CONJUNCTION;
                mod_name = &mod_name[1..];
            } else {
                // Broadcaster
                mod_type = BROADCAST;
            }
            
            modules.insert(mod_name, (mod_type, targets, storage));
        }
        
        // Populate storage for conjunction modules
        for module in modules.clone().iter() {
            for target in &module.1.1 {
                if modules.contains_key(target) {
                    if modules[target].0 == 2 {
                        modules.get_mut(target).unwrap().2.insert(*module.0, LOW);
                    }
                } else {
                    if target == &"rx" {
                        rx_present = true;
                    }
                    //println!("Module \"{}\" does not exist", target);
                }
            }
        }
        
        fn simulate_button_press<'a>(modules: &mut HashMap<&str, (u8, Vec<&'a str>, HashMap<&'a str, bool>)>, inverter_list: &Vec<&str>) -> (u64, u64, &'a str) {
            // (source, dest, strength (false=low, true=high))
            let mut pulses: VecDeque<(&str, &str, bool)> = VecDeque::new();
            pulses.push_back(("button", "broadcaster", LOW));
            
            let mut low_count = 0;
            let mut high_count = 0;
            let mut seen_inverter = "";
            
            while !pulses.is_empty() {
                let pulse = pulses.pop_front().unwrap();
                
                if pulse.2 == HIGH {
                    high_count += 1;
                } else {
                    low_count += 1;
                }
                
                // Check if one of the flagged modules received a low pulse
                if !pulse.2 && inverter_list.contains(&pulse.1) {
                    seen_inverter = pulse.1;
                }
                
                if !modules.contains_key(pulse.1) {
                    //println!("Sent {} to \"{}\" which doesn't exist", pulse.2, pulse.1);
                    continue;
                }
                
                let module = modules.get_mut(pulse.1).unwrap();
                
                if module.0 == BROADCAST {
                    // Send the same pulse to all targets
                    for next_target in &module.1 {
                        pulses.push_back((pulse.1, next_target, pulse.2));
                    }
                } else if module.0 == FLIPFLOP {
                    if pulse.2 == LOW {
                        // Flip the state
                        module.2.insert("state", !module.2["state"]);
                        // Send the corresponding pulse to all targets
                        for next_target in &module.1 {
                            pulses.push_back((pulse.1, next_target, module.2["state"]));
                        }
                    }
                } else {
                    module.2.insert(pulse.0, pulse.2);
                    let pulse_type = module.2.values().any(|v| *v == LOW); // False if all high
                    for next_target in &module.1 {
                        pulses.push_back((pulse.1, next_target, pulse_type));
                    }
                }
                
            }
            
            return (low_count, high_count, seen_inverter);
            
        }
        
        // (low, high)
        let mut results = Vec::new();
        
        let mut modified_modules = modules.clone();
        for _ in 0..1000 {
            results.push(simulate_button_press(&mut modified_modules, &Vec::new()));
            
            if modified_modules == modules {
                break;
            }
        }
        
        let sum_low: u64 = results.iter().map(|t| t.0).sum::<u64>() * (1000/results.len() as u64);
        let sum_high: u64 = results.iter().map(|t| t.1).sum::<u64>() * (1000/results.len() as u64);
        let remainder_low: u64 = results[..(1000%results.len())].iter().map(|t| t.0).sum();
        let remainder_high: u64 = results[..(1000%results.len())].iter().map(|t| t.1).sum();
        
        println!("Part 1: {}", (sum_low+remainder_low) * (sum_high+remainder_high));
        
        
        // ==== Part 2 ==== //
        
        /*
            This solution leans heavily on the layout of the modules in my given input.
            It probably doesn't work globally, and it may fail for other AoC inputs.
            
            The broadcaster sends a low pulse to 4 flip-flops.
            Each flip-flop is connected to a structure which acts as a binary counter, outputting a low
            pulse after a large number of button presses.
            The output of each binary counter goes through an inverter (conjunction with just 1 input)
            which connects to a final collector conjunction which leads to the "rx" module.
            
            Therefore the answer is whenever all 4 inverters receive a low pulse (the lowest common multiple
            can be used to find the final answer after simulating a relatively low number of button presses).
        */
        
        if !rx_present {
            return;
        }
        
        let mut inverters = Vec::new();
        let mut inverter_iteration_counts = Vec::new();
        
        for module in modules.iter() {
            if module.1.0 == CONJUNCTION && module.1.2.len() == 1 {
                inverters.push(*module.0);
                inverter_iteration_counts.push(0);
            }
        }
        
        let mut modified_modules = modules.clone();
        
        // Find the iteration when each of the flagged modules receives a low pulse
        let mut iterations: u64 = 1;
        while inverter_iteration_counts.iter().any(|i| *i == 0) {
            let inverter = simulate_button_press(&mut modified_modules, &inverters).2;
            
            if inverter.len() > 0 {
                let matching_inverter = inverters.iter().position(|i| i == &inverter).unwrap();
                if inverter_iteration_counts[matching_inverter] == 0 {
                    inverter_iteration_counts[matching_inverter] = iterations;
                }
            }
            
            iterations += 1;
        }
        
        // Find the lowest common multiple of the iteration counts
        let mut lcm = inverter_iteration_counts[0];
        for n in inverter_iteration_counts {
            // Find the greatest common divisor
            let mut gcd = lcm;
            let mut b = n;
            while b != 0 {
                (gcd, b) = (b, gcd % b);
            }
            // Calculate the new LCM
            lcm = lcm*n / gcd;
        }
        
        println!("Part 2: {}", lcm);
        
    }
    
}
