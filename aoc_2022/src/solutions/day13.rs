pub mod day13 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        let mut packet_pairs = Vec::new();
        
        for l in 0..(lines.len()+1)/3 {
            if lines[l*3].len() > 0 {
                packet_pairs.push((
                    lines[l*3].chars().collect::<Vec<_>>(),
                    lines[l*3+1].chars().collect::<Vec<_>>()
                ));
            }
        }
        
        //println!("{:?}", packet_pairs);
        
        
        // ==== Part 1 ==== //
        
        let mut right_order_indices = Vec::new();
        
        for (i, pair) in packet_pairs.iter().enumerate() {
            
            if compare(pair.0.clone(), pair.1.clone()) {
                right_order_indices.push(i+1);
            }
            
        }
        
        //println!("right_order_indices: {:?}", right_order_indices);
        println!("Part 1: {}", right_order_indices.iter().sum::<usize>());
        
        
        // ==== Part 2 ==== //
        
        let mut all_packets = lines.iter()
            .filter(|s| s.len() > 0)
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        all_packets.push(vec!['[','[','2',']',']']);
        all_packets.push(vec!['[','[','6',']',']']);
        
        all_packets.sort_by(|a, b| {
            let a_smaller = compare(a.clone(), b.clone());
            if a_smaller {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        });
        
        let idx1 = all_packets.iter().position(|cv| cv.len() == 5 && cv[1] == '[' && cv[2] == '2').unwrap()+1;
        let idx2 = all_packets.iter().position(|cv| cv.len() == 5 && cv[1] == '[' && cv[2] == '6').unwrap()+1;
        
        println!("Part 2: {}", idx1*idx2);
        
    }
    
    fn compare(packet1: Vec<char>, packet2: Vec<char>) -> bool {
        
        let mut p1_iter = packet1.iter();
        let mut p2_iter = packet2.iter();
        p1_iter.next(); // Skip the first '['
        p2_iter.next();
        
        loop {
            // Process one entry of the outer list per loop
            
            let mut int_1: Option<u32> = None;
            let mut list_1: Vec<char> = Vec::new();
            let mut int_2: Option<u32> = None;
            let mut list_2: Vec<char> = Vec::new();
            
            // Check initial char for basic info
            let next1 = p1_iter.next();
            if next1.is_none() {
                return true;
            }
            let mut prev_char_1 = next1.unwrap();
            match prev_char_1 {
                '0'..='9' => {
                    // Entry is an integer - collect all digits & parse
                    let mut digits = Vec::new();
                    while *prev_char_1 >= '0' && *prev_char_1 <= '9' {
                        digits.push(prev_char_1);
                        prev_char_1 = p1_iter.next().unwrap();
                    }
                    int_1 = Some( digits.iter().map(|&&c| c).collect::<String>().parse::<u32>().unwrap() );
                }
                '[' => {
                    // Entry is an list - collect all elements of the list
                    let mut chars = Vec::new();
                    let mut brace_lvls = 1;
                    while brace_lvls > 0 {
                        chars.push(prev_char_1);
                        prev_char_1 = p1_iter.next().unwrap();
                        match prev_char_1 {
                            '[' => brace_lvls += 1,
                            ']' => {
                                brace_lvls -= 1;
                            },
                            _ => {}
                        }
                    }
                    chars.push(prev_char_1);
                    p1_iter.next();
                    list_1 = chars.iter().map(|&&c| c).collect::<Vec<_>>();
                },
                ']' => { return true; },
                ',' => { panic!(); },
                _ => { panic!(); }
            }
            
            // Check initial char for basic info
            let next2 = p2_iter.next();
            if next2.is_none() {
                return false;
            }
            let mut prev_char_2 = next2.unwrap();
            match prev_char_2 {
                '0'..='9' => {
                    // Entry is an integer - collect all digits & parse
                    let mut digits = Vec::new();
                    while *prev_char_2 >= '0' && *prev_char_2 <= '9' {
                        digits.push(prev_char_2);
                        prev_char_2 = p2_iter.next().unwrap();
                    }
                    int_2 = Some( digits.iter().map(|&&c| c).collect::<String>().parse::<u32>().unwrap() );
                }
                '[' => {
                    // Entry is an list - collect all elements of the list
                    let mut chars = Vec::new();
                    let mut brace_lvls = 1;
                    while brace_lvls > 0 {
                        chars.push(prev_char_2);
                        prev_char_2 = p2_iter.next().unwrap();
                        match prev_char_2 {
                            '[' => brace_lvls += 1,
                            ']' => {
                                brace_lvls -= 1;
                            },
                            _ => {}
                        }
                    }
                    chars.push(prev_char_2);
                    p2_iter.next();
                    list_2 = chars.iter().map(|&&c| c).collect::<Vec<_>>();
                },
                ']' => { return false; },
                ',' => { panic!(); },
                _ => { panic!(); }
            }
            
            if int_1.is_some() && int_2.is_some() {
                // Compare two integers
                if int_1.unwrap() > int_2.unwrap() {
                    return false;
                } else if int_1.unwrap() < int_2.unwrap() {
                    return true;
                }
                
            } else if int_1.is_some() && int_2.is_none() {
                // Convert integer into list
                list_1 = format!("[{}]", int_1.unwrap()).chars().collect::<Vec<_>>();
                
            } else if int_1.is_none() && int_2.is_some() {
                // Convert integer into list
                list_2 = format!("[{}]", int_2.unwrap()).chars().collect::<Vec<_>>();
                
            }
            
            // Compare two lists recursively
            if list_1.len() > 0 && list_2.len() > 0 {
                return compare(list_1, list_2);
            }
            
        }
        
    }
    
}
