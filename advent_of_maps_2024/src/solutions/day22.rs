pub mod day22 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Calculate the minimum amount of money to buy n items given two options
        
        let n = lines[0].parse::<u64>().unwrap();
        
        let arr1 = lines[1].split_ascii_whitespace().map(|s| s.replace("$","").parse::<u64>().unwrap()).collect::<Vec<_>>();
        let arr2 = lines[2].split_ascii_whitespace().map(|s| s.replace("$","").parse::<u64>().unwrap()).collect::<Vec<_>>();
        
        // Number gained
        // n1 represents the option that is cheaper per item
        // n1 represents the option that is more expensive per item
        let n1;
        let n2;
        
        // Price for n_ items
        let p1;
        let p2;
        
        // Assign values to n_ and p_, depending on which option is a better deal
        if arr1[1]/arr1[0] < arr2[1]/arr2[0] {
            n1 = arr1[0];
            n2 = arr2[0];
            p1 = arr1[1];
            p2 = arr2[1];
        } else {
            n1 = arr2[0];
            n2 = arr1[0];
            p1 = arr2[1];
            p2 = arr1[1];
        }
        
        // The number of items that can be bought with deal 1
        let mut allocated_to_1 = n - (n % n1);
        
        // Reduce allocated_to_1 until the remainder can be bought with deal 2
        while (n - allocated_to_1) % n2 != 0 {
            allocated_to_1 -= n1;
        }
        
        println!("Solution: {}", (allocated_to_1 * p1 / n1) + ((n-allocated_to_1) * p2 / n2));
        
    }
    
    /// Find the greatest common divisor of two integers
    fn _gcd(a: u64, b: u64) -> u64 {
        if a == b {
            return a;
        }
        let mut x = a.min(b);
        let mut y = a.max(b);
        
        while y > 0 {
            let temp = x;
            x = y;
            y = temp % y;
        }
        return x;
    }
    
    /// Find the lowest common multiple of two integers
    fn _lcm(a: u64, b: u64) -> u64 {
        return a*b / _gcd(a,b);
    }
    
}
