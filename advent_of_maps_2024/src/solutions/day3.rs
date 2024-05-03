pub mod day3 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        //let n = lines[0].parse::<u32>().unwrap();
        
        let target: u32 = 12321;
        
        let arr = lines[1].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        
        let mut count = 0;
        
        for i in 0..arr.len() {
            for j in i..arr.len() {
                if arr[i] + arr[j] == target {
                    count += 1;
                }
            }
        }
        
        println!("Part 1: {}", count);
        
    }
    
}
