pub mod day14 {
    
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut platform = lines.iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        // Roll all round rocks north
        fn roll_north(platform: &mut Vec<Vec<char>>) {
            for r in 0..platform.len() {
                for c in 0..platform[0].len() {
                    
                    if platform[r][c] == 'O' {
                        platform[r][c] = '.';
                        let mut pos = r;
                        while pos > 0 && platform[pos-1][c] == '.' {
                            pos -= 1;
                        }
                        platform[pos][c] = 'O';
                    }
                }
            }
        }
        
        // Calculate the load level on a given platform state
        fn find_load_level(platform: &Vec<Vec<char>>) -> usize {
            let mut output = 0;
            for r in 0..platform.len() {
                for c in 0..platform[0].len() {
                    if platform[r][c] == 'O' {
                        output += platform.len() - r;
                    }
                }
            }
            return output;
        }
        
        roll_north(&mut platform);
        
        println!("Part 1: {}", find_load_level(&platform));
        
        
        // ==== Part 2 ==== //
        
        /**
         * Approach: Look for a repeating pattern (in load levels) & mathematically work out the result after 1000000000 cycles.
         */
        
        // Roll all round rocks south
        fn roll_south(platform: &mut Vec<Vec<char>>) {
            for r in (0..platform.len()).rev() {
                for c in 0..platform[0].len() {
                    
                    if platform[r][c] == 'O' {
                        platform[r][c] = '.';
                        let mut pos = r;
                        while pos < platform.len()-1 && platform[pos+1][c] == '.' {
                            pos += 1;
                        }
                        platform[pos][c] = 'O';
                    }
                }
            }
        }
        
        // Roll all round rocks east
        fn roll_east(platform: &mut Vec<Vec<char>>) {
            for c in (0..platform[0].len()).rev() {
                for r in 0..platform.len() {
                    
                    if platform[r][c] == 'O' {
                        platform[r][c] = '.';
                        let mut pos = c;
                        while pos < platform[0].len()-1 && platform[r][pos+1] == '.' {
                            pos += 1;
                        }
                        platform[r][pos] = 'O';
                    }
                }
            }
        }
        
        // Roll all round rocks west
        fn roll_west(platform: &mut Vec<Vec<char>>) {
            for c in 0..platform[0].len() {
                for r in 0..platform.len() {
                    
                    if platform[r][c] == 'O' {
                        platform[r][c] = '.';
                        let mut pos = c;
                        while pos > 0 && platform[r][pos-1] == '.' {
                            pos -= 1;
                        }
                        platform[r][pos] = 'O';
                    }
                }
            }
        }
        
        // Roll north, west, south, east
        fn spin_cycle(platform: &mut Vec<Vec<char>>) {
            roll_north(platform);
            roll_west(platform);
            roll_south(platform);
            roll_east(platform);
        }
        
        // After finding a repeating pattern this vector will contain two copies of the pattern
        let mut results = Vec::new();
        
        // Count how many times each load level has been seen
        let mut seen_values: HashMap<usize, u32> = HashMap::new();
        
        // Mark whether or not any load level has been seen more than once
        let mut seen_repeat = false;
        
        // Store the index where the repeating pattern starts
        let mut pattern_start = 0;
        
        let mut cycle = 0;
        loop {
            spin_cycle(&mut platform);
            
            // Calculate load level
            let load_level = find_load_level(&platform);
            
            if results.contains(&load_level) {
                if seen_repeat {
                    if seen_values.values().find(|v| **v < 2).is_none() {
                        break;
                    }
                } else {
                    results = Vec::new();
                    seen_repeat = true;
                    pattern_start = cycle;
                }
            }
            
            results.push(load_level);
            if seen_repeat {
                seen_values.entry(load_level).and_modify(|n| { *n += 1 }).or_insert(1);
            }
            
            cycle += 1;
        }
        
        println!("Part 2: {}", results[(1_000_000_000 - pattern_start - 1) % results.len()]);
        
    }
    
}
