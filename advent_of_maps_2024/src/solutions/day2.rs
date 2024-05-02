pub mod day2 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let n = lines[0].parse::<u32>().unwrap();
        
        let mut x = 0;
        
        let mut res = 0;
        
        while x < n {
            if sum_digits(x)%7 == 0 {
                res += 1;
            }
            x += 1;
        }
        
        fn sum_digits(x: u32) -> u32 {
            if x < 10 {
                return x;
            }
            
            let mut x = x;
            let mut sum = 0;
            
            while x != 0 {
                sum += x % 10;
                x /= 10;
            }
            
            return sum;
        }
        
        println!("Part 1: {}", res);
        
    }
    
}
