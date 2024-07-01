pub mod day15 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        fn path_finder(area: &Vec<Vec<u32>>) -> u32 {
            // Each position stores its risk, relative coords of its predecessor in the path (row, col), and the total risk to get there.
            let mut maze = area.iter().map(|row| row.iter().map(|a| (*a as i32, (0,0), 0)).collect::<Vec<_>>()).collect::<Vec<_>>();
            
            let mut agent: (i32, usize, usize); // risk (negative), row, col
            let mut branches = std::collections::BinaryHeap::new();
            branches.push((0i32, 0usize, 0usize));
            
            let directions = [(1,0),(0,1),(-1,0),(0,-1)];
            
            while branches.len() > 0 {
                agent = branches.pop().unwrap();
                
                if (agent.1, agent.2) == (maze.len() - 1, maze[0].len() - 1) {
                   // Found exit
                   return -agent.0 as u32;
                }
                
                for dir in directions {
                    let row = (agent.1 as isize + dir.0) as isize;
                    let col = (agent.2 as isize + dir.1) as isize;
                    
                    // Test if the direction is in bounds
                    if row >= 0 && col >= 0 && row < maze.len() as isize && col < maze.len() as isize {
                        let (row, col) = (row as usize, col as usize);
                        let total_risk = agent.0-maze[row][col].0;
                        // Check if the new position is unexplored or if this path is better
                        if maze[row][col].1 == (0,0) || maze[row][col].2 < total_risk {
                            branches.push((total_risk, row, col));
                            maze[row][col].1 = (-dir.0, -dir.1);
                            maze[row][col].2 = total_risk;
                        }
                    }
                    
                }
                
            }
            
            unreachable!();
        }
        
        let area = lines.iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        println!("Part 1: {}", path_finder(&area));
        
        
        // ==== Part 2 ==== //
        
        let mut area2 = Vec::new();
        
        for i in 0..5 { // row
            
            let mut temp = vec![Vec::new(); area.len()];
            
            for j in 0..5 { // col
                
                for row in 0..area.len() {
                    
                    for col in 0..area[0].len() {
                        
                        temp[row].push((area[row][col]+i+j-1)%9 + 1);
                        
                    }
                    
                }
                
            }
            
            area2 = [area2, temp].concat();
            
        }
        
        println!("Part 2: {}", path_finder(&area2));
        
    }
    
}
