pub mod day15 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Cheating at slots with dynamic programming
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let arr1 = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        // Keys: p, iteration
        let mut cache: Vec<Vec<Option<u64>>> = vec![vec![None; arr0[2]]; arr1.len()];
        
        // Return minimum
        pub fn dp_sol(p: usize, i: usize, l: usize, s: usize, nums: &Vec<u64>, cache: &mut Vec<Vec<Option<u64>>>) -> u64 {
            
            if i == s {
                // Reached target number of iterations
                return 0;
            }
            
            if cache[p][i].is_none() {
                // Not cached, need to calculate answer
                
                let rollover = nums[(p+l)%nums.len()] + dp_sol((p+l)%nums.len(), i+1, l, s, nums, cache);
                let no_rollover = nums[(p+l+1)%nums.len()] + dp_sol((p+l+1)%nums.len(), i+1, l, s, nums, cache);
                
                // Return the minimum score from each path
                cache[p][i] = Some(rollover.min(no_rollover));
                
            }
            
            return cache[p][i].unwrap();
            
        }
        
        println!("Solution: {}", dp_sol(0, 0, arr0[1], arr0[2], &arr1, &mut cache));
        
    }
    
}
