pub mod day10 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Count all instances of arr[i] > arr[j] when i < j
        
        let arr = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        let mut upsetness = 0;
        
        for i in 0..arr.len() {
            for j in i+1..(arr.len()) {
                if arr[i] > arr[j] {
                    upsetness += 1;
                }
            }
        }
        
        //let mut enum_arr = arr.iter().enumerate().collect::<Vec<_>>();
        //enum_arr.sort_by_cached_key(|(_i,a)| *a);
        //
        //for i in 0..enum_arr.len() {
        //    //if i < enum_arr[i].0 {
        //    //    upsetness += enum_arr[i].0 - i;
        //    //}
        //    if i > enum_arr[i].0 {
        //        upsetness += i - enum_arr[i].0;
        //    }
        //}
        
        println!("Solution: {}", upsetness);
        
    }
    
}
