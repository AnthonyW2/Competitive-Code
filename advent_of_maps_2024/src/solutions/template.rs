pub mod day_ {
    
    pub fn solution(lines: Vec<String>) {
        
        // 
        
        let n = lines[0].parse::<u64>().unwrap();
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        let arr1 = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        for (li, line) in (&lines).iter().enumerate() {
            
        }
        
        println!("Solution: {}", "_");
        
    }
    
}
