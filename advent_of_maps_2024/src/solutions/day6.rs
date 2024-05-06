pub mod day6 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let arr = lines[0].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        
        let n1 = arr[0];
        let n2 = arr[1];
        
        let mut high_diffs = 0;
        
        for n in n1..=n2 {
            
            let mut num = n;
            
            while num > 9 {
                let str = format!("{}", num).chars().collect::<Vec<_>>();
            
                let mut new_str = String::new();
                
                for i in 0..(str.len()-1) {
                    let a = format!("{}", str[i]).parse::<i32>().unwrap();
                    let b = format!("{}", str[i+1]).parse::<i32>().unwrap();
                    
                    new_str.push_str(&format!("{}", (b-a).abs()));
                }
                
                num = new_str.parse::<u32>().unwrap();
                
            }
            
            if num == 0{
                high_diffs += 1;
            }
            
        }
        
        println!("Part 1: {}", high_diffs);
        
    }
    
}
