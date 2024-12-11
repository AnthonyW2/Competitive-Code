pub mod day11 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let starting_rocks = lines[0].split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        fn iterate_rocks (rocks: &Vec<u64>) -> Vec<u64> {
            
            let mut new_rocks = Vec::new();
            
            for rock in rocks {
                if *rock == 0 {
                    new_rocks.push(1);
                    continue;
                }
                let str = rock.to_string();
                if str.len() % 2 == 0 {
                    let left = &str[..str.len() / 2];
                    let right = &str[str.len() / 2..];
                    new_rocks.push(left.parse::<u64>().unwrap());
                    new_rocks.push(right.parse::<u64>().unwrap());
                } else {
                    new_rocks.push(rock * 2024);
                }
            }
            
            return new_rocks;
            
        }
        
        let mut rocks = starting_rocks.clone();
        
        let mut count1 = 0;
        
        for rock in rocks {
            
            let mut group = vec![rock];
            
            for i in 0..25 {
                
                group = iterate_rocks(&group);
                
            }
            
            //println!("{:?}", group);
            
            count1 += group.len();
            
        }
        
        println!("Part 1: {}", count1);
        
        
        // ==== Part 2 ==== //
        
        println!("Part 2: {}", "_");
        
    }
    
}
