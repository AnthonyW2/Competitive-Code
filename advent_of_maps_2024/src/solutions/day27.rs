pub mod day27 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Find the modular multiplicative inverse of a list of integers
        
        let arr1 = lines[1].split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        let m = 6373;
        
        // Original, inefficient solution
        //fn find_mod_mult_inv(a: u64, m: u64) -> u64 {
        //    let mut b = 1;
        //    while a*b % m != 1 {
        //        b += 1;
        //    }
        //    return b;
        //}
        
        /// Find the modular moultiplicative inverse of a mod m, using the Extended Euclidean Algorithm
        fn find_mod_mult_inv(a: u64, m: u64) -> u64 {
            let (_gcd, x, _y) = ext_euclidean_alg(a, m);
            return ((x + (m as i64)) % (m as i64)) as u64;
        }
        
        let mut sum = 0;
        
        for a in arr1 {
            sum += find_mod_mult_inv(a, m);
        }
        
        println!("Solution: {}", sum);
        
    }
    
    /// Execute the Extended Euclidean Algorithm on two integers
    fn ext_euclidean_alg(a: u64, b: u64) -> (u64, i64, i64) {
        if a == b {
            return (a, 0, 1);
        }
        
        let (mut x, mut y);
        let (mut sp, mut s);
        let (mut tp, mut t);
        if a > b {
            x = b as i64;
            y = a as i64;
            (sp, s, tp, t) = (1,0,0,1);
        } else {
            x = a as i64;
            y = b as i64;
            (sp, s, tp, t) = (0,1,1,0);
        }
        
        while x > 0 {
            let temp_x = x;
            let q = y / x; // Quotient
            x = y % x; // New remainder
            y = temp_x; // Previous remainder
            
            let temp_s = s;
            s = sp - q * s;
            sp = temp_s;
            
            let temp_t = t;
            t = tp - q * t;
            tp = temp_t;
        }
        return (y as u64, sp, tp);
    }
    
}
