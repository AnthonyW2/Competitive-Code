pub mod day8 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Count the number of pairs of lines that hash to the same value
        
        let mut hashes = Vec::new();
        
        for line in (&lines).iter() {
            
            let char1 = line.chars().next().unwrap();
            let char2 = line.chars().last().unwrap();
            
            let hash = char1 as u8 + char2 as u8;
            
            hashes.push(hash);
            
        }
        
        let mut count = 0;
        
        for i in 0..hashes.len() {
            for j in (i+1)..hashes.len() {
                
                if hashes[i] == hashes[j] {
                    count += 1;
                }
                
            }
        }
        
        println!("Number of pairs: {}", count);
        
    }
    
}
