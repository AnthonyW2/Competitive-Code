pub mod day3 {
    
    use itertools::Itertools;
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let input = lines.join("");
        let mults = input.split("mul(").collect::<Vec<_>>();
        
        let mut sum = 0;
        
        for m in &mults {
            
            let chars = m.chars().collect::<Vec<_>>();
            let n1 = chars.clone().iter().take_while(|c| &&'0' <= c && c <= &&'9').collect::<String>();
            if n1.len() == 0 || n1.len() == chars.len() || chars[n1.len()] != ',' {
                continue;
            }
            let n2 = chars[n1.len()+1..].into_iter().take_while(|c| &&'0' <= c && c <= &&'9').collect::<String>();
            if n2.len() == 0 || n1.len() + 1 + n2.len() == chars.len() || chars[n1.len() + 1 + n2.len()] != ')' {
                continue;
            }
            
            sum += n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap();
            
        }
        
        println!("Part 1: {}", sum);
        
        
        // ==== Part 2 ==== //
        
        let mut chars = input.chars().peekable();
        
        let mul_prefix = Vec::from("mul(");
        let dont_prefix = Vec::from("n't()");
        let do_prefix = Vec::from("do()");
        
        let mut mul_state = true;
        
        let mut sum2 = 0;
        
        'main: loop {
            
            if chars.peek().is_none() {
                break 'main;
            }
            
            while chars.peek().is_some() && (chars.peek().unwrap() != &'m' && chars.peek().unwrap() != &'d') {
                chars.next();
            }
            
            let mut is_do = true;
            for c in &do_prefix {
                if chars.peek().is_some() && chars.peek().unwrap() == &(*c as char) {
                    chars.next();
                } else {
                    is_do = false;
                    break;
                }
            }
            if is_do {
                mul_state = true;
                continue 'main;
            }
            
            let mut is_dont = true;
            for c in &dont_prefix {
                if chars.peek().is_some() && chars.peek().unwrap() == &(*c as char) {
                    chars.next();
                } else {
                    is_dont = false;
                    break;
                }
            }
            if is_dont {
                mul_state = false;
                continue 'main;
            }
            
            let mut is_mul = true;
            'check_mul: for c in &mul_prefix {
                if chars.peek().is_some() && chars.peek().unwrap() == &(*c as char) {
                    chars.next();
                } else {
                    is_mul = false;
                    break 'check_mul;
                }
            }
            if !is_mul {
                continue 'main;
            }
            
            let n1 = (&mut chars).peeking_take_while(|c| &'0' <= c && c <= &'9').collect::<String>();
            
            let comma = (&mut chars).next();
            if comma.is_none() || comma.unwrap() != ',' {
                continue 'main;
            }
            
            let n2 = (&mut chars).peeking_take_while(|c| &'0' <= c && c <= &'9').collect::<String>();
            
            let close_bracket = (&mut chars).next();
            if close_bracket.is_none() || close_bracket.unwrap() != ')' {
                continue 'main;
            }
            
            if mul_state {
                sum2 += n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap();
            }
            
            
        }
        
        println!("Part 2: {}", sum2);
        
    }
    
}
