pub mod day6 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Diff
        
        let arr = lines[0].split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        
        let n1 = arr[0];
        let n2 = arr[1];
        
        let mut high_diff_count = 0;
        
        for n in n1..=n2 {
            
            let mut num = n;
            
            while num > 9 {
                let digits = format!("{}", num).chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
                
                num = 0;
                
                for i in 0..(digits.len()-1) {
                    let a = digits[i];
                    let b = digits[i+1];
                    
                    num += (b-a).abs() * 10i32.pow((digits.len()-i-2) as u32);
                }
                
            }
            
            if num == 0{
                high_diff_count += 1;
            }
            
        }
        
        println!("High diffs: {}", high_diff_count);
        
    }
    
}
