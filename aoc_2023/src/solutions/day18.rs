pub mod day18 {
    
    use std::collections::HashMap;
    use std::collections::BTreeMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let dir_list = [(0,1), (1,0), (0,-1), (-1,0)]; // row, col
        let corner_map: HashMap<(i64, i64), char> = HashMap::from([
            ((1,1), 'F'),
            ((-1,1), 'L'),
            ((1,-1), '7'),
            ((-1,-1), 'J'),
        ]);
        
        // Parse one line of instructions for part 1
        fn parse_instructions(line: &String) -> (usize, usize, String) {
            let dir_map: HashMap<char, usize> = HashMap::from([
                ('R', 0),
                ('D', 1),
                ('L', 2),
                ('U', 3),
            ]);
            
            let mut split = line.split(" ");
            let dir = *dir_map.get(&split.next().unwrap().chars().next().unwrap()).unwrap();
            let dist = split.next().unwrap().parse::<usize>().unwrap();
            let color = split.next().unwrap().replace("(","").replace(")","");
            return (dir, dist, color);
        }
        
        let mut highest_point = 0; // row (min)
        let mut lowest_point = 0;  // row (max)
        let mut leftmost_point = 0;  // col (min)
        let mut rightmost_point = 0; // col (max)
        
        let mut digger = (0,0); // (row, col)
        
        // Move the digger around according to the instructions to find the bounds of the shape
        for line in &lines {
            let (dir, dist, _color) = parse_instructions(line);
            
            digger.0 += (dir_list[dir].0) * dist as i64;
            digger.1 += (dir_list[dir].1) * dist as i64;
            
            highest_point = highest_point.min(digger.0);
            lowest_point = lowest_point.max(digger.0);
            leftmost_point = leftmost_point.min(digger.1);
            rightmost_point = rightmost_point.max(digger.1);
        }
        
        // (color, direction, edge type)
        let mut terrain: Vec<Vec<(String, usize, char)>> = vec![vec![(String::new(), 0, '.'); (rightmost_point-leftmost_point+1) as usize]; (lowest_point-highest_point+1) as usize];
        
        // row, col
        let mut digger = (-highest_point as usize, -leftmost_point as usize);
        
        // Draw the trench in the terrain
        for (li, line) in (&lines).iter().enumerate() {
            let (dir, dist, color) = parse_instructions(line);
            
            for _ in 0..dist {
                digger.0 = (digger.0 as i64 + dir_list[dir].0) as usize;
                digger.1 = (digger.1 as i64 + dir_list[dir].1) as usize;
                
                let dir_type = if dir_list[dir].1 == 0 {'|'} else {'-'};
                terrain[digger.0][digger.1] = (color.clone(), dir, dir_type);
            }
            
            let next_dir = parse_instructions(&lines[(li+1)%lines.len()]).0;
            let dir_sum = (dir_list[next_dir].0 - dir_list[dir].0, dir_list[next_dir].1 - dir_list[dir].1);
            terrain[digger.0][digger.1].2 = *corner_map.get(&dir_sum).unwrap();
            
        }
        if digger != (-highest_point as usize, -leftmost_point as usize) {
            panic!("Starting position is not equivalent to ending position");
        }
        
        // Print the terrain map
        //for row in &terrain {
        //    println!("{}", row.iter().map(|t| t.2).collect::<String>());
        //}
        //println!();
        
        // Given the trench map from part 1, return the total area it covers & encloses.
        // Uses the same area-finding algorithm as day 10.
        fn dug_area(terrain: &Vec<Vec<(String, usize, char)>>) -> u32 {
            let mut result = 0;
            
            for row in terrain {
                
                let mut loop_layers = 0; // Count how many times the loop is crossed
                let mut in_trench = false;
                let mut last_seen_corner = '.'; // Track the type of pipe seenwhen stepping onto the loop
                
                for tile in row.iter().enumerate() {
                    if tile.1.2 != '.' {
                        // Found part of the loop
                        
                        if tile.1.2 == '|' {
                            // Guaranteed boundary cross
                            loop_layers += 1;
                        } else if tile.1.2 == 'L' || tile.1.2 == 'F' {
                            // Stepping onto the loop
                            in_trench = true;
                            last_seen_corner = tile.1.2;
                        }
                        if in_trench {
                            if last_seen_corner == 'L' && tile.1.2 == 'J' || last_seen_corner == 'F' && tile.1.2 == '7' {
                                // Stepping off the loop - NOT a boundary cross
                                in_trench = false;
                            } else if last_seen_corner == 'L' && tile.1.2 == '7' || last_seen_corner == 'F' && tile.1.2 == 'J' {
                                // Stepping off the loop - this is a boundary cross
                                loop_layers += 1;
                                in_trench = false;
                            }
                        }
                        
                        result += 1;
                        
                    } else {
                        if loop_layers % 2 == 1 {
                            // Inside loop, iterate accumulator
                            result += 1;
                        }
                    }
                }
                
            }
            
            return result;
        }
        
        println!("Part 1: {}", dug_area(&terrain));
        
        
        // ==== Part 2 ==== //
        
        // Parse one line of instructions for part 2
        fn get_instructions2(line: &String) -> (usize, i64) {
            let hex = line.split(" ").nth(2).unwrap().replace("(#","").replace(")","");
            let dist = i64::from_str_radix(&hex[0..5], 16).unwrap();
            let dir = hex[5..=5].parse::<usize>().unwrap();
            return (dir, dist);
        }
        
        // Store the corners & the vertical lines to the left & right of every corner
        // [row, [col, corner_type]]
        let mut trench: BTreeMap<i64, BTreeMap<i64, char>> = BTreeMap::new();
        
        // min_row, max_row, col
        let mut vertical_lines: Vec<(i64, i64, i64)> = Vec::new();
        
        let mut digger = (0,0);
        
        for (li, line) in (&lines).iter().enumerate() {
            let (dir, dist) = get_instructions2(line);
            //let (dir, dist, _color) = get_instructions(line); let dist = dist as i64;
            
            // Get corner type
            let next_dir = get_instructions2(&lines[(li+1)%lines.len()]).0;
            //let next_dir = get_instructions(&lines[(li+1)%lines.len()]).0;
            let next_corner = *corner_map.get(&(dir_list[next_dir].0 - dir_list[dir].0, dir_list[next_dir].1 - dir_list[dir].1)).unwrap();
            
            let start = digger.clone();
            
            digger.0 += (dir_list[dir].0) * dist;
            digger.1 += (dir_list[dir].1) * dist;
            
            // Add corners to the trench map
            if !trench.contains_key(&digger.0) {
                trench.insert(digger.0, BTreeMap::new());
            }
            trench.entry(digger.0).and_modify(|m| {
                m.insert(digger.1, next_corner);
            });
            
            if dir_list[dir].0 != 0 {
                // Add vertical line
                vertical_lines.push((digger.0.min(start.0), digger.0.max(start.0), digger.1));
            }
            
        }
        
        // Iterate through rows in the trench map
        let rows = trench.iter().map(|t| *t.0).collect::<Vec<_>>();
        for row in rows {
            // Iterate through all vertical lines
            for line in &vertical_lines {
                // Add a '|' in line with every entry in the trench map
                if line.0 < row && row < line.1 {
                    trench.entry(row).and_modify(|m| {
                        m.insert(line.2, '|');
                    });
                }
            }
        }
        
        // Given the corner map from part 2, return the total area it covers & encloses.
        fn dug_area2(trench: &BTreeMap<i64, BTreeMap<i64, char>>) -> i64 {
            let mut result = 0;
            
            let mut last_seen_row_present = false;
            let mut last_seen_row = 0;
            
            let mut area_inside_row = 0; // Area within the trench (including left & right boundaries) to be multiplied by the vertical distance
            
            for row in trench.iter() {
                //println!("{}: {:#?}", row.0, row.1);
                
                // Add area to result
                if last_seen_row_present {
                    result += area_inside_row * (row.0 - last_seen_row - 1);
                }
                area_inside_row = 0;
                last_seen_row = *row.0;
                
                let mut area_along_row = 0; // All dug out area (including the trench) in this row
                
                // Count how many times the loop is crossed
                // An odd number means that this position is inside the shape
                let mut loop_layers = 0;
                
                let mut in_trench = false;
                let mut last_seen_corner = '.'; // Track the type of corner seen when stepping onto the loop
                
                let mut last_seen_col = 0;
                
                for col in row.1.iter() {
                    
                    if in_trench || loop_layers%2 == 1 {
                        area_along_row += col.0 - last_seen_col;
                    }
                    if loop_layers%2 == 1 {
                        area_inside_row += col.0 - last_seen_col;
                    }
                    last_seen_col = *col.0;
                    
                    if *col.1 == '|' {
                        // Guaranteed boundary cross
                        loop_layers += 1;
                        
                        if loop_layers%2 == 1 {
                            area_along_row += 1;
                            area_inside_row += 1;
                        }
                        
                    } else if *col.1 == 'L' {
                        // Stepping into the trench
                        in_trench = true;
                        last_seen_corner = *col.1;
                        
                        if loop_layers%2 == 0 {
                            area_along_row += 1;
                        }
                        
                    } else if *col.1 == 'F' {
                        // Stepping into the trench
                        in_trench = true;
                        last_seen_corner = *col.1;
                        
                        loop_layers += 1;
                        
                        if loop_layers%2 == 1 {
                            area_along_row += 1;
                            area_inside_row += 1;
                        }
                        
                    }
                    
                    if in_trench {
                        if last_seen_corner == 'F' && *col.1 == '7' {
                            // Stepping off the loop - NOT a boundary cross
                            in_trench = false;
                            loop_layers += 1;
                        
                            if loop_layers%2 == 1 {
                                area_inside_row += 1;
                            }
                            
                        } else if last_seen_corner == 'L' && *col.1 == 'J' {
                            // Stepping off the loop - NOT a boundary cross
                            in_trench = false;
                            
                        } else if last_seen_corner == 'F' && *col.1 == 'J' {
                            // Stepping off the loop - this is a boundary cross
                            in_trench = false;
                            
                        } else if last_seen_corner == 'L' && *col.1 == '7' {
                            // Stepping off the loop - this is a boundary cross
                            in_trench = false;
                            loop_layers += 1;
                        
                            if loop_layers%2 == 1 {
                                area_inside_row += 1;
                            }
                            
                        }
                    }
                    
                }
                
                result += area_along_row;
                
                last_seen_row_present = true;
            }
            
            return result;
        }
        
        println!("Part 2: {}", dug_area2(&trench));
        
    }
    
}
