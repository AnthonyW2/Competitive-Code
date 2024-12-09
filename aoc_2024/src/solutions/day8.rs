pub mod day8 {
    
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut initial_map = Vec::new();
        
        // Char, [x, y]
        let mut antenna_positions: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
        
        for (li, line) in (&lines).iter().enumerate() {
            
            initial_map.push(line.chars().collect::<Vec<_>>());
            
            for (j, c) in (&initial_map[li]).iter().enumerate() {
                if *c != '.' {
                    if antenna_positions.contains_key(c) {
                        antenna_positions.get_mut(c).unwrap().push((j as isize, li as isize));
                    } else {
                        antenna_positions.insert(*c, vec![(j as isize, li as isize)]);
                    }
                }
            }
            
        }
        
        //println!("{:?}", antenna_positions);
        
        let mut map = initial_map.clone();
        
        let mut antinode_count = 0;
        
        // Check and mark a position on the map
        fn check_position (position: (isize, isize), count: &mut i32, map: &mut Vec<Vec<char>>) {
            
            if position.0 >= 0 && position.0 < map[0].len() as isize && position.1 >= 0 && position.1 < map.len() as isize {
                if map[position.1 as usize][position.0 as usize] != '#' {
                    *count += 1;
                    map[position.1 as usize][position.0 as usize] = '#';
                }
            }
            
        }
        
        for freq in antenna_positions.values() {
            for a in 0..freq.len() {
                for b in (a+1)..freq.len() {
                    
                    let gap = (freq[b].0 - freq[a].0, freq[b].1 - freq[a].1);
                    
                    check_position((freq[a].0 - gap.0, freq[a].1 - gap.1), &mut antinode_count, &mut map);
                    check_position((freq[b].0 + gap.0, freq[b].1 + gap.1), &mut antinode_count, &mut map);
                    
                }
            }
        }
        
        println!("Part 1: {}", antinode_count);
        
        
        // ==== Part 2 ==== //
        
        let mut map = initial_map.clone();
        
        let mut antinode_count = 0;
        
        for freq in antenna_positions.values() {
            for a in 0..freq.len() {
                for b in (a+1)..freq.len() {
                    
                    let gap = (freq[b].0 - freq[a].0, freq[b].1 - freq[a].1);
                    
                    let factor = gcd(gap.0.abs(), gap.1.abs());
                    let increment = (gap.0 / factor, gap.1 / factor);
                    
                    let mut pos = (freq[a].0, freq[a].1);
                    while pos.0 >= 0 && pos.0 < map[0].len() as isize && pos.1 >= 0 && pos.1 < map.len() as isize {
                        check_position(pos, &mut antinode_count, &mut map);
                        
                        pos.0 -= increment.0;
                        pos.1 -= increment.1;
                    }
                    
                    let mut pos = (freq[a].0, freq[a].1);
                    while pos.0 >= 0 && pos.0 < map[0].len() as isize && pos.1 >= 0 && pos.1 < map.len() as isize {
                        check_position(pos, &mut antinode_count, &mut map);
                        
                        pos.0 += increment.0;
                        pos.1 += increment.1;
                    }
                    
                }
            }
        }
        
        println!("Part 2: {}", antinode_count);
        
    }
    
    /// Find the greatest common divisor of two integers
    fn gcd(a: isize, b: isize) -> isize {
        if a == b {
            return a;
        }
        let mut x = a.min(b);
        let mut y = a.max(b);
        
        while y > 0 {
            let temp = x;
            x = y;
            y = temp % y;
        }
        return x;
    }
    
}
