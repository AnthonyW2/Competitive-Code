pub mod day5 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let n = lines[0].parse::<u32>().unwrap();
        
        println!("Part 1: {}", (n*123+456)/23);
        
    }
    
}
