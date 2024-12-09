pub mod day9 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let input = (&lines[0]).chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<_>>();
        
        // (ID, size)
        // For empty space: ID = -1
        let mut expanded = Vec::new();
        
        for i in 0..input.len() {
            
            if i % 2 == 0 {
                expanded.push(((i/2) as i64, input[i]));
            } else {
                expanded.push((-1, input[i]));
            }
            
        }
        
        let mut fs1 = expanded.clone();
        
        let mut gap = 1;
        while gap < fs1.len() {
            
            // Advance the gap pointer
            if fs1[gap].0 != -1 {
                gap += 1;
                continue;
            }
            
            // Remove 0-space gaps
            if fs1[gap].1 == 0 {
                fs1.remove(gap);
                continue;
            }
            
            // Remove trailing empty space
            if fs1[fs1.len() - 1].0 == -1 {
                fs1.remove(fs1.len() - 1);
                continue;
            }
            
            if fs1[gap].1 == fs1[fs1.len() - 1].1 {
                // Move last item into gap
                fs1[gap] = fs1[fs1.len() - 1];
                fs1.remove(fs1.len() - 1);
            } else if fs1[gap].1 > fs1[fs1.len() - 1].1 {
                // Move last item before gap and resize gap
                fs1[gap].1 = fs1[gap].1 - fs1[fs1.len() - 1].1;
                fs1.insert(gap, fs1[fs1.len() - 1]);
                fs1.remove(fs1.len() - 1);
                gap += 1;
            } else {
                // Fill gap with last item and resize last item
                let last = fs1.len() - 1;
                fs1[last] = (fs1[last].0, fs1[last].1 - fs1[gap].1);
                fs1[gap] = (fs1[last].0, fs1[gap].1);
            }
            
        }
        
        // Calculate the filesystem checksum
        fn checksum (filesystem: Vec<(i64, i64)>) -> i64 {
            let mut sum = 0;
            
            let mut global_idx = 0;
            
            for i in 0..filesystem.len() {
                for _ in 0..filesystem[i].1 {
                    if filesystem[i].0 != -1 {
                        sum += global_idx * filesystem[i].0;
                    }
                    global_idx += 1;
                }
            }
            
            return sum;
        }
        
        println!("Part 1: {}", checksum(fs1));
        
        
        // ==== Part 2 ==== //
        
        let mut fs2 = expanded.clone();
        
        let mut last = fs2[fs2.len() - 1].0;
        let mut last_idx = fs2.len() - 1;
        
        // Remove an item from the filesystem, consolidating surrounding free space
        fn remove_item (filesystem: &mut Vec<(i64, i64)>, idx:  usize) {
            let mut idx = idx;
            
            let mut total_size = filesystem[idx].1;
            
            if idx > 0 && filesystem[idx-1].0 == -1 {
                total_size += filesystem[idx-1].1;
                filesystem.remove(idx-1);
                idx -= 1;
            }
            if idx < filesystem.len() - 1 && filesystem[idx+1].0 == -1 {
                total_size += filesystem[idx+1].1;
                filesystem.remove(idx+1);
            }
            
            filesystem[idx] = (-1, total_size);
            
        }
        
        while last_idx > 0 && last > 0 {
            
            // Skip empty space
            if fs2[last_idx].0 != last {
                last_idx -= 1;
                continue;
            }
            
            'findgap: for i in 0..last_idx {
                if fs2[i].0 == -1 {
                    if fs2[i].1 == fs2[last_idx].1 {
                        // Move last item into gap
                        fs2[i] = fs2[last_idx];
                        remove_item(&mut fs2, last_idx);
                        break 'findgap;
                    } else if fs2[i].1 > fs2[last_idx].1 {
                        // Move last item before gap and resize gap
                        fs2[i].1 = fs2[i].1 - fs2[last_idx].1;
                        fs2.insert(i, fs2[last_idx]);
                        last_idx += 1;
                        remove_item(&mut fs2, last_idx);
                        break 'findgap;
                    }
                }
            }
            last -= 1;
            last_idx -= 1;
            
        }
        
        println!("Part 2: {}", checksum(fs2));
        
    }
    
}
