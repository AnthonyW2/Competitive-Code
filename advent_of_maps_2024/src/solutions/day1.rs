pub mod day1 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        // ==== Part 1 ==== //
        
        let output = lines[0].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).sum::<u32>();
        
        println!("Part 1: {}", output);
        
    }
    
}
