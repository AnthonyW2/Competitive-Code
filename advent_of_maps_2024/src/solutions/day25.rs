pub mod day25 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Return the number of rectangles with more white squares than black squares on an n×n chess board
        
        // O(n) solution:
        //let n = lines[0].parse::<usize>().unwrap();
        //let mut result = 0;
        //for i in (1..n).step_by(2) {
        //    result = (result + (n-i+1)*(n-i+1)*((n-i+1)/2)/2) % 1000000007;
        //}
        //println!("Solution: {}", result);
        
        // O(1) solution:
        let n = lines[0].parse::<u128>().unwrap();
        println!("Solution: {}", ((n*n*(n+2)*(n+2))/32) % 1000000007);
        
    }
    
}
