pub mod day26 {
    
    //use std::collections::BinaryHeap;
    
    pub fn solution(lines: Vec<String>) {
        
        // Find the kth largest subarray sum
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let nums = lines[1].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        
        let n = nums.len();
        let k = arr0[1] as usize;
        
        let mut prefix_sums = vec![nums[0]];
        //let mut prefix_sums = vec![0, nums[0]];
        
        for i in 1..nums.len() {
            prefix_sums.push(prefix_sums[i-1] + nums[i]);
            //prefix_sums.push(prefix_sums[i] + nums[i]);
        }
        
        /// Get the sum of a subarray (with inclusive bounds), given the rolling sum
        #[inline(always)]
        fn get_sub_sum(prefix_sums: &Vec<u32>, start: usize, end: usize) -> u32 {
            if start > 0 {
                return prefix_sums[end] - prefix_sums[start-1];
            } else {
                return prefix_sums[end];
            }
            //return prefix_sums[end+1] - prefix_sums[start];
        }
        
        
        
        // O(n*n * log(n*n)) solution
        // Uses over 4.5GiB of memory (when using u32)
        let mut sums = Vec::new();
        
        // Iterate through all subarray lengths
        for l in 1..n {
            // Iterate through all starting indices
            for i in 0..(n-l) {
                let sum = get_sub_sum(&prefix_sums,i,i+l);
                sums.push(sum);
            }
        }
        //println!("Found all {} sums", sums.len());
        
        sums.sort_unstable();
        
        println!("Solution: {}", sums[sums.len() - k as usize]);
        
        
        
        /*
        // O(k log(sqrt(k))) solution
        // Slower than the solution above in practice, but has a very small memory footprint
        let mut pqueue: BinaryHeap<(u32, (usize, usize))> = BinaryHeap::new();
        pqueue.push((
            get_sub_sum(&prefix_sums, 0,n-1),
            (0, n-1)
        ));
        
        let mut counter = 1;
        
        let mut visited = vec![Vec::new(); n];
        for i in 0..n {
            visited[i] = vec![false; n-i];
        }
        
        visited[0][n-1] = true;
        
        while counter < k {
            
            let current_range = pqueue.pop().unwrap().1;
            
            if current_range.1 > current_range.0 {
                // Check subarray with decremented end
                if !visited[current_range.0][current_range.1-current_range.0-1] {
                    pqueue.push((
                        get_sub_sum(&prefix_sums, current_range.0,current_range.1-1),
                        (current_range.0, current_range.1-1)
                    ));
                    visited[current_range.0][current_range.1-current_range.0-1] = true;
                }
                
                // Check subarray with incremented start
                if !visited[current_range.0+1][current_range.1-current_range.0-1] {
                    pqueue.push((
                        get_sub_sum(&prefix_sums, current_range.0+1,current_range.1),
                        (current_range.0+1, current_range.1)
                    ));
                    visited[current_range.0+1][current_range.1-current_range.0-1] = true;
                }
                
            }
            
            counter += 1;
        }
        
        println!("Solution: {}", pqueue.pop().unwrap().0);
        */
        
    }
    
}
