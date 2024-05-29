pub mod day29 {
    
    use std::collections::HashSet;
    
    pub fn solution(lines: Vec<String>) {
        
        // Find unique colourings of n-sided-polygon with k colours ()
        
        let mut sum = 0;
        
        for (_li, line) in (&lines).iter().enumerate().skip(1) {
            let arr = line.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            
            let result = find_number_of_colourings(arr[0] as usize, arr[1]);
            
            println!("{}-gon with {} colours: {}", arr[0], arr[1], result);
            
            sum += result;
            
        }
        
        /// Find the number of colourings of an n-gon with k colours available.
        /// Rotated or flipped polygons are treated as identical.
        fn find_number_of_colourings(n: usize, k: u32) -> u32 {
            
            if k == 1 {
                return 1;
            }
            
            let mut colouring_count = 1;
            
            // All colours combinations, stored in hashsets of combinations with matching digit sums
            let mut colour_combinations: Vec<HashSet<u32>> = vec![HashSet::new(); n*(k as usize)];
            
            // Store the current combination as a vector of base-k digits
            let mut current_combination = vec![0; n];
            
            // Iterate through all base-k numbers
            'base_k_numbers: loop {
                
                // Add one to the base-k number
                current_combination[n-1] += 1;
                // Ensure the vector is a valid base-k number
                for i in (0..n).rev() {
                    if current_combination[i] >= k {
                        if i != 0 {
                            current_combination[i] = 0;
                            current_combination[i-1] += 1;
                        } else {
                            break 'base_k_numbers;
                        }
                    } else {
                        break;
                    }
                    
                }
                
                // Find the sum of digits of this combination
                let digit_sum = current_combination.iter().sum::<u32>() as usize;
                
                // Find the minimal rotation of the current combination, as a base-k integer
                let rotated_combination = find_smallest_rotation(&current_combination, k);
                // Find the minimal rotation of the reverse of the current combination
                let flipped = current_combination.clone().into_iter().rev().collect::<Vec<u32>>();
                let rotated_flipped_combination = find_smallest_rotation(&flipped, k);
                
                // Add the combination if it is unique
                if !colour_combinations[digit_sum].contains(&rotated_combination) && !colour_combinations[digit_sum].contains(&rotated_flipped_combination) {
                    colour_combinations[digit_sum].insert(rotated_combination);
                    colouring_count += 1;
                }
                
            }
            
            return colouring_count;
            
        }
        
        // Given a vector of base-k digits, find the rotation that minimises the value if it was turned into a base-k integer, and return that integer
        fn find_smallest_rotation(combination: &Vec<u32>, k: u32) -> u32 {
            
            let mut smallest_sum = u32::MAX;
            
            for r in 0..combination.len() {
                
                let new_sum = combination.iter()
                    .rev()
                    .enumerate()
                    .map(|(i, n)| n*(k.pow(((i+r)%combination.len()) as u32)))
                    .sum();
                
                if new_sum < smallest_sum {
                    smallest_sum = new_sum;
                }
                
            }
            
            return smallest_sum;
            
        }
        
        println!("Solution: {}", sum);
        
    }
    
}
