pub mod day26 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Find the kth largest subarray sum
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        let nums = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        let k = arr0[1] as usize;
        
        let mut rolling_sum = vec![nums[0]];
        
        for i in 1..nums.len() {
            rolling_sum.push(rolling_sum[i-1] + nums[i]);
        }
        
        /// Get the sum of a subarray, given the rolling sum
        fn get_sub_sum(rolling_sum: &Vec<u64>, start: usize, end: usize) -> u64 {
            if start > 0 {
                return rolling_sum[end] - rolling_sum[start-1];
            } else {
                return rolling_sum[end];
            }
        }
        
        
        // O(n * n * log(n) * log(n)) solution:
        let mut sums = Vec::new();
        
        for i in 0..rolling_sum.len() {
            for j in (i+1)..rolling_sum.len() {
                
                let sum = get_sub_sum(&rolling_sum,i,j);
                sums.push(sum);
                
            }
        }
        //println!("Found all sums");
        
        sums.sort_unstable();
        
        println!("Solution: {}", sums[sums.len() - k as usize]);
        
    }
    
}
