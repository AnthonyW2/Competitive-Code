pub mod day12 {
    
    use std::collections::VecDeque;
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        let mut start = (0,0); // row, col
        let mut end = (0,0); // row, col
        
        let mut heightmap = lines.iter()
            .enumerate()
            .map(|(row, s)| s.chars().enumerate().map(|(col, c)| {
                if c == 'S' {
                    start = (row,col);
                    return (0, false);
                } else if c == 'E' {
                    end = (row,col);
                    return (27, false);
                }
                return (c as i32 - 96, false);
            }).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        
        // ==== Part 1 ==== //
        
        let mut agent: (usize, usize, u32); // row, col
        let mut branches: VecDeque<(usize, usize, u32)> = VecDeque::from([(start.0,start.1,0)]);
        
        let mut steps = 0;
        
        while branches.len() > 0 {
            agent = branches.pop_front().unwrap();
            
            let cur_height = heightmap[agent.0][agent.1].0;
            
            if cur_height == 27 {
                // Found exit
                steps = agent.2;
                break;
            }
            
            if agent.0+1 < heightmap.len() && heightmap[agent.0+1][agent.1].0 - cur_height <= 1 && !heightmap[agent.0+1][agent.1].1 {
                // Can move down
                branches.push_back((agent.0+1, agent.1, agent.2+1));
                heightmap[agent.0+1][agent.1].1 = true; // mark where we've been
            }
            if agent.1+1 < heightmap[0].len() && heightmap[agent.0][agent.1+1].0 - cur_height <= 1 && !heightmap[agent.0][agent.1+1].1 {
                // Can move right
                branches.push_back((agent.0, agent.1+1, agent.2+1));
                heightmap[agent.0][agent.1+1].1 = true;
            }
            if agent.0 > 0 && heightmap[agent.0-1][agent.1].0 - cur_height <= 1 && !heightmap[agent.0-1][agent.1].1 {
                // Can move up
                branches.push_back((agent.0-1, agent.1, agent.2+1));
                heightmap[agent.0-1][agent.1].1 = true;
            }
            if agent.1 > 0 && heightmap[agent.0][agent.1-1].0 - cur_height <= 1 && !heightmap[agent.0][agent.1-1].1 {
                // Can move left
                branches.push_back((agent.0, agent.1-1, agent.2+1));
                heightmap[agent.0][agent.1-1].1 = true;
            }
        }
        
        println!("Part 1: {}", steps);
        
        
        // ==== Part 2 ==== //
        
        let mut heightmap = lines.iter()
            .map(|s| s.chars().map(|c| {
                if c == 'S' {
                    return (0, false);
                } else if c == 'E' {
                    return (27, false);
                }
                return (c as i32 - 96, false);
            }).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let mut agent: (usize, usize, u32); // row, col
        let mut branches: VecDeque<(usize, usize, u32)> = VecDeque::from([(end.0,end.1,0)]);
        
        let mut steps2 = 0;
        
        while branches.len() > 0 {
            agent = branches.pop_front().unwrap();
            
            let cur_height = heightmap[agent.0][agent.1].0;
            
            if cur_height == 1 {
                // Found exit
                steps2 = agent.2;
                break;
            }
            
            if agent.0+1 < heightmap.len() && cur_height - heightmap[agent.0+1][agent.1].0 <= 1 && !heightmap[agent.0+1][agent.1].1 {
                // Can move down
                branches.push_back((agent.0+1, agent.1, agent.2+1));
                heightmap[agent.0+1][agent.1].1 = true;
            }
            if agent.1+1 < heightmap[0].len() && cur_height - heightmap[agent.0][agent.1+1].0 <= 1 && !heightmap[agent.0][agent.1+1].1 {
                // Can move right
                branches.push_back((agent.0, agent.1+1, agent.2+1));
                heightmap[agent.0][agent.1+1].1 = true;
            }
            if agent.0 > 0 && cur_height - heightmap[agent.0-1][agent.1].0 <= 1 && !heightmap[agent.0-1][agent.1].1 {
                // Can move up
                branches.push_back((agent.0-1, agent.1, agent.2+1));
                heightmap[agent.0-1][agent.1].1 = true;
            }
            if agent.1 > 0 && cur_height - heightmap[agent.0][agent.1-1].0 <= 1 && !heightmap[agent.0][agent.1-1].1 {
                // Can move left
                branches.push_back((agent.0, agent.1-1, agent.2+1));
                heightmap[agent.0][agent.1-1].1 = true;
            }
        }
        
        println!("Part 2: {}", steps2);
        
    }
    
}
