pub mod day9 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        
        // ==== Parts 1 & 2 ==== //
        
        let mut next_vals = Vec::new(); // part 1
        let mut previous_vals = Vec::new(); // part 2
        
        for line in &lines {
            
            let mut diffs = Vec::new();
            diffs.push(line.split(' ').map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>());
            
            while diffs[diffs.len()-1].len() > 1 && (diffs[diffs.len()-1][diffs[diffs.len()-1].len()-1] != 0 || diffs[diffs.len()-1][0] != 0) {
                
                let mut new_diff = Vec::new();
                for i in 0..(diffs[diffs.len()-1].len()-1) {
                    new_diff.push(diffs[diffs.len()-1][i+1] - diffs[diffs.len()-1][i]);
                }
                diffs.push(new_diff);
                
            }
            
            // Work back up the diffs & extrapolate
            for (i, diff) in diffs.clone().iter().enumerate().rev() {
                if i == diffs.len()-1 {
                    diffs[i].push(0);
                    diffs[i].insert(0,0);
                } else {
                    let next_diff = diffs[i+1].clone();
                    diffs[i].push(diff[diff.len()-1] + next_diff[next_diff.len()-1]); // part 1
                    diffs[i].insert(0,diff[0] - next_diff[0]); // part 2
                }
            }
            
            next_vals.push(diffs[0][diffs[0].len()-1]);
            previous_vals.push(diffs[0][0]);
            
        }
        
        println!("Part 1: {}", next_vals.iter().sum::<i64>());
        println!("Part 2: {}", previous_vals.iter().sum::<i64>());
        
    }
    
}
