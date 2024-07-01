pub mod day28 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Maximise cost for goat to eat the lawn, given rules
        
        // Store the cost of each patch on the lawn
        let lawn = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        // Store the ranges between which only one patch can be eaten
        let mut ranges = Vec::new();
        
        // Identify all patchs that are not within a limiting range
        let mut not_within_range = vec![true; lawn.len()];
        
        // For each index in `lawn`, store the next larger index that is not within the same range(s)
        let mut next_valid_index_list = Vec::new();
        
        // For each index in `lawn`, store the next range that starts at a position greater than the next valid index at that position
        let mut next_valid_range_list = Vec::new();
        
        for (_li, line) in (&lines).iter().enumerate().skip(2) {
            let arr = line.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
            
            // Add the range to the list
            ranges.push(((arr[0]-1) as usize, (arr[1]-1) as usize));
            
            // Flag patchs that are within a range
            for i in (arr[0]-1)..=(arr[1]-1) {
                not_within_range[i as usize] = false;
            }
            
        }
        
        ranges.sort_unstable();
        
        for i in 0..lawn.len() {
            
            // Find next minimum index after i
            let mut min_idx = i;
            for r in 0..ranges.len() {
                if i >= ranges[r].0 && i <= ranges[r].1 && ranges[r].1 > min_idx {
                    min_idx = ranges[r].1;
                } else if i <= ranges[r].0 {
                    break;
                }
            }
            next_valid_index_list.push(min_idx+1);
            
            
            // Find next range after the minimum index
            let mut next_range = 0;
            while next_range < ranges.len() && ranges[next_range].0 <= min_idx+1 {
                next_range += 1;
            }
            next_valid_range_list.push(next_range);
            
        }
        
        
        
        // Indexing format: cache[range idx][abs idx - range lower]
        let mut cache: Vec<Vec<Option<u64>>> = Vec::new();
        for r in 0..ranges.len() {
            cache.push(vec![None; lawn.len() - ranges[r].0 + 1]);
        }
        
        /// Find the maximum cost of only patchs within ranges
        fn find_max_cost(lawn: &Vec<u64>, ranges: &Vec<(usize, usize)>, next_valid_index_list: &Vec<usize>, next_valid_range_list: &Vec<usize>, range: usize, min_idx: usize, cache: &mut Vec<Vec<Option<u64>>>) -> u64 {
            
            if min_idx >= lawn.len() {
                return 0;
            }
            
            // Check cache to see if the answer has already been calculated
            if cache[range][min_idx.max(ranges[range].0) - ranges[range].0].is_some() {
                return cache[range][min_idx.max(ranges[range].0) - ranges[range].0].unwrap();
            }
            
            let mut best_sum = 0;
            
            // Check what happens if nothing in this range is eaten
            if range+1 < ranges.len() {
                best_sum = find_max_cost(lawn, ranges, next_valid_index_list, next_valid_range_list, range+1, min_idx, cache);
            }
            
            // Check what happens if each of the positions is eaten
            for i in ranges[range].0.max(min_idx)..=ranges[range].1 {
                
                let mut new_sum = lawn[i];
                
                // Find the next minimum index
                let new_min_idx = next_valid_index_list[i];
                
                // Find the next valid range
                let next_range = next_valid_range_list[i];
                
                if next_range < ranges.len() {
                    // Recursively call on the next range
                    new_sum += find_max_cost(lawn, ranges, next_valid_index_list, next_valid_range_list, next_range, new_min_idx, cache);
                }
                
                if new_sum > best_sum {
                    best_sum = new_sum;
                }
                
            }
            
            cache[range][min_idx.max(ranges[range].0) - ranges[range].0] = Some(best_sum);
            
            return best_sum;
            
        }
        
        let mut max_cost = 0;
        
        // Add up the costs of the patchs where the rules don't apply
        for i in 0..lawn.len() {
            if not_within_range[i] {
                max_cost += lawn[i];
            }
        }
        
        // Find the maximum cost of patchs where the rules do apply
        max_cost += find_max_cost(&lawn, &ranges, &next_valid_index_list, &next_valid_range_list, 0, 0, &mut cache);
        
        println!("Solution: {}", max_cost);
        
    }
    
}

