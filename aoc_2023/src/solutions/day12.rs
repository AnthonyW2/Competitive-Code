pub mod day12 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        
        // ==== Part 1 ==== //
        
        fn find_arrangements(conditions: &Vec<char>, groups: &Vec<usize>) -> u32 {
            
            //println!("{:?}, {:?}", conditions, groups);
            
            if groups.len() == 0 {
                //println!("Success");
                if conditions.contains(&'#') {
                    return 0;
                }
                return 1;
            }
            if conditions.len() < groups[0] {
                //println!("Fail");
                return 0;
            }
            
            let mut output = 0;
            
            for i in 0..=(conditions.len() - groups[0]) {
                let mut valid_position = true;
                'check_validity: for j in i..(i+groups[0]) {
                    if conditions[j] != '#' && conditions[j] != '?' {
                        valid_position = false;
                        break 'check_validity;
                    }
                }
                //println!("Valid: {}", valid_position);
                if conditions.len() > i+groups[0] && conditions[i+groups[0]] == '#' {
                    valid_position = false;
                }
                if i > 0 && conditions[i-1] == '#' {
                    break;
                }
                
                if valid_position {
                    let cond_slice = Vec::from(&conditions[ (conditions.len()).min(i+groups[0]+1)..]);
                    let group_slice = Vec::from(&groups[1..]);
                    output += find_arrangements(&cond_slice, &group_slice);
                }
            }
            
            //println!("Output: {}", output);
            
            return output;
        }
        
        let mut arrangements = 0;
        
        for line in &lines {
            //println!("{}", line);
            
            let mut split = line.split(' ');
            let conditions = split.next().unwrap().chars().collect::<Vec<_>>();
            let groups = split.next().unwrap().split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            
            //println!("{:?}\n{:?}", conditions, groups);
            
            arrangements += find_arrangements(&conditions, &groups);
            //println!("{}",arrangements);
            
        }
        
        println!("Part 1: {}", arrangements);
        
        
        // ==== Part 2 ==== //
        
        let mut arrangements2 = 0;
        
        for line in &lines {
            println!("{}", line);
            
            let mut split = line.split(' ');
            let conditions = split.next().unwrap().chars().collect::<Vec<_>>();
            let conditions_5: Vec<char> = [&conditions[..], &['?'], &conditions[..], &['?'], &conditions[..], &['?'], &conditions[..], &['?'], &conditions[..]].concat();
            let groups = split.next().unwrap().split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let groups_5: Vec<usize> = [&groups[..], &groups[..], &groups[..], &groups[..], &groups[..]].concat();
            
            //println!("{:?}\n{:?}", conditions, groups);
            
            arrangements2 += find_arrangements(&conditions_5, &groups_5);
            //println!("{}", arrangements2);
            
        }
        
        println!("Part 2: {}", arrangements2);
        
    }
    
}
