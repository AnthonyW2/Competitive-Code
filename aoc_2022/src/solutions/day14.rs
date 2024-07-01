pub mod day14 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let sand_source = (500,0); // (x,y)
        
        // (col = x, row = y)
        let walls = lines.iter()
            .map(|s| s.split(" -> ")
                .map(|c| {
                    let mut ints = c.split(',').map(|n| n.parse::<usize>().unwrap());
                    (ints.next().unwrap(), ints.next().unwrap())
                })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        // Find the boundaries of the simulation space
        let mut min_x = sand_source.0;
        let mut max_x = sand_source.0;
        let mut max_y = sand_source.1;
        for wall in &walls {
            for coord in wall {
                min_x = min_x.min(coord.0);
                max_x = max_x.max(coord.0);
                max_y = max_y.max(coord.1);
            }
        }
        min_x = min_x.min(sand_source.0 - max_y - 1); // Expand the horizontal boundaries for part 2
        max_x = max_x.max(sand_source.0 + max_y + 1);
        
        // Add the walls to the simulation space
        let mut original_sim_area = vec![vec!['.'; max_x - min_x + 3]; max_y + 2];
        for wall in &walls {
            for c in 0..(wall.len()-1) {
                
                // Draw a line from wall[c] to wall[c+1]
                if wall[c].0 == wall[c+1].0 {
                    let min = wall[c].1.min(wall[c+1].1);
                    let max = wall[c].1.max(wall[c+1].1);
                    for y in min..=max {
                        original_sim_area[y][wall[c].0-min_x+1] = '#';
                    }
                } else {
                    let min = wall[c].0.min(wall[c+1].0);
                    let max = wall[c].0.max(wall[c+1].0);
                    for x in min..=max {
                        original_sim_area[wall[c].1][x-min_x+1] = '#';
                    }
                }
                
            }
        }
        
        let mut sim_area_1 = original_sim_area.clone();
        
        // Simulate 1 piece of sand falling (return true if the sand came to rest)
        fn sim_sand(sim_area: &mut Vec<Vec<char>>, sand_source: (usize, usize)) -> bool {
            
            // (x,y)
            let mut sand = sand_source.to_owned();
            
            while sand.1 < sim_area.len()-1 && sim_area[sand.1][sand.0] == '.' {
                
                if sim_area[sand.1+1][sand.0] == '.' {
                    sand.1 += 1;
                } else if sim_area[sand.1+1][sand.0-1] == '.' {
                    sand.1 += 1;
                    sand.0 -= 1;
                } else if sim_area[sand.1+1][sand.0+1] == '.' {
                    sand.1 += 1;
                    sand.0 += 1;
                } else {
                    sim_area[sand.1][sand.0] = 'o';
                    return true;
                }
                
            }
            
            return false;
        }
        
        let mut result1 = 0;
        while sim_sand(&mut sim_area_1, (sand_source.0-min_x+1, sand_source.1)) {
            result1 += 1;
        }
        
        println!("Part 1: {}", result1);
        
        
        // ==== Part 2 ==== //
        
        let mut sim_area_2 = original_sim_area.clone();
        sim_area_2.push(vec!['#'; max_x - min_x + 3]);
        
        let mut result2 = 0;
        while sim_sand(&mut sim_area_2, (sand_source.0-min_x+1, sand_source.1)) {
            result2 += 1;
        }
        
        println!("Part 2: {}", result2);
        
    }
    
}
