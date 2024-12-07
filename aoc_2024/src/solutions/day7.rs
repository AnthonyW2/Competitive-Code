pub mod day7 {
    
    pub fn solution(lines: Vec<String>) {
        
        fn find_sol (cache: &mut Vec<u64>, nums: &[u64], target: u64, concatenate: bool) -> bool {
            
            if nums.len() == 1 {
                return target == nums[0];
            }
            
            // Addition
            if target >= nums[nums.len()-1] {
                let result1 = find_sol(cache, &nums[0..(nums.len()-1)], target - nums[nums.len()-1], concatenate);
                if result1 {
                    return true;
                }
            }
            
            // Multiplication
            if target % nums[nums.len()-1] == 0 {
                let result2 = find_sol(cache, &nums[0..(nums.len()-1)], target / nums[nums.len()-1], concatenate);
                if result2 {
                    return true;
                }
            }
            
            // Concatenation (part 2)
            if concatenate && target > nums[nums.len()-1] {
                
                let new_target = target - nums[nums.len()-1];
                let magnitude = 10u64.pow(nums[nums.len()-1].to_string().len() as u32);
                if new_target % magnitude == 0 {
                    let result3 = find_sol(cache, &nums[0..(nums.len()-1)], new_target / magnitude, concatenate);
                    if result3 {
                        return true;
                    }
                }
                
            }
            
            return false;
            
        }
        
        let mut sum1 = 0;
        let mut sum2 = 0;
        
        for line in &lines {
            
            let mut cache = Vec::new();
            let mut split1 = line.split(": ");
            let target = split1.next().unwrap().parse::<u64>().unwrap();
            let nums = split1.next().unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
            
            let result1 = find_sol(&mut cache, &nums[..], target, false);
            if result1 {
                sum1 += target;
            }
            
            let result2 = find_sol(&mut cache, &nums[..], target, true);
            if result2 {
                sum2 += target;
            }
            
        }
        
        println!("Part 1: {}", sum1);
        
        println!("Part 2: {}", sum2);
        
    }
    
}
