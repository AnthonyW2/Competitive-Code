pub mod day12 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let initial_map = lines.iter()
            .map(
                |s| s.chars()
                .map(|c| (c, false))
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();
        
        // Return matching neighbours
        fn check_neighbours(map: &Vec<Vec<(char, bool)>>, (x,y): (usize, usize), char: char, invert: bool) -> Vec<(usize, usize)> {
            
            let mut output = Vec::new();
            
            if x > 0 && (map[y][x-1].0 == char && !invert || map[y][x-1].0 != char && invert) {
                output.push((x-1, y));
            }
            
            if y > 0 && (map[y-1][x].0 == char && !invert || map[y-1][x].0 != char && invert) {
                output.push((x, y-1));
            }
            
            if x < map[0].len()-1 && (map[y][x+1].0 == char && !invert || map[y][x+1].0 != char && invert) {
                output.push((x+1, y));
            }
            
            if y < map[0].len()-1 && (map[y+1][x].0 == char && !invert || map[y+1][x].0 != char && invert) {
                output.push((x, y+1));
            }
            
            return output;
            
        }
        
        // Traverse a plot from a given starting location, finding area and perimeter
        fn traverse_plot(map: &mut Vec<Vec<(char, bool)>>, start: (usize, usize)) -> (usize, usize) {
            
            let mut to_check = vec![start];
            map[start.1][start.0].1 = true;
            
            let mut area = 0;
            let mut perimeter = 0;
            
            while to_check.len() > 0 {
                
                let current_pos = to_check.pop().unwrap();
                area += 1;
                
                // Find matching neighbours
                let neighbours = check_neighbours(map, current_pos, map[current_pos.1][current_pos.0].0, false);
                perimeter += 4-neighbours.len();
                
                // Find neighbours that haven't been checked
                let mut not_checked = neighbours.into_iter().filter(|(x,y)| !map[*y][*x].1).collect::<Vec<_>>();
                
                // Mark as checked
                for (x,y) in &not_checked {
                    map[*y][*x].1 = true;
                }
                
                // Append neighbours that are yet to be checked
                to_check.append(&mut not_checked);
                
            }
            
            return (area, perimeter);
            
        }
        
        let mut map1 = initial_map.clone();
        
        let mut cost1 = 0;
        
        for y in 0..map1.len() {
            for x in 0..map1[0].len() {
                if !map1[y][x].1 {
                    let result = traverse_plot(&mut map1, (x,y));
                    cost1 += result.0 * result.1;
                }
            }
        }
        
        println!("Part 1: {}", cost1);
        
        
        // ==== Part 2 ==== //
        
        // Traverse a plot from a given starting location, finding the area & number of sides
        fn traverse_plot2(map: &mut Vec<Vec<(char, bool)>>, start: (usize, usize)) -> (usize, usize) {
            
            let mut outline_check_map = vec![vec![('.', false); map[0].len()]; map.len()];
            
            let mut to_check = vec![start];
            map[start.1][start.0].1 = true;
            
            let mut area = 0;
            
            let mut spaces_to_check = Vec::new();
            
            while to_check.len() > 0 {
                
                let current_pos = to_check.pop().unwrap();
                area += 1;
                
                outline_check_map[current_pos.1][current_pos.0].0 = '#';
                
                // Find matching neighbours
                let neighbours = check_neighbours(map, current_pos, map[current_pos.1][current_pos.0].0, false);
                
                // Add spaces that aren't within the region to a list of candidates to start counting sides from
                if neighbours.len() < 4 {
                    let spaces = check_neighbours(map, current_pos, map[current_pos.1][current_pos.0].0, true);
                    spaces_to_check.push(spaces[0]);
                }
                
                // Find neighbours that haven't been checked
                let mut not_checked = neighbours.into_iter().filter(|(x,y)| !map[*y][*x].1).collect::<Vec<_>>();
                
                // Mark as checked
                for (x,y) in &not_checked {
                    map[*y][*x].1 = true;
                }
                
                // Append neighbours that are yet to be checked
                to_check.append(&mut not_checked);
                
            }
            
            let mut sides = 0;
            
            // Count sides
            for space in spaces_to_check {
                if !outline_check_map[space.1][space.0].1 {
                    // Start traversing the edge from an empty space
                    sides += traverse_outline(&mut outline_check_map, space, '#');
                }
            }
            
            if sides == 0 {
                sides = 4;
            }
            
            return (area, sides);
            
        }
        
        fn turn_right(vel: (isize, isize)) -> (isize, isize) {
            return (-vel.1, vel.0);
        }
        fn turn_left(vel: (isize, isize)) -> (isize, isize) {
            return (vel.1, -vel.0);
        }
        
        // Traverse the outline of a plot, returning the number of corners
        // This function moves an agent around the edge of the plot, keeping the plot to its right
        fn traverse_outline(map: &mut Vec<Vec<(char, bool)>>, start: (usize, usize), region_char: char) -> usize {
            
            let mut corners = 0;
            
            let start_pos = start;
            let mut pos = start;
            
            let rel_neighbours = check_neighbours(map, pos, region_char, false).iter()
                .map(|(x,y)| ((*x as isize)-(pos.0 as isize), (*y as isize)-(pos.1 as isize)))
                .collect::<Vec<_>>();
            if rel_neighbours.len() == 4 {
                map[pos.1][pos.0].1 = true;
                return 4;
            }
            let start_vel = turn_left(rel_neighbours[0]);
            let mut vel = start_vel;
            
            loop {
                
                // If already checked, the loop is over
                if map[pos.1][pos.0].1 && pos == start_pos && vel == start_vel {
                    return corners;
                }
                
                // Mark this position as visited
                map[pos.1][pos.0].1 = true;
                
                let neighbours = check_neighbours(map, pos, region_char, false);
                let rel_neighbours = neighbours.iter()
                    .map(|(x,y)| ((*x as isize)-(pos.0 as isize), (*y as isize)-(pos.1 as isize)))
                    .collect::<Vec<_>>();
                
                match rel_neighbours.len() {
                    0 => {
                        vel = turn_right(vel);
                        corners += 1;
                        pos = ((pos.0 as isize + vel.0) as usize, (pos.1 as isize + vel.1) as usize);
                    },
                    1 => {
                        if rel_neighbours[0] != turn_right(vel) {
                            vel = turn_right(vel);
                            corners += 1;
                        }
                        pos = ((pos.0 as isize + vel.0) as usize, (pos.1 as isize + vel.1) as usize);
                    },
                    2 => {
                        if !rel_neighbours.contains(&turn_right(vel)) {
                            vel = turn_right(vel);
                            corners += 1;
                            pos = ((pos.0 as isize + vel.0) as usize, (pos.1 as isize + vel.1) as usize);
                        } else if rel_neighbours.contains(&vel) {
                            vel = turn_left(vel);
                            corners += 1;
                        } else {
                            pos = ((pos.0 as isize + vel.0) as usize, (pos.1 as isize + vel.1) as usize);
                        }
                    },
                    3 => {
                        if !rel_neighbours.contains(&turn_right(vel)) {
                            vel = turn_right(vel);
                            corners += 1;
                        } else if rel_neighbours.contains(&vel) {
                            vel = turn_left(vel);
                            corners += 1;
                        } else {
                            pos = ((pos.0 as isize + vel.0) as usize, (pos.1 as isize + vel.1) as usize);
                        }
                    },
                    4 => {
                        return 4;
                    },
                    _ => { }
                }
                
            }
            
        }
        
        // Add a border around the outside of the map
        let mut map2 = initial_map.clone()
            .into_iter()
            .map(
                |mut v| {
                    v.insert(0, ('.', false));
                    v.push(('.', false));
                    return v;
                }
            )
            .collect::<Vec<_>>();
        map2.insert(0, vec![('.', false); map2[0].len()]);
        map2.push(vec![('.', false); map2[0].len()]);
        
        let mut cost2 = 0;
        
        for y in 1..map2.len()-1 {
            for x in 1..map2[0].len()-1 {
                if !map2[y][x].1 {
                    let result = traverse_plot2(&mut map2, (x,y));
                    cost2 += result.0 * result.1;
                }
            }
        }
        
        // 872548 too high
        
        println!("Part 2: {}", cost2);
        
    }
    
}
