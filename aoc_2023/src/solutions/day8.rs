pub mod day8 {
    
    pub fn solution(lines: Vec<String>) {
        
        let instructions = lines[0].chars().collect::<Vec<_>>();
        
        let mut nodes = Vec::new();
        for line in &lines[2..] {
            let replaced = line.replace("(", "").replace(")", "");
            let split1 = replaced
                .split(" = ")
                .map(|s| String::from(s))
                .collect::<Vec<String>>();
            let split2 = split1[1]
                .split(", ")
                .map(|s| String::from(s))
                .collect::<Vec<String>>();
            nodes.push( (split1[0].clone(), (split2[0].clone(), split2[1].clone())) );
        }
        
        let mut encoded_nodes = Vec::new();
        for node in &nodes {
            let left = nodes.iter().position(|n| n.0 == node.1.0).unwrap();
            let right = nodes.iter().position(|n| n.0 == node.1.1).unwrap();
            encoded_nodes.push((
                node.0.chars().collect::<Vec<_>>(),
                (left, right)
            ));
        }
        
        //println!("{:?}", nodes);
        
        
        // ==== Part 1 ==== //
        
        let mut current_node = nodes.iter().position(|n| n.0 == "AAA").unwrap();
        
        let mut steps = 0;
        
        while nodes[current_node].0 != "ZZZ" {
            
            if instructions[steps as usize % instructions.len()] == 'L' {
                current_node = encoded_nodes[current_node].1.0;
            } else {
                current_node = encoded_nodes[current_node].1.1;
            }
            
            steps += 1;
        }
        
        println!("Part 1: {}", steps);
        
        
        // ==== Part 2 ==== //
        
        let current_nodes = encoded_nodes
            .iter()
            .enumerate()
            .filter(|(_i, n)| n.0[2] == 'A')
            .map(|(i, _n)| i)
            .collect::<Vec<usize>>();
        
        let mut step_counts = Vec::new();
        
        // Calculate individual step counts for each starting position
        for node in &current_nodes {
            let mut current_pos = *node;
            let mut individual_steps: u64 = 0;
            
            while encoded_nodes[current_pos].0[2] != 'Z' {
                
                if instructions[individual_steps as usize % instructions.len()] == 'L' {
                    current_pos = encoded_nodes[current_pos].1.0;
                } else {
                    current_pos = encoded_nodes[current_pos].1.1;
                }
                
                individual_steps += 1;
            }
            
            step_counts.push(individual_steps);
        }
        
        // Find the lowest common multiple of the step counts and the number of instructions
        let mut lcm = instructions.len() as u64;
        
        for n in step_counts {
            // Find the greatest common divisor
            let mut gcd = lcm;
            let mut b = n;
            while b != 0 {
                (gcd, b) = (b, gcd % b);
            }
            // Calculate the new LCM
            lcm = lcm*n / gcd;
        }
        
        println!("Part 2: {}", lcm);
        
        
        /* // First idea: way too slow
        let mut steps2: u64 = 0;
        loop {
            
            //println!("{:?}", current_nodes);
            if steps2 % 1_000_000 == 0 {
                println!("{}", steps2);
            }
            
            let mut new_nodes = Vec::new();
            
            let mut all_at_end = true;
            
            for node in &current_nodes {
                all_at_end = all_at_end && encoded_nodes[*node].0[2] == 'Z';
                
                if instructions[steps2 as usize % instructions.len()] == 'L' {
                    new_nodes.push(encoded_nodes[*node].1.0);
                } else {
                    new_nodes.push(encoded_nodes[*node].1.1);
                }
                
            }
            
            if all_at_end {
                break;
            }
            
            current_nodes = new_nodes;
            
            steps2 += 1;
        }
        println!("Part 2: {}", steps2);
        */
        
        
    }
    
}
