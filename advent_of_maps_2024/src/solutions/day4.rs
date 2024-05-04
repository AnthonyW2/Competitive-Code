pub mod day4 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let arr = lines[1].split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        
        let mut max_length = 0;
        
        //for i in 0..arr.len() {
        //    let mut sub_length = 0;
        //    'inner: for j in i..arr.len() {
        //        if arr[i]%2 == arr[j]%2 {
        //            sub_length += 1;
        //        } else {
        //            break 'inner;
        //        }
        //    }
        //    if sub_length > max_length {
        //        max_length = sub_length;
        //    }
        //}
        //
        //println!("Part 1: {}", max_length);
        
        let mut start = 0;
        for i in 0..arr.len() {
            if arr[i]%2 != arr[start]%2 {
                if i-start > max_length {
                    max_length = i-start;
                }
                start = i;
            }
        }
        
        println!("Part 1: {}", max_length);
        
    }
    
}
