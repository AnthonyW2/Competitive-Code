pub mod day9 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Minimum & macimum possible imposters
        
        print!("Solutions: ");
        
        for (li, line) in (&lines).iter().enumerate() {
            if li > 0 {
                
                let arr = line.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
                
                let n = arr[0];
                let k = arr[1];
                let x = arr[2];
                
                print!("{} {} ", 1.max(x-(n-k)), (x-1).min(k));
                
            }
        }
        
        println!();
        
    }
    
}
