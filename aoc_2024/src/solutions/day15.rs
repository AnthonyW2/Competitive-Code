pub mod day15 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut map = Vec::new();
        let mut moves = Vec::new();
        
        let mut input_mode = 0;
        
        for line in &lines {
            if input_mode == 0 {
                if line.len() == 0 {
                    input_mode = 1;
                } else {
                    map.push(line.chars().collect::<Vec<_>>());
                }
            } else {
                moves.append(&mut line.chars().collect::<Vec<_>>())
            }
        }
        
        println!("Part 1: {}", "_");
        
        
        // ==== Part 2 ==== //
        
        println!("Part 2: {}", "_");
        
    }
    
}
