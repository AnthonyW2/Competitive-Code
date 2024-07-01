pub mod day20 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Find the (contiguous) subarray with the maximal sum
        
        let arr1 = lines[1].split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        
        let mut max_sum = 0;
        
        for i in 0..arr1.len() {
            let mut sum = 0;
            
            for j in i..arr1.len() {
                
                sum += arr1[j];
                
                if sum > max_sum {
                    max_sum = sum;
                }
                
            }
        }
        
        println!("Solution: {}", max_sum);
        
    }
    
}
