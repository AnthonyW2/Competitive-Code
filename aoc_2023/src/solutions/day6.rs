pub mod day6 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        
        // ==== Part 1 ==== //
        
        let times = (lines[0].split_whitespace().collect::<Vec<_>>())[1..].iter().map(|t| t.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let distances = (lines[1].split_whitespace().collect::<Vec<_>>())[1..].iter().map(|t| t.parse::<u32>().unwrap()).collect::<Vec<_>>();
        
        //println!("{:?}", times);
        //println!("{:?}", distances);
        
        let mut ways_to_win = Vec::new();
        
        for race in 0..times.len() {
            
            let total_time = times[race] as f64;
            let record_dist = distances[race] as f64;
            
            // Calculate lower & upper bounds for the time needed to hold the button to beat the record
            let mut lower_limit = ( total_time - (total_time*total_time - 4f64*record_dist).sqrt() )/2f64;
            let mut upper_limit = ( total_time + (total_time*total_time - 4f64*record_dist).sqrt() )/2f64;
            
            if lower_limit % 1f64 == 0f64 {
                lower_limit += 1f64;
            }
            if upper_limit % 1f64 == 0f64 {
                upper_limit -= 1f64;
            }
            
            //println!("{} {}", lower_limit, upper_limit);
            
            ways_to_win.push((upper_limit.floor()-lower_limit.ceil()+1f64) as u64);
            
        }
        
        //println!("{:?}", ways_to_win);
        
        println!("Part 1: {}", ways_to_win.iter().product::<u64>());
        
        
        // ==== Part 2 ==== //
        
        let total_time = lines[0].split(":").last().unwrap().replace(" ", "").parse::<f64>().unwrap();
        let record_dist = lines[1].split(":").last().unwrap().replace(" ", "").parse::<f64>().unwrap();
        
        // Calculate lower & upper bounds for the time needed to hold the button to beat the record
        let mut lower_limit = ( total_time - (total_time*total_time - 4f64*record_dist).sqrt() )/2f64;
        let mut upper_limit = ( total_time + (total_time*total_time - 4f64*record_dist).sqrt() )/2f64;
        
        if lower_limit % 1f64 == 0f64 {
            lower_limit += 1f64;
        }
        if upper_limit % 1f64 == 0f64 {
            upper_limit -= 1f64;
        }
        
        //println!("{} {}", lower_limit, upper_limit);
        
        println!("Part 2: {}", (upper_limit.floor()-lower_limit.ceil()+1f64) as u64);
        
    }
    
}
