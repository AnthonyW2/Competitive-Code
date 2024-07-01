pub mod day19 {
    
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        // Parse input
        let mut workflows_raw = Vec::new();
        let mut collector = Vec::new();
        for line in &lines {
            if line.len() > 0 {
                collector.push(line);
            } else if workflows_raw.len() == 0 {
                workflows_raw = collector.iter().map(|s| {
                    let mut split1 = s[..s.len()-1].split("{");
                    return (split1.next().unwrap(), split1.next().unwrap().split(",").collect::<Vec<_>>());
                }).collect::<Vec<_>>();
                collector = Vec::new();
            }
        }
        
        let parts = collector.iter().map(|s| {
            let mut part = HashMap::new();
            for attribute in s[1..s.len()-1].split(",") {
                part.insert(attribute.chars().next().unwrap(), attribute[2..].parse::<u64>().unwrap());
            }
            return part;
        }).collect::<Vec<_>>();
        let mut workflow_map = HashMap::new();
        for workflow in workflows_raw.clone() {
            workflow_map.insert(workflow.0, workflow.1);
        }
        
        // 'A' if the part is accepted; 'R' if rejected
        fn process_part(part: &HashMap<char, u64>, workflows: &HashMap<&str, Vec<&str>>, workflow: &str) -> char {
            
            let process_rule_outcome = |outcome| -> char {
                match outcome {
                    "A" => 'A',
                    "R" => 'R',
                    _ => process_part(part, workflows, outcome)
                }
            };
            
            for rule in &workflows[workflow] {
                
                let rule_parts = rule.split(":").collect::<Vec<_>>();
                if rule_parts.len() == 1 {
                    return process_rule_outcome(rule_parts[0]);
                } else {
                    let attribute_val = part[&rule_parts[0].chars().next().unwrap()];
                    let threshold_val = rule_parts[0][2..].parse::<u64>().unwrap();
                    if rule_parts[0].contains('>') {
                        if attribute_val > threshold_val {
                            return process_rule_outcome(rule_parts[1]);
                        }
                    } else {
                        if attribute_val < threshold_val {
                            return process_rule_outcome(rule_parts[1]);
                        }
                    }
                }
                
            }
            
            unreachable!();
        }
        
        let mut result1 = 0;
        
        for part in &parts {
            if process_part(part, &workflow_map, "in") == 'A' {
                result1 += part[&'x'];
                result1 += part[&'m'];
                result1 += part[&'a'];
                result1 += part[&'s'];
            }
        }
        
        println!("Part 1: {}", result1);
        
        
        // ==== Part 2 ==== //
        
        // Return a list of all combinations of ranges of numbers after running through the workflows (recursively)
        fn find_valid_number_combinations(numbers: HashMap<char, (u64, u64)>, workflows: &HashMap<&str, Vec<&str>>, workflow: &str) -> Vec<HashMap<char, (u64, u64)>> {
            let mut numbers = numbers.to_owned();
            
            let mut combinations: Vec<HashMap<char, (u64, u64)>> = Vec::new();
            
            let process_rule_outcome = |new_ranges, outcome| -> Vec<HashMap<char, (u64, u64)>> {
                match outcome {
                    "A" => vec![new_ranges],
                    "R" => vec![],
                    _ => find_valid_number_combinations(new_ranges, workflows, outcome)
                }
            };
            
            for rule in &workflows[workflow] {
                
                let rule_parts = rule.split(":").collect::<Vec<_>>();
                if rule_parts.len() == 1 {
                    // All ranges go to different workflow
                    combinations.extend(process_rule_outcome(numbers.clone(), rule_parts[0]));
                    
                } else {
                    let key = &rule_parts[0].chars().next().unwrap();
                    let attribute_range = numbers[key];
                    let threshold_val = rule_parts[0][2..].parse::<u64>().unwrap();
                    
                    if rule_parts[0].contains('>') {
                        if attribute_range.0 > threshold_val {
                            // All ranges go to different workflow
                            combinations.extend(process_rule_outcome(numbers.clone(), rule_parts[1]));
                            break;
                        } else if attribute_range.1 <= threshold_val {
                            // All ranges stay
                        } else {
                            // Split range
                            // The upper part of the range goes to the next workflow
                            let mut new_ranges = numbers.clone();
                            new_ranges.get_mut(key).unwrap().0 = threshold_val+1;
                            combinations.extend(process_rule_outcome(new_ranges, rule_parts[1]));
                            // The lower part of the range stays
                            numbers.get_mut(key).unwrap().1 = threshold_val;
                        }
                    } else {
                        if attribute_range.1 < threshold_val {
                            // All ranges go to different workflow
                            combinations.extend(process_rule_outcome(numbers.clone(), rule_parts[1]));
                            break;
                        } else if attribute_range.0 >= threshold_val {
                            // All ranges stay
                        } else {
                            // Split range
                            // The lower part of the range goes to the next workflow
                            let mut new_ranges = numbers.clone();
                            new_ranges.get_mut(key).unwrap().1 = threshold_val-1;
                            combinations.extend(process_rule_outcome(new_ranges, rule_parts[1]));
                            // The upper part of the range stays
                            numbers.get_mut(key).unwrap().0 = threshold_val;
                        }
                    }
                }
            }
            return combinations;
        }
        
        let starting_ranges = HashMap::from([
            ('x', (1,4000)),
            ('m', (1,4000)),
            ('a', (1,4000)),
            ('s', (1,4000)),
        ]);
        let valid_combinations = find_valid_number_combinations(starting_ranges, &workflow_map, "in");
        
        let mut result2 = 0;
        
        for ranges in &valid_combinations {
            result2 += (ranges[&'x'].1-ranges[&'x'].0+1) * (ranges[&'m'].1-ranges[&'m'].0+1) * (ranges[&'a'].1-ranges[&'a'].0+1) * (ranges[&'s'].1-ranges[&'s'].0+1);
            
        }
        
        println!("Part 2: {}", result2);
        
    }
    
}
