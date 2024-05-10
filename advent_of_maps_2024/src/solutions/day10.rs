pub mod day10 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Count all instances of arr[i] > arr[j] when i < j
        
        let arr = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        //println!("{}", arr1.len());
        
        let mut upsetness = 0;
        
        for i in 0..arr.len() {
            for j in i+1..(arr.len()) {
                if arr[i] > arr[j] {
                    upsetness += 1;
                }
            }
            //println!("{}", i);
        }
        
        println!("Solution: {}", upsetness);
        
    }
    
}
