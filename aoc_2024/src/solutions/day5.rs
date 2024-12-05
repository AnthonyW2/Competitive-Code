pub mod day5 {
    
    pub fn solution(lines: Vec<String>) {
        
        let mut ordering_rules = Vec::new();
        let mut updates = Vec::new();
        
        let mut line_section = 0;
        
        for line in &lines {
            
            if line_section == 0 {
                
                if line == "" {
                    line_section += 1;
                } else {
                    let split = line.split('|').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
                    ordering_rules.push((split[0],split[1]));
                }
                
            } else {
                let split = line.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
                updates.push(split);
            }
            
        }
        
        ordering_rules.sort();
        
        // Part 1
        fn check_order (update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
            
            let mut ordered = true;
            
            'rules: for r in rules {
                
                let mut seen_first = false;
                let mut seen_second = false;
                
                'pages: for p in update {
                    
                    if *p == r.0 {
                        seen_first = true;
                        if seen_second {
                            ordered = false;
                            break 'rules;
                        }
                    } else if *p == r.1 {
                        seen_second = true;
                        if seen_first {
                            break 'pages;
                        }
                    }
                    
                }
                
            }
            
            return ordered;
            
        }
        
        // Part 2
        fn fix_ordering (update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
            
            let mut reordered: Vec<u32> = update.to_owned();
            
            // This can panic if the rules don't describe a total ordering
            reordered.sort_unstable_by(|a,b| compare_two(*a,*b, rules));
            
            return reordered;
            
        }
        
        fn compare_two (n1: u32, n2: u32, rules: &Vec<(u32, u32)>) -> std::cmp::Ordering {
            
            for r in rules {
                if r.0 == n1 && r.1 == n2 {
                    return std::cmp::Ordering::Less;
                } else if r.1 == n1 && r.0 == n2 {
                    return std::cmp::Ordering::Greater;
                }
            }
            
            return std::cmp::Ordering::Equal;
            
        }
        
        let mut ordered_middle_count = 0;
        let mut unordered_middle_count = 0;
        
        for u in &updates {
            
            let ordered = check_order(u, &ordering_rules);
            
            if ordered {
                ordered_middle_count += u[u.len() / 2];
            } else {
                let fixed = fix_ordering(u, &ordering_rules);
                unordered_middle_count += fixed[fixed.len() / 2];
            }
            
        }
        
        println!("Part 1: {}", ordered_middle_count);
        
        println!("Part 2: {}", unordered_middle_count);
        
    }
    
}
