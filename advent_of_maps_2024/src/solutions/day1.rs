pub mod day1 {
    
    pub fn solution(lines: Vec<String>) {
        
        // A + B
        
        let output = lines[0].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).sum::<u32>();
        
        println!("A+B: {}", output);
        
    }
    
}
