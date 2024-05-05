pub mod day_ {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let n = lines[0].parse::<u32>().unwrap();
        
        let arr = lines[1].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        
        for (li, line) in (&lines).iter().enumerate() {
            
        }
        
        println!("Part 1: {}", "_");
        
    }
    
}
