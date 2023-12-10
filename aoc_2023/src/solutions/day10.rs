pub mod day10 {
    
    use std::collections::VecDeque;
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        
        // ==== Part 1 ==== //
        
        let mut start = (0,0); // row, col
        
        // Relative directions (col, row) associated with pipes
        let pipe_directions = HashMap::from([
            ('|', vec![(-1,0),(1,0)]),
            ('-', vec![(0,-1),(0,1)]),
            ('L', vec![(-1,0),(0,1)]),
            ('J', vec![(-1,0),(0,-1)]),
            ('7', vec![(1,0),(0,-1)]),
            ('F', vec![(1,0),(0,1)]),
            ('S', vec![(1,0),(-1,0),(0,1),(0,-1)]),
        ]);
        let valid_tiles = ['|', '-', 'L', 'J', '7', 'F'];
        
        let mut pipe_map = lines.iter()
            .enumerate()
            .map(|(row, s)| s.chars().enumerate().map(|(col, c)| {
                if c == 'S' {
                    start = (row,col);
                    return (c, 1);
                }
                return (c, 0);
            }).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        // Determine what type of pipe the S (start) is
        // This is necessary for the part 2 solution.
        let mut valid_starting_dirs = Vec::new();
        for dir in &pipe_directions[&'S'] {
            let row = start.0 as isize + dir.0;
            let col = start.1 as isize + dir.1;
            if row >= 0 &&
                col >= 0 &&
                (row as usize) < pipe_map.len() &&
                (col as usize) < pipe_map[0].len() &&
                pipe_map[row as usize][col as usize].1 == 0 &&
                valid_tiles.contains(&pipe_map[row as usize][col as usize].0) &&
                pipe_directions[&pipe_map[row as usize][col as usize].0].contains( &(-dir.0, -dir.1) )
            {
                // Can move this direction
                valid_starting_dirs.push(dir);
            }
        }
        let start_type = pipe_directions.iter().find(|dir| {
            if dir.0 != &'S' && dir.1.contains(valid_starting_dirs[0]) && dir.1.contains(valid_starting_dirs[1]) {
                return true;
            }
            false
        }).unwrap().0;
        pipe_map[start.0][start.1].0 = *start_type;
        
        
        // Use BFS to follow the pipes in both directions
        let mut agent: (usize, usize, u32) = (0,0,0); // row, col
        let mut branches: VecDeque<(usize, usize, u32)> = VecDeque::from([(start.0, start.1, 0)]);
        
        while branches.len() > 0 {
            agent = branches.pop_front().unwrap();
            
            let cur_pipe_type = pipe_map[agent.0][agent.1].0;
            
            for dir in &pipe_directions[&cur_pipe_type] {
                let row = agent.0 as isize + dir.0;
                let col = agent.1 as isize + dir.1;
                if row >= 0 &&
                    col >= 0 &&
                    (row as usize) < pipe_map.len() &&
                    (col as usize) < pipe_map[0].len() &&
                    pipe_map[row as usize][col as usize].1 == 0 &&
                    valid_tiles.contains(&pipe_map[row as usize][col as usize].0) &&
                    pipe_directions[&pipe_map[row as usize][col as usize].0].contains( &(-dir.0, -dir.1) )
                {
                    // Can move this direction
                    branches.push_back((row as usize, col as usize, agent.2+1));
                    pipe_map[row as usize][col as usize].1 = agent.2+1; // mark where we've been
                }
            }
            
        }
        
        println!("Part 1: {}", agent.2);
        
        
        // ==== Part 2 ==== //
        
        let mut acc2 = 0;
        
        for row in &pipe_map {
            
            let mut loop_layers = 0; // Count how many times the loop is crossed
            let mut on_pipe = false;
            let mut last_seen_pipe = '.'; // Track the type of pipe seenwhen stepping onto the loop
            
            for tile in row.iter().enumerate() {
                if tile.1.1 != 0 {
                    // Found pipe in the loop
                    
                    if tile.1.0 == '|' {
                        // Guaranteed boundary cross
                        loop_layers += 1;
                    } else if tile.1.0 == 'L' || tile.1.0 == 'F' {
                        // Stepping onto the loop
                        on_pipe = true;
                        last_seen_pipe = tile.1.0;
                    }
                    if on_pipe {
                        if last_seen_pipe == 'L' && tile.1.0 == 'J' || last_seen_pipe == 'F' && tile.1.0 == '7' {
                            // Stepping off the loop - NOT a boundary cross
                            on_pipe = false;
                        } else if last_seen_pipe == 'L' && tile.1.0 == '7' || last_seen_pipe == 'F' && tile.1.0 == 'J' {
                            // Stepping off the loop - this is a boundary cross
                            loop_layers += 1;
                            on_pipe = false;
                        }
                        
                    }
                    
                } else {
                    if loop_layers % 2 == 1 {
                        // Inside loop, iterate accumulator
                        acc2 += 1;
                    }
                }
            }
            
        }
        
        println!("Part 2: {}", acc2);
        
    }
    
}
