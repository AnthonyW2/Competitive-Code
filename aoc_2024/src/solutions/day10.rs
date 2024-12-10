pub mod day10 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut map = Vec::new();
        
        for line in &lines {
            
            let nums = line.chars().map(|c| (c.to_digit(10).unwrap() as u64, false)).collect::<Vec<_>>();
            map.push(nums);
            
        }
        
        fn find_endpoints(map: &mut Vec<Vec<(u64, bool)>>, start: (usize, usize)) -> u64 {
            
            let mut score = 0;
            
            let current_height = map[start.1][start.0].0;
            
            map[start.1][start.0].1 = true;
            
            if current_height == 9 {
                return 1;
            }
            
            if start.0 > 0 && !map[start.1][start.0-1].1 && map[start.1][start.0-1].0 == current_height+1 {
                score += find_endpoints(map, (start.0-1, start.1));
            }
            if start.0 < map[0].len() - 1 && !map[start.1][start.0+1].1 && map[start.1][start.0+1].0 == current_height+1 {
                score += find_endpoints(map, (start.0+1, start.1));
            }
            
            if start.1 > 0 && !map[start.1-1][start.0].1 && map[start.1-1][start.0].0 == current_height+1 {
                score += find_endpoints(map, (start.0, start.1-1));
            }
            if start.1 < map.len() - 1 && !map[start.1+1][start.0].1 && map[start.1+1][start.0].0 == current_height+1 {
                score += find_endpoints(map, (start.0, start.1+1));
            }
            
            return score;
            
        }
        
        let mut sum1 = 0;
        
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x].0 == 0 {
                    sum1 += find_endpoints(&mut (map.clone()), (x,y));
                }
            }
        }
        
        println!("Part 1: {}", sum1);
        
        
        // ==== Part 2 ==== //
        
        fn find_rating(map: &mut Vec<Vec<(u64, bool)>>, start: (usize, usize)) -> u64 {
            
            let mut score = 0;
            
            let current_height = map[start.1][start.0].0;
            
            if current_height == 9 {
                return 1;
            }
            
            if start.0 > 0 && map[start.1][start.0-1].0 == current_height+1 {
                score += find_rating(map, (start.0-1, start.1));
            }
            if start.0 < map[0].len() - 1 && map[start.1][start.0+1].0 == current_height+1 {
                score += find_rating(map, (start.0+1, start.1));
            }
            
            if start.1 > 0 && map[start.1-1][start.0].0 == current_height+1 {
                score += find_rating(map, (start.0, start.1-1));
            }
            if start.1 < map.len() - 1 && map[start.1+1][start.0].0 == current_height+1 {
                score += find_rating(map, (start.0, start.1+1));
            }
            
            return score;
            
        }
        
        let mut sum2 = 0;
        
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x].0 == 0 {
                    sum2 += find_rating(&mut (map.clone()), (x,y));
                }
            }
        }
        
        println!("Part 2: {}", sum2);
        
    }
    
}
