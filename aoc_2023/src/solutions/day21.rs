pub mod day21 {
    
    pub fn solution(lines: Vec<String>) {
        
        let garden_map = lines.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        
        // ==== Part 1 ==== //
        
        const STEP_LIMIT: usize = 64;
        
        let empty_garden_map = lines.iter()
            .map(|s| s.chars()
                .map(|c| {
                    if c == 'S' { return ('.', false); }
                    (c, false)
                })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let mut current_garden_map = garden_map.iter()
            .map(|v| v.iter()
                .map(|c| {
                    if *c == 'S' { return ('.', true); }
                    (*c, false)
                })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let directions = [(0,1), (0,-1), (1,0), (-1,0)];
        
        for _ in 0..STEP_LIMIT {
            let mut new_garden_map = empty_garden_map.clone();
            
            for (r, row) in (&current_garden_map).iter().enumerate() {
                for (c, col) in row.iter().enumerate() {
                    if col.1 {
                        
                        for dir in directions {
                            let new_row = r as isize + dir.0;
                            let new_col = c as isize + dir.1;
                            if new_row >= 0 &&
                                new_col >= 0 &&
                                (new_row as usize) < current_garden_map.len() &&
                                (new_col as usize) < current_garden_map[0].len() &&
                                current_garden_map[new_row as usize][new_col as usize].0 == '.'
                            {
                                // Can move this direction
                                new_garden_map[new_row as usize][new_col as usize].1 = true;
                            }
                        }
                        
                    }
                }
            }
            
            current_garden_map = new_garden_map;
        }
        
        let mut garden_count = 0;
        
        for row in &current_garden_map {
            for col in row {
                if col.1 {
                    garden_count += 1;
                }
            }
        }
        
        println!("Part 1: {}", garden_count);
        
        
        // ==== Part 2 ==== //
        
        println!("Part 2: {}", "_");
        
    }
    
}
