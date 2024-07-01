pub mod day1 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        // ==== Part 1 ==== //
        
        let mut output = Vec::new();
        
        for line in &lines {
            
            let mut d0 = ' ';
            let mut d1 = ' ';
            
            for c in line.chars() {
                match c {
                    '0'..='9' => {
                        if d0 == ' ' {
                            d0 = c;
                        }
                        d1 = c;
                    },
                    _ => {}
                }
            }
            
            if d0 != ' ' && d1 != ' ' {
                output.push(format!("{d0}{d1}").parse::<u32>().unwrap());
            }
            
        }
        
        println!("Part 1: {}", output.iter().sum::<u32>());
        
        
        // ==== Part 2 ==== //
        
        let mut output2 = Vec::new();
        
        let patterns = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        
        for line in &lines {
            
            let mut left_digit = 0;
            let mut i = 0;
            'line_idx: while i < line.len() {
                
                let slice = &line[i..];
                
                for (pn, p) in patterns.iter().enumerate() {
                    if p.len() <= slice.len() && &slice[0..p.len()] == *p {
                        left_digit = (pn as u32 % 9) + 1;
                        break 'line_idx;
                    }
                }
                
                i += 1;
            }
            
            let mut right_digit = 0;
            let mut i = line.len()-1;
            'line_idx: loop {
                
                let slice = &line[i..];
                
                for (pn, p) in patterns.iter().enumerate() {
                    if p.len() <= slice.len() && &slice[0..p.len()] == *p {
                        right_digit = (pn as u32 % 9) + 1;
                        break 'line_idx;
                    }
                }
                
                if i == 0 { break; }
                i -= 1;
            }
            
            output2.push(left_digit*10 + right_digit);
            
        }
        
        println!("Part 2: {}", output2.iter().sum::<u32>());
        
    }
    
}
