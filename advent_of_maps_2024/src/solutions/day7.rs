pub mod day7 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Find quadratic solutions
        
        let arr = lines[0].split_ascii_whitespace().map(|s| s.parse::<f64>().unwrap()).collect::<Vec<_>>();
        
        let a = arr[0];
        let b = arr[1];
        let c = arr[2];
        
        if b*b - 4.0*a*c < 0.0 {
            // No solution
            println!("-");
            return;
        }
        
        let x1 = (-b + (b*b - 4.0*a*c).sqrt()) / (2.0*a);
        let x2 = (-b - (b*b - 4.0*a*c).sqrt()) / (2.0*a);
        
        println!("Solutions: {} {}", x1.round(), x2.round());
        
    }
    
}
