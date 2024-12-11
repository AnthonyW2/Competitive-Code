pub mod day11 {
    
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let starting_rocks = lines[0].split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        // Key: (rock, iterations)
        // Value: count
        let mut cache: HashMap<(u64, u32), usize> = HashMap::new();
        
        fn iterate_rock (rock: u64, iterations: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
            
            // Base case
            if iterations == 0 {
                return 1;
            }
            
            // Return cached result
            if cache.contains_key(&(rock, iterations)) {
                return *cache.get(&(rock, iterations)).unwrap();
            }
            
            if rock == 0 {
                let result = iterate_rock(1, iterations-1, cache);
                cache.insert((rock, iterations), result);
                return result;
            }
            
            let str = rock.to_string();
            if str.len() % 2 == 0 {
                let left = &str[..str.len() / 2];
                let right = &str[str.len() / 2..];
                let mut count = 0;
                count += iterate_rock(left.parse::<u64>().unwrap(), iterations-1, cache);
                count += iterate_rock(right.parse::<u64>().unwrap(), iterations-1, cache);
                cache.insert((rock, iterations), count);
                return count;
            }
            
            let result = iterate_rock(rock * 2024, iterations-1, cache);
            cache.insert((rock, iterations), result);
            return result;
            
        }
        
        let mut rocks = starting_rocks.clone();
        
        let mut count1 = 0;
        
        for rock in rocks.clone() {
            
            count1 += iterate_rock(rock, 25, &mut cache);
            
        }
        
        println!("Part 1: {}", count1);
        
        
        // ==== Part 2 ==== //
        
        let mut count2 = 0;
        
        for rock in rocks {
            
            count2 += iterate_rock(rock, 75, &mut cache);
            
        }
        
        println!("Part 2: {}", count2);
        
    }
    
}
