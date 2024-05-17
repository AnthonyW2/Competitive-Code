pub mod day17 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Insert signs into a sum to get a target value
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let arr1 = lines[1].split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        
        let k = arr0[1];
        
        // Given a list of numbers, insert signs between them such that they add up to the target value
        fn insert_signs(nums: &Vec<i64>, i: usize, target: i64, answer: String) -> Option<String> {
            
            if i == nums.len() {
                if target == 0 {
                    return Some(answer);
                } else {
                    return None;
                }
            }
            
            // Plus
            let mut new_str_1 = answer.clone();
            if i == 0 {
                new_str_1.push_str(&format!("{}", nums[i]));
            } else {
                //new_str_1.push_str(&format!("+{}", nums[i]));
                new_str_1.push_str(&format!("plus{}", nums[i]));
            }
            let num_add_result = insert_signs(nums, i+1, target-nums[i], new_str_1);
            
            if num_add_result.is_some() {
                return num_add_result;
            }
            
            // Minus
            let mut new_str_2 = answer.clone();
            //new_str_2.push_str(&format!("-{}", nums[i]));
            new_str_2.push_str(&format!("minus{}", nums[i]));
            let num_subtract_result = insert_signs(nums, i+1, target+nums[i], new_str_2);
            
            return num_subtract_result;
            
        }
        
        let solution = insert_signs(&arr1, 0, k, String::new());
        
        println!("Solution: {}", solution.unwrap());
        
    }
    
}
