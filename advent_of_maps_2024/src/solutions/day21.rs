pub mod day21 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Calculate the sum of the minimum real, rounded roots of the polynomials
        
        // This "solution" reformats the input into a Mathematica expression. Evaluating it gives the answer directly.
        
        print!("Total[{{");
        
        for (li, line) in (&lines).iter().enumerate().skip(1) {
            
            let a = line.replace("pi", "Pi").replace("e", "E");
            let b = format!("Min[x/.Solve[{} == 0, Assumptions -> x ∈ Reals] // Round]", a);
            
            print!("{}", b);
            
            if li < lines.len()-1 {
                print!(",");
            }
            
        }
        
        println!("}}]");
        
        //println!("Solution: {}", "_");
        
    }
    
}
