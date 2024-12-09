pub mod day6 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        // x, y
        let mut start_pos = (0,0);
        let start_vel = (0,-1);
        
        let mut start_map = Vec::new();
        
        for (li, line) in (&lines).iter().enumerate() {
            
            start_map.push(line.chars().collect::<Vec<_>>());
            
            for (j, c) in (&start_map[li]).iter().enumerate() {
                if *c == '^' {
                    start_pos = (j as isize, li as isize);
                }
            }
            
        }
        
        start_map[start_pos.1 as usize][start_pos.0 as usize] = 'X';
        
        let mut map = start_map.clone();
        let mut pos = start_pos.clone();
        let mut vel = start_vel.clone();
        
        // Returns whether or not the guard is still in bounds, new position, new velocity, and trigger status
        fn simulate_step (map: &Vec<Vec<char>>, pos: (isize, isize), vel: (isize, isize), trigger_pos: (isize, isize)) -> (bool, (isize, isize), (isize, isize), bool) {
            
            let mut new_pos = (pos.0 + vel.0, pos.1 + vel.1);
            let mut new_vel = vel;
            
            let mut trigger = false;
            
            // While intersecting an obstacle
            while new_pos.0 >= 0 && new_pos.0 < map[0].len() as isize && new_pos.1 >= 0 && new_pos.1 < map.len() as isize && map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                
                // Trigger
                if new_pos == trigger_pos {
                    trigger = true;
                }
                
                new_vel = (-new_vel.1, new_vel.0);
                new_pos = (pos.0 + new_vel.0, pos.1 + new_vel.1);
                
            }
            
            // Bounds check
            if new_pos.0 < 0 || new_pos.0 >= map[0].len() as isize || new_pos.1 < 0 || new_pos.1 >= map.len() as isize {
                return (false, new_pos, vel, trigger);
            }
            
            return (true, new_pos, new_vel, trigger);
            
        }
        
        loop {
            let result = simulate_step(&map, pos, vel, (-1,-1));
            pos = result.1;
            vel = result.2;
            if !result.0 {
                break;
            }
            map[pos.1 as usize][pos.0 as usize] = 'X';
        }
        
        let mut marked_count = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 'X' {
                    marked_count += 1;
                }
            }
        }
        
        println!("Part 1: {}", marked_count);
        
        
        // ==== Part 2 ==== //
        
        // Check if adding an obstacle results in an infinite loop
        fn test_obstacle (map: &Vec<Vec<char>>, start_pos: (isize, isize), start_vel: (isize, isize), obstacle: (isize, isize)) -> bool {
            let mut new_map = map.clone();
            new_map[obstacle.1 as usize][obstacle.0 as usize] = '#';
            
            let mut pos = start_pos.clone();
            let mut vel = start_vel.clone();
            
            let mut seen_new_obstacle = (-1,-1);
            
            let mut step_count = 0;
            
            loop {
                let result = simulate_step(&new_map, pos, vel, obstacle);
                pos = result.1;
                vel = result.2;
                step_count += 1;
                if result.3 {
                    if seen_new_obstacle == (-1,-1) {
                        seen_new_obstacle = pos;
                    } else if seen_new_obstacle == pos {
                        return true;
                    }
                }
                if !result.0 {
                    break;
                }
                
                if step_count > map.len() * map[0].len() {
                    // It's plausible that the loop does not involve the obstacle.
                    // In this case, just detect a loop by the number of steps.
                    return true;
                }
                new_map[pos.1 as usize][pos.0 as usize] = 'X';
            }
            
            return false;
        }
        
        let mut loop_count = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if (x as isize, y as isize) != start_pos && map[y][x] == 'X' {
                    if test_obstacle(&map, start_pos, start_vel, (x as isize, y as isize)) {
                        loop_count += 1;
                    }
                }
            }
        }
        
        println!("Part 2: {}", loop_count);
        
    }
    
}
