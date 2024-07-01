pub mod day14 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Minimum quarter-split crackers
        
        let n = lines[0].parse::<u64>().unwrap();
        
        let quarters = n/4;
        let halves = (n%4)/2;
        let wholes = n%2;
        
        println!("Crackers: {}", quarters + halves + wholes);
        
    }
    
}
