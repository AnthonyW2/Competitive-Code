pub mod day13 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut patterns = Vec::new();
        let mut latest_pattern = Vec::new();
        for line in &lines {
            if line.len() == 0 {
                patterns.push(latest_pattern);
                latest_pattern = Vec::new();
            } else {
                latest_pattern.push(line.chars().collect::<Vec<_>>());
            }
        }
        patterns.push(latest_pattern);
        
        fn find_horizontal_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
            
            for i in 1..pattern.len() {
                
                let mut valid_line = true;
                for j in 1..pattern.len() {
                    
                    if j > i || i+j-1 >= pattern.len() {
                        break;
                    }
                    
                    if pattern[i-j] != pattern[i+j-1] {
                        valid_line = false;
                        break;
                    }
                    
                }
                
                if valid_line {
                    return Some(i);
                }
                
            }
            
            return None;
        }
        
        fn find_vertical_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
            
            for i in 1..pattern[0].len() {
                
                let mut valid_line = true;
                for j in 1..pattern[0].len() {
                    
                    if j > i || i+j-1 >= pattern[0].len() {
                        break;
                    }
                    
                    let left_line = pattern.iter().map(|v| v[i-j]);
                    let right_line = pattern.iter().map(|v| v[i+j-1]);
                    if !left_line.eq(right_line) {
                        valid_line = false;
                        break;
                    }
                    
                }
                
                if valid_line {
                    return Some(i);
                }
                
            }
            
            return None;
        }
        
        let mut result1 = 0;
        
        for pattern in &patterns {
            
            let h_idx = find_horizontal_reflection(pattern);
            if h_idx.is_some() {
                result1 += h_idx.unwrap()*100;
            }
            
            let v_idx = find_vertical_reflection(pattern);
            if v_idx.is_some() {
                result1 += v_idx.unwrap();
            }
            
        }
        
        println!("Part 1: {}", result1);
        
        
        // ==== Part 2 ==== //
        
        fn find_smudged_horizontal_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
            
            for i in 1..pattern.len() {
                
                let mut differences = 0;
                
                for j in 1..pattern.len() {
                    
                    if j > i || i+j-1 >= pattern.len() {
                        break;
                    }
                    
                    differences += pattern[i-j].iter().zip(pattern[i+j-1].iter()).filter(|&(a,b)| a != b).count();
                    
                }
                
                if differences == 1 {
                    return Some(i);
                }
                
            }
            
            return None;
        }
        
        fn find_smudged_vertical_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
            
            for i in 1..pattern[0].len() {
                
                let mut differences = 0;
                
                for j in 1..pattern[0].len() {
                    
                    if j > i || i+j-1 >= pattern[0].len() {
                        break;
                    }
                    
                    let left_line = pattern.iter().map(|v| v[i-j]);
                    let right_line = pattern.iter().map(|v| v[i+j-1]);
                    differences += left_line.zip(right_line).filter(|&(a,b)| a != b).count();
                    
                }
                
                if differences == 1 {
                    return Some(i);
                }
                
            }
            
            return None;
        }
        
        let mut result2 = 0;
        
        for pattern in &patterns {
            
            let h_idx = find_smudged_horizontal_reflection(pattern);
            if h_idx.is_some() {
                result2 += h_idx.unwrap()*100;
            }
            
            let v_idx = find_smudged_vertical_reflection(pattern);
            if v_idx.is_some() {
                result2 += v_idx.unwrap();
            }
            
        }
        
        println!("Part 2: {}", result2);
        
    }
    
}
