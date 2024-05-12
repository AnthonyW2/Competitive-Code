pub mod day12 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Find the closest value to a target by summing bins
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        // Target (favourite) number
        let target = arr0[1];
        
        print!("Solutions: ");
        
        for (_li, line) in (&lines).iter().enumerate().skip(1) {
            
            let arr = line.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
            
            let x = arr[0];
            let y = arr[1];
            
            let mut previous_best = 0;
            let mut best = 0;
            let mut i = 1;
            
            while best < target {
                // Add the value in the ith bin to the current "best"
                best += i;
                if i%2 == 0 {
                    best += 1;
                }
                if i%y == 0 {
                    best += x;
                }
                
                // Check if we've passed the target value
                if best > target {
                    if best-target < target-previous_best {
                        // The value above the target value is closest
                        print!("{} ", best);
                        break;
                    } else {
                        // The value below the target value is closest
                        print!("{} ", previous_best);
                        break;
                    }
                }
                
                previous_best = best;
                
                i += 1;
            }
            
        }
        println!();
        
    }
    
}
