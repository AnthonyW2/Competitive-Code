pub mod day16 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Minimum swaps required to sort list
        
        let n = lines[0].parse::<usize>().unwrap();
        
        // For each mind (index), store the body it is currently in
        let mut minds = vec![0; n];
        
        // Populate the minds list
        for (_li, line) in (&lines).iter().enumerate().skip(1) {
            let arr = line.split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let a = arr[0];
            let b = arr[1];
            
            minds[b-1] = a-1;
        }
        
        let mut swap_count = 0;
        
        // Repeatedly move each mind to its correct body until the list is sorted
        let mut sorted = false;
        while !sorted {
            sorted = true;
            
            for i in 0..n {
                if minds[i] != i {
                    let temp = minds[i];
                    minds[i] = minds[temp];
                    minds[temp] = temp;
                    
                    swap_count += 1;
                    sorted = false;
                }
            }
        }
        
        println!("Solution: {}", swap_count);
        
    }
    
}
