pub mod day16 {
    
    use std::collections::VecDeque;
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        // Use BFS to follow the laser paths
        fn count_energized(mirror_map: &mut Vec<Vec<(char, [bool; 4])>>, starting_position: (isize, isize, (isize, isize), u32)) -> u32 {
            let direction_indices = HashMap::from([
                ((1,0), 0),
                ((-1,0), 1),
                ((0,1), 2),
                ((0,-1), 3)
            ]);
            
            let mut agent: (isize, isize, (isize, isize), u32); // row, col, (d.row, d.col), distance
            let mut branches: VecDeque<(isize, isize, (isize, isize), u32)> = VecDeque::from([starting_position]);
            
            let mut result = 0;
            
            while branches.len() > 0 {
                agent = branches.pop_front().unwrap();
                
                let current_tile = mirror_map[agent.0 as usize][agent.1 as usize];
                
                // Increment the result if this tile has not been energized before
                if !current_tile.1.contains(&true) {
                    result += 1;
                }
                
                // Only continue the beam if no beam travelling in the same direction has been here before
                if !current_tile.1[*direction_indices.get(&agent.2).unwrap()] {
                    mirror_map[agent.0 as usize][agent.1 as usize].1[*direction_indices.get(&agent.2).unwrap()] = true;
                    
                    let mut new_dir: Option<(isize, isize)> = None;
                    
                    match current_tile.0 {
                        '/' => {
                            new_dir = Some((-agent.2.1, -agent.2.0));
                        },
                        '\\' => {
                            new_dir = Some((agent.2.1, agent.2.0));
                        },
                        '|' => {
                            if agent.2.0 == 0 {
                                // Split up & down
                                let new_pos_1 = (agent.0 + 1, agent.1);
                                if new_pos_1.0 >= 0 && (new_pos_1.0 as usize) < mirror_map[0].len() && new_pos_1.1 >= 0 && (new_pos_1.1 as usize) < mirror_map.len() {
                                    branches.push_back((new_pos_1.0, new_pos_1.1, (1,0), agent.3+1));
                                }
                                let new_pos_2 = (agent.0 - 1, agent.1);
                                if new_pos_2.0 >= 0 && (new_pos_2.0 as usize) < mirror_map[0].len() && new_pos_2.1 >= 0 && (new_pos_2.1 as usize) < mirror_map.len() {
                                    branches.push_back((new_pos_2.0, new_pos_2.1, (-1,0), agent.3+1));
                                }
                                
                            } else {
                                // Go straight through
                                new_dir = Some(agent.2);
                            }
                        },
                        '-' => {
                            if agent.2.1 == 0 {
                                // Split left & right
                                let new_pos_1 = (agent.0, agent.1 + 1);
                                if new_pos_1.0 >= 0 && (new_pos_1.0 as usize) < mirror_map[0].len() && new_pos_1.1 >= 0 && (new_pos_1.1 as usize) < mirror_map.len() {
                                    branches.push_back((new_pos_1.0, new_pos_1.1, (0,1), agent.3+1));
                                }
                                let new_pos_2 = (agent.0, agent.1 - 1);
                                if new_pos_2.0 >= 0 && (new_pos_2.0 as usize) < mirror_map[0].len() && new_pos_2.1 >= 0 && (new_pos_2.1 as usize) < mirror_map.len() {
                                    branches.push_back((new_pos_2.0, new_pos_2.1, (0,-1), agent.3+1));
                                }
                                
                            } else {
                                // Go straight through
                                new_dir = Some(agent.2);
                            }
                        },
                        _ => {
                            new_dir = Some(agent.2);
                        },
                    }
                    
                    if new_dir.is_some() {
                        let new_pos = (agent.0 + new_dir.unwrap().0, agent.1 + new_dir.unwrap().1);
                        if new_pos.0 >= 0 && (new_pos.0 as usize) < mirror_map[0].len() && new_pos.1 >= 0 && (new_pos.1 as usize) < mirror_map.len() {
                            branches.push_back((new_pos.0, new_pos.1, new_dir.unwrap(), agent.3));
                        }
                    }
                }
                
            }
            
            return result;
        }
        
        let mirror_map = (&lines).iter().map(|s| s.chars().map(|c| (c, [false, false, false, false])).collect::<Vec<_>>()).collect::<Vec<_>>();
        
        println!("Part 1: {}", count_energized(&mut mirror_map.to_owned(), (0, 0, (0,1), 0)));
        
        
        // ==== Part 2 ==== //
        
        let mut max_energised = 0;
        
        for r in 0..mirror_map.len() {
            let new_result = count_energized(&mut mirror_map.to_owned(), (r as isize, 0, (0,1), 0));
            max_energised = max_energised.max(new_result);
            
            let new_result = count_energized(&mut mirror_map.to_owned(), (r as isize, (mirror_map.len()-1) as isize, (0,-1), 0));
            max_energised = max_energised.max(new_result);
        }
        
        for c in 0..mirror_map[0].len() {
            let new_result = count_energized(&mut mirror_map.to_owned(), (0, c as isize, (1,0), 0));
            max_energised = max_energised.max(new_result);
            
            let new_result = count_energized(&mut mirror_map.to_owned(), ((mirror_map[0].len()-1) as isize, c as isize, (-1,0), 0));
            max_energised = max_energised.max(new_result);
        }
        
        println!("Part 2: {}", max_energised);
        
    }
    
}
