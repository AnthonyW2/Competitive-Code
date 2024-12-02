pub mod day2 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut safe_reports = 0;
        
        let mut reports = Vec::new();
        
        for line in &lines {
            let levels = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            reports.push(levels);
        }
        
        for report in &reports {
            
            let mut inc = true;
            let mut dec = true;
            let mut safe = true;
            
            'levels: for j in 1..(report.len()) {
                
                let prev = report[j-1];
                let curr = report[j];
                
                if prev <= curr {
                    dec = false;
                }
                if prev >= curr {
                    inc = false;
                }
                if prev.abs_diff(curr) > 3 {
                    safe = false;
                    break 'levels;
                }
                
            }
            
            if safe && (inc || dec) {
                safe_reports += 1;
            }
            
        }
        
        println!("Part 1: {}", safe_reports);
        
        
        // ==== Part 2 ==== //
        
        let mut safe_reports = 0;
        
        for report in &reports {
            
            'ignore: for ignore in 0..(report.len()) {
                
                let mut inc = true;
                let mut dec = true;
                let mut safe = true;
                
                'levels: for j in 1..(report.len()) {
                    
                    // Skip ignored levels
                    if j == ignore || (j-1 == ignore && j == 1) {
                        continue 'levels;
                    }
                    
                    let prev = if j-1 == ignore { report[j-2] } else { report[j-1] };
                    let curr = report[j];
                    
                    if prev <= curr {
                        dec = false;
                    }
                    if prev >= curr {
                        inc = false;
                    }
                    if prev.abs_diff(curr) > 3 {
                        safe = false;
                        break 'levels;
                    }
                    
                }
                
                if safe && (inc || dec) {
                    safe_reports += 1;
                    break 'ignore;
                }
                
            }
            
        }
        
        println!("Part 2: {}", safe_reports);
        
    }
    
}
