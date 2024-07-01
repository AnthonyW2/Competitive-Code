pub mod day5 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Moon runes (morse code)
        
        let n = lines[0].parse::<u32>().unwrap();
        
        println!("Solution: {}", (n*123+456)/23);
        
    }
    
}
