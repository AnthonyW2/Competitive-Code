pub mod day18 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Count runs of the same character
        
        const PLACEHOLDER: char = '#';
        
        let mut curr_char = PLACEHOLDER;
        let mut run_len = 0;
        
        let mut output = String::new();
        
        for c in lines[0].chars() {
            
            if curr_char == c {
                // Same character - increment counter
                run_len += 1;
            } else {
                // End of a run - add the length & character of the previous run to the output
                if curr_char != PLACEHOLDER {
                    if run_len > 1 {
                        output.push_str(&format!("{}{}", run_len, curr_char));
                    } else {
                        output.push_str(&format!("{}", curr_char));
                    }
                }
                // Start a new run
                run_len = 1;
                curr_char = c;
            }
            
        }
        
        // Add last run of characters
        if run_len > 1 {
            output.push_str(&format!("{}{}", run_len, curr_char));
        } else {
            output.push_str(&format!("{}", curr_char));
        }
        
        println!("Solution: {}", output.len());
        
    }
    
}
