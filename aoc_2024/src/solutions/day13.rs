pub mod day13 {
    
    //use std::collections::BinaryHeap;
    //use std::cmp::Reverse;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        // A, B, Prize
        let mut machines: Vec<((usize, usize), (usize, usize), (usize, usize))> = Vec::new();
        
        const A_COST: usize = 3;
        const B_COST: usize = 1;
        
        for (li, line) in (&lines).iter().enumerate() {
            match li % 4 {
                0 => {
                    // A
                    let split1 = line.split(": ").collect::<Vec<_>>()[1].split(", ").collect::<Vec<_>>();
                    let x = split1[0].split("+").nth(1).unwrap().parse::<usize>().unwrap();
                    let y = split1[1].split("+").nth(1).unwrap().parse::<usize>().unwrap();
                    machines.push(((x,y),(0,0),(0,0)));
                }
                1 => {
                    // B
                    let split1 = line.split(": ").collect::<Vec<_>>()[1].split(", ").collect::<Vec<_>>();
                    let x = split1[0].split("+").nth(1).unwrap().parse::<usize>().unwrap();
                    let y = split1[1].split("+").nth(1).unwrap().parse::<usize>().unwrap();
                    machines.last_mut().unwrap().1 = (x,y);
                }
                2 => {
                    // Prize
                    let split1 = line.split(": ").collect::<Vec<_>>()[1].split(", ").collect::<Vec<_>>();
                    let x = split1[0].split("=").nth(1).unwrap().parse::<usize>().unwrap();
                    let y = split1[1].split("=").nth(1).unwrap().parse::<usize>().unwrap();
                    machines.last_mut().unwrap().2 = (x,y);
                }
                _ => { }
            }
        }
        
        fn calculate_pos(a: (usize, usize), b: (usize, usize), na: usize, nb: usize) -> (usize, usize) {
            return (a.0*na + b.0*nb, a.1*na + b.1*nb);
        }
        
        fn calculate_min_tokens1(a: (usize, usize), b: (usize, usize), p: (usize, usize)) -> Option<usize> {
            
            let mut na = 0;
            let mut nb = 100;
            
            while nb > 0 {
                
                let mut pos = calculate_pos(a, b, na, nb);
                
                while pos.0 > p.0 || pos.1 > p.1 {
                    if nb == 0 {
                        return None;
                    }
                    nb -= 1;
                    pos = calculate_pos(a, b, na, nb);
                }
                
                while pos.0 < p.0 || pos.1 < p.1 {
                    if na > 100 {
                        return None;
                    }
                    na += 1;
                    pos = calculate_pos(a, b, na, nb);
                }
                
                if pos == p {
                    return Some(na*A_COST + nb*B_COST);
                }
                
            }
            
            return None;
            
        }
        
        let mut tokens1 = 0;
        
        for (a, b, p) in machines.clone() {
            let result = calculate_min_tokens1(a,b,p);
            if result.is_some() {
                tokens1 += result.unwrap();
            }
        }
        
        println!("Part 1: {}", tokens1);
        
        
        // ==== Part 2 ==== //
        
        fn calculate_min_tokens2(a: (usize, usize), b: (usize, usize), p: (usize, usize)) -> Option<usize> {
            
            let mut na = 0;
            let mut nb = (p.0 / b.0) + 1;
            
            loop {
                
                let mut pos = calculate_pos(a, b, na, nb);
                
                if pos.0 > p.0 {
                    let dec = (pos.0-p.0)/b.0;
                    if dec > nb {
                        return None;
                    }
                    if dec == 0 && nb > 0 {
                        nb -= 1;
                    }
                    nb -= dec;
                    pos = calculate_pos(a, b, na, nb);
                }
                
                if pos.1 > p.1 {
                    let dec = (pos.1-p.1)/b.1;
                    if dec > nb {
                        return None;
                    }
                    if dec == 0 && nb > 0 {
                        nb -= 1;
                    }
                    nb -= dec;
                    pos = calculate_pos(a, b, na, nb);
                }
                
                if pos.0 < p.0 {
                    let inc = (p.0-pos.0)/a.0;
                    if inc == 0 {
                        na += 1;
                    }
                    na += inc;
                    pos = calculate_pos(a, b, na, nb);
                }
                
                if pos.1 < p.1 {
                    let inc = (p.1-pos.1)/a.1;
                    if inc == 0 {
                        na += 1;
                    }
                    na += inc;
                    pos = calculate_pos(a, b, na, nb);
                }
                    
                if pos == p {
                    return Some(na*A_COST + nb*B_COST);
                }
                
            }
            
        }
        
        let mut tokens2 = 0;
        
        for (a, b, p) in machines.into_iter().map(|(a,b,p)| (a,b,(p.0+10000000000000, p.1+10000000000000))) {
            let result = calculate_min_tokens2(a,b,p);
            if result.is_some() {
                tokens2 += result.unwrap();
            }
        }
        
        println!("Part 2: {}", tokens2);
        
    }
    
}
