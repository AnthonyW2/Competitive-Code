pub mod day1 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut similarity1 = 0;
        
        let mut l1 = Vec::new();
        let mut l2 = Vec::new();
        
        for line in (&lines).iter() {
            let split = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            
            l1.push(split[0]);
            l2.push(split[1]);
        }
        
        l1.sort();
        l2.sort();
        
        for i in 0..(&l1).len() {
            similarity1 += l1[i].abs_diff(l2[i]);
        }
        
        println!("Part 1: {}", similarity1);
        
        
        // ==== Part 2 ==== //
        
        let mut similarity2 = 0;
        
        for i in 0..(&l1).len() {
            let matches = l2.iter().filter(|n| **n == l1[i]).count();
            
            similarity2 += l1[i] * (matches as u32);
        }
        
        println!("Part 2: {}", similarity2);
        
    }
    
}
