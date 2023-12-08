pub mod day4 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        
        // ==== Part 1 & 2 ==== //
        
        let mut acc1 = 0;
        let mut acc2 = 0;
        
        let mut num_cards = vec![1u32; lines.len()];
        
        for (li, line) in (&lines).iter().enumerate() {
            let mut split1 = line.split(": ").last().unwrap().split(" | ");
            let winning = split1.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            let winning_set = std::collections::BTreeSet::from_iter(winning.iter().cloned());
            let our_nums = split1.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            let our_num_set = std::collections::BTreeSet::from_iter(our_nums.iter().cloned());
            
            let intersection: Vec<_> = winning_set.intersection(&our_num_set).cloned().collect();
            if intersection.len() > 0 {
                acc1 += 2u64.pow(intersection.len() as u32 - 1); // Part 1
                
                // Part 2
                for i in (li+1)..=(li+intersection.len()) {
                    if i >= lines.len() { break; }
                    num_cards[i] += num_cards[li];
                }
            }
            
            acc2 += num_cards[li]; // Part 2
            
        }
        
        println!("Part 1: {}", acc1);
        
        println!("Part 2: {}", acc2);
        
    }
    
}
