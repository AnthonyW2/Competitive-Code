pub mod day3 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        
        // ==== Part 1 ==== //
        
        // Relative offset to check adjacent positions (row, col)
        let adjacent_positions: [(isize, isize); 8] = [
            (-1,-1), (-1,0), (-1,1),
            (0,-1),          (0,1),
            (1,-1),  (1,0),  (1,1)
        ];
        
        let check_adjacent = |lines: &Vec<String>, row: isize, col: isize| -> bool {
            
            fn check_sym(c: char) -> bool {
                match c {
                    '0'..='9' | '.' => false,
                    _ => true
                }
            }
            
            for (drow, dcol) in adjacent_positions {
                if row+drow >= 0 && row+drow < lines.len() as isize && col+dcol >= 0 && col+dcol < lines[0].len() as isize {
                    if check_sym(lines[(row+drow) as usize].chars().nth((col+dcol) as usize).unwrap()) {
                        return true;
                    }
                }
            }
            
            return false;
            
        };
        
        let mut valid_numbers = Vec::new();
        
        for (row, line) in (&lines).iter().enumerate() {
            
            let mut current_num = String::new();
            let mut is_valid = false;
            let mut is_num = false;
            
            for (col, c) in line.chars().enumerate() {
                
                match c {
                    '0'..='9' => {
                        current_num.push(c);
                        is_num = true;
                        // Check adjacent for symbols
                        is_valid = is_valid || check_adjacent(&lines, row as isize, col as isize);
                    },
                    _ => {
                        if is_num && is_valid {
                            valid_numbers.push((
                                current_num.parse::<u64>().unwrap(), // part number
                                row, // line it's in
                                col - current_num.len(), // column of leftmost digit
                                col - 1 // column of rightmost digit
                            ));
                        }
                        is_num = false;
                        is_valid = false;
                        current_num = String::new();
                    }
                }
                
            }
            
            if is_num && is_valid {
                valid_numbers.push((
                    current_num.parse::<u64>().unwrap(),
                    row,
                    line.len() - current_num.len(),
                    line.len() - 1
                ));
            }
            
        }
        
        //println!("{:?}", valid_numbers);
        
        println!("Part 1: {}", valid_numbers.iter().map(|t| t.0).sum::<u64>());
        
        
        // ==== Part 2 ==== //
        
        let get_gear_ratio = |lines: &Vec<String>, row: isize, col: isize| -> u64 {
            
            let valid_positions = adjacent_positions.iter()
                .filter(|(drow, dcol)| row+drow >= 0 && row+drow < lines.len() as isize && col+dcol >= 0 && col+dcol < lines[0].len() as isize)
                .map(|(drow, dcol)| ((row+drow) as usize, (col+dcol) as usize))
                .collect::<Vec<_>>();
            
            let matching_nums = valid_numbers.iter().filter(|num| {
                for (r, c) in &valid_positions {
                    if num.1 == *r && *c >= num.2 && *c <= num.3 { return true; }
                }
                return false;
            }).collect::<Vec<_>>();
            
            if matching_nums.len() == 2 {
                return matching_nums[0].0 * matching_nums[1].0;
            }
            
            return 0;
            
        };
        
        let mut acc2 = 0;
        
        for (row, line) in (&lines).iter().enumerate() {
            
            for (col, c) in line.chars().enumerate() {
                
                match c {
                    '*' => {
                        acc2 += get_gear_ratio(&lines, row as isize, col as isize);
                    },
                    _ => {}
                }
                
            }
            
        }
        
        println!("Part 2: {}", acc2);
        
    }
    
}
