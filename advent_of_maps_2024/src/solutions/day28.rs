pub mod day28 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Maximise cost to mow lawn, given rules
        
        // Store the cost of each space on the lawn
        let lawn = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        // Store the ranges between which only one space can be eaten
        let mut ranges = Vec::new();
        //let mut reversed_ranges = Vec::new();
        
        // Identify all spaces that are not within a limiting range
        let mut not_within_range = vec![true; lawn.len()];
        
        for (_li, line) in (&lines).iter().enumerate().skip(2) {
            let arr = line.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
            
            ranges.push(((arr[0]-1) as usize, (arr[1]-1) as usize));
            //reversed_ranges.push(((arr[1]-1) as usize, (arr[0]-1) as usize));
            
            for i in (arr[0]-1)..=(arr[1]-1) {
                not_within_range[i as usize] = false;
            }
            
        }
        
        ranges.sort_unstable();
        //reversed_ranges.sort_unstable();
        
        //println!("{:?}", lawn);
        //println!("{:?}", ranges);
        //println!("{:?}", not_within_range);
        
        
        
        let mut cache: Vec<Vec<Option<u64>>> = vec![vec![None; lawn.len()+1]; ranges.len()];
        
        /// Find the maximum cost of only spaces within ranges
        fn find_max_cost(lawn: &Vec<u64>, ranges: &Vec<(usize, usize)>, range: usize, min_idx: usize, cache: &mut Vec<Vec<Option<u64>>>) -> u64 {
            
            //println!("Range: {}", range);
            
            if cache[range][min_idx].is_some() {
                return cache[range][min_idx].unwrap();
            }
            
            println!("Range {} called", range);
            
            let mut best_sum = 0;
            
            let mut best_index = 0;
            
            // Check what happens if nothing in this range is eaten
            if range+1 < ranges.len() {
                best_sum = find_max_cost(lawn, ranges, range+1, min_idx, cache);
                best_index = -1;
            }
            
            // Check what happens if each of the positions is eaten
            for i in ranges[range].0.max(min_idx)..=ranges[range].1 {
                
                let mut new_sum = lawn[i];
                let mut next_range = range+1;
                
                let mut new_min_idx = 0;
                for r in 0..ranges.len() {
                    if i >= ranges[r].0 && i <= ranges[r].1 && ranges[r].1 > new_min_idx {
                        new_min_idx = ranges[r].1;
                    }
                }
                
                while next_range < ranges.len() && (i >= ranges[next_range].0) {
                    next_range += 1;
                }
                
                //println!("Range {} | Adding index {} | Next range {:?}", range, i, next_range);
                
                if next_range < ranges.len() {
                    if i >= ranges[next_range].0 || min_idx >= ranges[next_range].0 || i < min_idx {
                        println!("WARNING");
                    }
                    new_sum += find_max_cost(lawn, ranges, next_range, new_min_idx+1, cache);
                }
                
                if new_sum > best_sum {
                    println!("Range {} | Added index {} | Added range {:?} @ {}", range, i, next_range, new_min_idx+1);
                    best_sum = new_sum;
                    best_index = i as i32;
                }
                
            }
            
            //println!("Best sum from range {}: {} at index {}, from min_idx {}", range, best_sum, best_index, min_idx);
            
            cache[range][min_idx] = Some(best_sum);
            
            return best_sum;
            
        }
        
        let mut max_cost = 0;
        for i in 0..lawn.len() {
            if not_within_range[i] {
                max_cost += lawn[i];
                //println!("Add by default: lawn[{}] = {}", i, lawn[i]);
            }
        }
        
        max_cost += find_max_cost(&lawn, &ranges, 0, 0, &mut cache);
        
        println!("Solution: {}", max_cost);
        
        
        
    }
    
}

