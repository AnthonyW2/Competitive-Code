pub mod day24 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Calculate how long an object is within a range when falling
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<u128>().unwrap()).collect::<Vec<_>>();
        
        let n = arr0[0];
        let m = arr0[1] as f64;
        
        // Calculate the velocity after falling n blocks
        let v1_squared = (2*n) as f64;
        let v1 = v1_squared.sqrt();
        
        // Calculate the amount of time between n and m+n blocks
        let safe_ticks = (v1_squared + 2.0*m + 2.0).sqrt() - v1;
        
        println!("Solution: {}", safe_ticks.round());
        
    }
    
}
