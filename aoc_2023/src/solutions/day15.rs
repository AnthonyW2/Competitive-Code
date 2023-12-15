pub mod day15 {
    
    pub fn solution(lines: Vec<String>) {
        
        let sequence = lines[0].split(",").collect::<Vec<_>>();
        
        // ==== Part 1 ==== //
        
        fn hash(string: &str) -> usize {
            let mut value = 0;
            for c in string.chars() {
                value += c as usize;
                value *= 17;
                value %= 256;
            }
            return value;
        }
        
        let mut result1 = 0;
        
        for code in &sequence {
            result1 += hash(code);
        }
        
        println!("Part 1: {}", result1);
        
        
        // ==== Part 2 ==== //
        
        let mut boxes: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];
        
        for code in &sequence {
            
            let split_pos = code.chars().position(|c| c == '-' || c == '=').unwrap();
            let split = code.split_at(split_pos);
            
            let label = split.0;
            let box_idx = hash(label);
            let mut instruction = split.1.chars();
            
            if instruction.next().unwrap() == '=' {
                let focal_len = instruction.next().unwrap().to_digit(10).unwrap();
                let old_lens_pos = boxes[box_idx].iter().position(|s| s.0 == label);
                if old_lens_pos.is_some() {
                    // Replace lens
                    boxes[box_idx][old_lens_pos.unwrap()].1 = focal_len;
                } else {
                    // Add new lens
                    boxes[box_idx].push((label, focal_len));
                }
            } else {
                let pos = boxes[box_idx].iter().position(|s| s.0 == label);
                if pos.is_some() {
                    // Remove lens
                    boxes[box_idx].remove(pos.unwrap());
                }
            }
        }
        
        let mut result2 = 0;
        
        for b in (&boxes).iter().enumerate() {
            for l in b.1.iter().enumerate() {
                result2 += (1+b.0) * (1+l.0) * (l.1.1 as usize);
            }
        }
        
        println!("Part 2: {}", result2);
        
    }
    
}
