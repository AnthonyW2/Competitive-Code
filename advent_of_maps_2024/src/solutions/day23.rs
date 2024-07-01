pub mod day23 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Take the XOR of two integers, and raise one result to the power of the other
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<u128>().unwrap()).collect::<Vec<_>>();
        
        //let solution = (arr0[0]^arr0[2]).pow((arr0[1]^arr0[3]) as u32);
        //
        //println!("Solution: {}", solution);
        
        println!("Solution: {}^{}", arr0[0]^arr0[2], arr0[1]^arr0[3]);
        
    }
    
}
