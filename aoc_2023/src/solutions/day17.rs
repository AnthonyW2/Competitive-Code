pub mod day17 {
    
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        fn path_finder(area: &Vec<Vec<u32>>) -> u32 {
            
            // Solution:
            // Rather than using the 2D space as the graph, generate a graph dynamically with dimensions: pos, direction, straight-line-distance
            
            // Key: ((row, col), (dir.row, dir.col), straight line len)
            // Value: (total risk, (pre.row, pre.col))
            let mut nodes: HashMap<((usize, usize), (isize, isize), u32), (i32, (isize, isize))> = HashMap::new();
            
            let mut agent: (i32, (usize, usize), (isize, isize), u32); // risk (negative), (row, col), (d.row, d.col), straight line length
            let mut branches = BinaryHeap::new();
            branches.push((0i32, (0usize, 0usize), (0isize, 0isize), 0u32));
            
            let directions = [(1,0),(0,1),(-1,0),(0,-1)];
            
            while branches.len() > 0 {
                agent = branches.pop().unwrap();
                
                if (agent.1.0, agent.1.1) == (area.len() - 1, area[0].len() - 1) {
                   // Found exit
                   return -agent.0 as u32;
                }
                
                for dir in directions {
                    let mut new_straight_line_len = 1;
                    if dir == agent.2 {
                        new_straight_line_len = agent.3 + 1;
                    }
                    if new_straight_line_len > 3 || dir == (-agent.2.0, -agent.2.1) {
                        continue;
                    }
                    
                    let row = (agent.1.0 as isize + dir.0) as isize;
                    let col = (agent.1.1 as isize + dir.1) as isize;
                    
                    // Test if the direction is in bounds
                    if row >= 0 && col >= 0 && row < area.len() as isize && col < area[0].len() as isize {
                        let (row, col) = (row as usize, col as usize);
                        let total_risk = agent.0 - (area[row][col] as i32);
                        // Check if the new position is unexplored or if this path is better
                        let next_node = nodes.get(&((row, col), dir, new_straight_line_len));
                        if next_node.is_none() || next_node.unwrap().0 < total_risk {
                            branches.push((total_risk, (row, col), dir, new_straight_line_len));
                            nodes.insert(((row, col), dir, new_straight_line_len), (total_risk, (-dir.0, -dir.1)));
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
        
        fn path_finder2(area: &Vec<Vec<u32>>) -> u32 {
            // Key: ((row, col), (dir.row, dir.col), straight line len)
            // Value: (total risk, (pre.row, pre.col))
            let mut nodes: HashMap<((usize, usize), (isize, isize), u32), (i32, (isize, isize))> = HashMap::new();
            
            let mut agent: (i32, (usize, usize), (isize, isize), u32); // risk (negative), (row, col), (d.row, d.col), straight line length
            let mut branches = BinaryHeap::new();
            branches.push((0i32, (0usize, 0usize), (0isize, 0isize), 0u32));
            
            let directions = [(1,0),(0,1),(-1,0),(0,-1)];
            
            while branches.len() > 0 {
                agent = branches.pop().unwrap();
                
                if (agent.1.0, agent.1.1) == (area.len() - 1, area[0].len() - 1) && agent.3 >= 4 {
                   // Found exit
                   return -agent.0 as u32;
                }
                
                for dir in directions {
                    let new_straight_line_len;
                    if dir == agent.2 {
                        new_straight_line_len = agent.3 + 1;
                    } else {
                        if agent.3 < 4 && agent.3 != 0 {
                            continue;
                        }
                        new_straight_line_len = 1;
                    }
                    if new_straight_line_len > 10 || dir == (-agent.2.0, -agent.2.1) {
                        continue;
                    }
                    
                    let row = (agent.1.0 as isize + dir.0) as isize;
                    let col = (agent.1.1 as isize + dir.1) as isize;
                    
                    // Test if the direction is in bounds
                    if row >= 0 && col >= 0 && row < area.len() as isize && col < area[0].len() as isize {
                        let (row, col) = (row as usize, col as usize);
                        let total_risk = agent.0 - (area[row][col] as i32);
                        // Check if the new position is unexplored or if this path is better
                        let next_node = nodes.get(&((row, col), dir, new_straight_line_len));
                        if next_node.is_none() || next_node.unwrap().0 < total_risk {
                            branches.push((total_risk, (row, col), dir, new_straight_line_len));
                            nodes.insert(((row, col), dir, new_straight_line_len), (total_risk, (-dir.0, -dir.1)));
                        }
                    }
                    
                }
                
            }
            
            unreachable!();
        }
        
        println!("Part 2: {}", path_finder2(&area));
        
    }
    
    
    pub fn attempt1(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        fn path_finder(area: &Vec<Vec<u32>>) -> u32 {
            // Each position stores its risk, relative coords of its predecessor in the path (row, col), the total risk to get there, and when it was first seen.
            let mut maze = area.iter().map(|row| row.iter().map(|a| (*a as i32, (0isize,0isize), 0, 0)).collect::<Vec<_>>()).collect::<Vec<_>>();
            
            let mut agent: (i32, i32, (usize, usize), (isize, isize), u32); // risk (negative), total steps, (row, col), (d.row, d.col), straight line length
            let mut branches = std::collections::BinaryHeap::new();
            branches.push((0i32, 0i32, (0usize, 0usize), (0isize,0isize), 0u32));
            
            let directions = [(1,0),(0,1),(-1,0),(0,-1)];
            
            let mut step = 0;
            
            while !branches.is_empty() {
                agent = branches.pop().unwrap();
                
                step += 1;
                
                // Prune the heap
                if branches.len() > 20 {
                    branches.retain(|a| a.1+2 > agent.1)
                }
                
                if (agent.2.0, agent.2.1) == (maze.len() - 1, maze[0].len() - 1) {
                    // Found exit
                    
                    // DEBUG
                    let mut next = (area.len()-1, area[0].len()-1);
                    while next != (0,0) {
                        maze[next.0][next.1].0 = 0;
                        next = ((next.0 as isize + maze[next.0][next.1].1.0) as usize, (next.1 as isize + maze[next.0][next.1].1.1) as usize);
                    }
                    for line in &maze {
                        println!("{}", line.iter().map(|t| t.0.to_string()).collect::<String>());
                    }
                    //println!("{:?}", branches);
                    println!("{step} steps");
                    
                    return -agent.0 as u32;
                }
                
                for dir in directions {
                    let mut new_straight_line_len = 1;
                    if dir == agent.3 {
                        new_straight_line_len = agent.4 + 1;
                    }
                    if new_straight_line_len > 3 || dir == (-agent.3.0, -agent.3.1) {
                        //println!("Straight line too long {:?}", agent);
                        continue;
                    }
                    //println!("Straight line OK {:?}", agent);
                    
                    let row = (agent.2.0 as isize + dir.0) as isize;
                    let col = (agent.2.1 as isize + dir.1) as isize;
                    
                    // Test if the direction is in bounds
                    if row >= 0 && col >= 0 && row < maze.len() as isize && col < maze.len() as isize {
                        let (row, col) = (row as usize, col as usize);
                        let total_risk = agent.0-maze[row][col].0;
                        // Check if the new position is unexplored or if this path is better
                        if maze[row][col].1 == (0,0) || maze[row][col].2 < total_risk || maze[row][col].3 > agent.1 {
                            branches.push((total_risk, agent.1+1, (row, col), dir, new_straight_line_len));
                            maze[row][col].1 = (-dir.0, -dir.1);
                            maze[row][col].2 = total_risk;
                            if maze[row][col].3 == 0 {
                                maze[row][col].3 = (agent.1)+1;
                            }
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
        
        println!("Part 2: {}", "_");
        
    }
    
    
}
