pub mod day5 {
    
    pub fn solution(lines: Vec<String>) {
        
        let seeds = lines[0].split(": ").last().unwrap().split(' ').map(|c| c.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        let mut maps = Vec::new();
        let mut parsed_maps = Vec::new(); // part 2
        let mut current_block = Vec::new();
        for line in &lines[2..] {
            let str = line;
            if str.is_empty() {
                maps.push(current_block);
                current_block = Vec::new();
            } else {
                current_block.push(str);
            }
        }
        if current_block.len() > 0 {
            maps.push(current_block);
        }
        
        
        // ==== Part 1 ==== //
        
        let mut mapped_vals = seeds.clone();
        
        for map in &maps {
            
            let mut new_vals = vec![None; mapped_vals.len()];
            
            let mut new_parsed_maps = Vec::new(); // part 2
            
            for range in &map[1..] {
                let mut split = range.split(' ');
                let dest_start = split.next().unwrap().parse::<u64>().unwrap();
                let source_start = split.next().unwrap().parse::<u64>().unwrap();
                let range_len = split.next().unwrap().parse::<u64>().unwrap();
                
                new_parsed_maps.push((dest_start, source_start, range_len)); // part 2
                
                for (i, val) in (&mapped_vals).iter().enumerate() {
                    
                    if *val >= source_start && *val < source_start+range_len {
                        new_vals[i] = Some(dest_start + (val-source_start));
                    }
                    
                }
                
            }
            
            new_parsed_maps.sort_by_key(|k| k.1);
            parsed_maps.push(new_parsed_maps); // part 2
            
            for (i, val) in (&new_vals).iter().enumerate() {
                if val.is_some() {
                    mapped_vals[i] = val.unwrap();
                }
            }
            
        }
        
        println!("Part 1: {}", mapped_vals.iter().min().unwrap());
        
        
        // ==== Part 2 ==== //
        
        let mut mapped_ranges = Vec::new();
        for i in 0..(seeds.len()/2) {
            mapped_ranges.push((seeds[2*i], seeds[2*i+1]));
        }
        
        for map in &parsed_maps {
            
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();
            
            for (val_ran_start, val_ran_len) in mapped_ranges {
                
                let mut previous_source_end = val_ran_start;
                
                for r in 0..map.len() {
                    let dest_start = map[r].0;
                    let source_start = map[r].1;
                    let range_len = map[r].2;
                    
                    // Gap between ranges
                    if source_start > previous_source_end {
                        new_ranges.push((previous_source_end, source_start-previous_source_end));
                    }
                    previous_source_end = source_start+range_len;
                    
                    // Check if map range overlaps current value range
                    if source_start < val_ran_start+val_ran_len && source_start+range_len > val_ran_start {
                        
                        let new_start;
                        let new_end;
                        
                        if source_start+range_len <= val_ran_start+val_ran_len {
                            new_end = dest_start+range_len;
                        } else {
                            new_end = dest_start + (val_ran_start+val_ran_len) - source_start;
                        }
                        
                        if source_start >= val_ran_start {
                            new_start = dest_start;
                        } else {
                            new_start = dest_start + val_ran_start - source_start;
                        }
                        
                        new_ranges.push((new_start, new_end-new_start));
                        
                    }
                    
                }
                
                if previous_source_end < val_ran_start+val_ran_len {
                    let new_start = std::cmp::max(previous_source_end, val_ran_start);
                    new_ranges.push((new_start, val_ran_start+val_ran_len-new_start));
                }
                
            }
            
            mapped_ranges = new_ranges;
            
        }
        
        println!("Part 2: {}", mapped_ranges.iter().map(|ran| ran.0).min().unwrap());
        
    }
    
}
