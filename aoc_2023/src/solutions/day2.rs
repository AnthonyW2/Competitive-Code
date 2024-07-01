pub mod day2 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        
        // ==== Part 1 ==== //
        
        let limits = std::collections::HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);
        
        let mut id_sum = 0;
        
        // Iterate through games
        for line in &lines {
            let split = line.split(": ").collect::<Vec<_>>();
            let game_id = split[0].split(' ').nth(1).unwrap().parse::<u32>().unwrap();
            let sets = split[1].split("; ").map(|s| s.split(", ").collect::<Vec<_>>()).collect::<Vec<_>>();
            
            let mut valid = true;
            // Iterate through sets
            'sets: for set in sets {
                // Iterate through individual die colours
                for cubes in set {
                    let split2 = cubes.split(' ').collect::<Vec<_>>();
                    // Check if the number of dice is valid
                    if split2[0].parse::<u32>().unwrap() > limits[split2[1]] {
                        valid = false;
                        break 'sets;
                    }
                }
            }
            
            if valid {
                id_sum += game_id;
            }
        }
        
        println!("Part 1: {}", id_sum);
        
        
        // ==== Part 2 ==== //
        
        let mut sum2 = 0;
        
        // Iterate through games
        for line in &lines {
            let split = line.split(": ").collect::<Vec<_>>();
            let sets = split[1].split("; ").map(|s| s.split(", ").collect::<Vec<_>>()).collect::<Vec<_>>();
            
            let mut minimums = std::collections::HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ]);
            
            for set in sets {
                for cubes in set {
                    let split2 = cubes.split(' ').collect::<Vec<_>>();
                    let color = split2[1];
                    // Update the minimum number of this colour
                    minimums.insert(color, std::cmp::max(minimums[color], split2[0].parse::<u32>().unwrap()));
                }
            }
            
            sum2 += minimums["red"]*minimums["green"]*minimums["blue"];
            
        }
        
        println!("Part 2: {}", sum2);
        
    }
    
}
