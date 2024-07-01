pub mod day12 {
    
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        // Cache the results of the find_arrangements function.
        // This slows down part 1, but makes part 2 run many times faster.
        let mut result_cache: HashMap<(Vec<char>, Vec<usize>), u64> = HashMap::new();
        
        // Resursively find all the ways to fit some number of groups into the given conditions
        fn find_arrangements(conditions: Vec<char>, groups: Vec<usize>, cache: &mut HashMap<(Vec<char>, Vec<usize>), u64>) -> u64 {
            
            // Base cases
            if groups.len() == 0 {
                if conditions.contains(&'#') {
                    return 0;
                }
                return 1;
            }
            if conditions.len() < groups[0] {
                return 0;
            }
            
            // Return early if the result is already in the cache
            let cache_key = (conditions.clone(), groups.clone());
            if cache.contains_key(&cache_key) {
                return *cache.get(&cache_key).unwrap();
            }
            
            let mut output = 0;
            
            for i in 0..=(conditions.len() - groups[0]) {
                // Check if the group can fit here
                let mut valid_position = true;
                'check_validity: for j in i..(i+groups[0]) {
                    if conditions[j] != '#' && conditions[j] != '?' {
                        valid_position = false;
                        break 'check_validity;
                    }
                }
                if conditions.len() > i+groups[0] && conditions[i+groups[0]] == '#' {
                    valid_position = false;
                }
                if i > 0 && conditions[i-1] == '#' {
                    break;
                }
                
                // Recursively call this function on slices of the two inputs
                if valid_position {
                    let cond_slice = Vec::from(&conditions[ (conditions.len()).min(i+groups[0]+1)..]);
                    let group_slice = Vec::from(&groups[1..]);
                    output += find_arrangements(cond_slice, group_slice, cache);
                }
            }
            
            cache.insert(cache_key, output);
            
            return output;
        }
        
        let mut arrangements = 0;
        
        for line in &lines {
            //println!("{}", line);
            
            let mut split = line.split(' ');
            let conditions = split.next().unwrap().chars().collect::<Vec<_>>();
            let groups = split.next().unwrap().split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            
            arrangements += find_arrangements(conditions, groups, &mut result_cache);
            
        }
        
        println!("Part 1: {}", arrangements);
        
        
        // ==== Part 2 ==== //
        
        let mut arrangements2 = 0;
        
        for line in &lines {
            // Reset the result cache
            result_cache = HashMap::new();
            
            //println!("{}", line);
            let mut split = line.split(' ');
            
            //let conditions = split.next().unwrap().chars().collect::<Vec<_>>();
            let conditions_str = split.next().unwrap();
            let conditions_5 = format!("{conditions_str}?{conditions_str}?{conditions_str}?{conditions_str}?{conditions_str}").split_inclusive('.').filter(|s| s != &".").map(|s| s.chars()).flatten().collect::<Vec<_>>();
            //let conditions_5: Vec<char> = format!("{conditions_str}?{conditions_str}?{conditions_str}?{conditions_str}?{conditions_str}").chars().collect::<Vec<_>>();
            
            let groups = split.next().unwrap().split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let groups_5: Vec<usize> = [&groups[..], &groups[..], &groups[..], &groups[..], &groups[..]].concat();
            
            let result_5 = find_arrangements(conditions_5, groups_5, &mut result_cache);
            
            //println!("Result: {}", result_5);
            
            arrangements2 += result_5;
            
        }
        
        println!("Part 2: {}", arrangements2);
        
    }
    
}
