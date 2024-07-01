pub mod day11 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Difference between two bounds
        
        // The maximum "trolling" number
        let mut max_trolling = 0;
        // The minimum not-trolling number
        let mut min_not_bad = u64::MAX;
        
        for (_li, line) in (&lines).iter().enumerate().skip(1) {
            let arr = line.split_ascii_whitespace().collect::<Vec<_>>();
            let num = arr[0].parse::<u64>().unwrap();
            
            if arr[1] == "TROLLING" {
                if max_trolling < num {
                    max_trolling = num;
                }
            } else {
                if min_not_bad > num {
                    min_not_bad = num;
                }
            }
        }
        
        println!("Solution: {}", min_not_bad - max_trolling - 1);
        
    }
    
}
