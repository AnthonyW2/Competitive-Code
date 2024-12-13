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
        fn check_neighbours(map: &Vec<Vec<(char, bool)>>, (x,y): (usize, usize)) -> Vec<(usize, usize)> {
            
            let current = map[y][x].0;
            
            let mut output = Vec::new();
            
            if x > 0 && map[y][x-1].0 == current {
                output.push((x-1, y));
            }
            
            if y > 0 && map[y-1][x].0 == current {
                output.push((x, y-1));
            }
            
            if x < map[0].len()-1 && map[y][x+1].0 == current {
                output.push((x+1, y));
            }
            
            if y < map[0].len()-1 && map[y+1][x].0 == current {
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
                let neighbours = check_neighbours(map, current_pos);
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
            
            let mut to_check = vec![start];
            map[start.1][start.0].1 = true;
            
            let mut area = 0;
            let mut sides = 0;
            
            while to_check.len() > 0 {
                
                let current_pos = to_check.pop().unwrap();
                area += 1;
                
                // Find matching neighbours
                let neighbours = check_neighbours(map, current_pos);
                
                // Find neighbours that haven't been checked
                let mut not_checked = neighbours.into_iter().filter(|(x,y)| !map[*y][*x].1).collect::<Vec<_>>();
                
                // Mark as checked
                for (x,y) in &not_checked {
                    map[*y][*x].1 = true;
                }
                
                // Append neighbours that are yet to be checked
                to_check.append(&mut not_checked);
                
            }
            
            return (area, sides);
            
        }
        
        let mut map2 = initial_map.clone();
        
        let mut cost2 = 0;
        
        for y in 0..map2.len() {
            for x in 0..map2[0].len() {
                if !map2[y][x].1 {
                    let result = traverse_plot2(&mut map2, (x,y));
                    cost2 += result.0 * result.1;
                }
            }
        }
        
        println!("Part 2: {}", cost2);
        
    }
    
}
