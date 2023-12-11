pub mod day11 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        // ==== Part 1 ==== //
        
        let mut space = lines.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        
        // Collect coordinates of all galaxies
        let mut galaxies_pre_expantion = Vec::new();
        for row in space.iter().enumerate() {
            for col in row.1.iter().enumerate() {
                if *col.1 == '#' {
                    galaxies_pre_expantion.push((row.0, col.0));
                }
            }
        }
        
        // Quadratic loop through all galaxies, adding coordinate-differences to accumulator 1
        let mut pre_expantion_result = 0;
        for g1 in 0..galaxies_pre_expantion.len() {
            for g2 in (g1+1)..galaxies_pre_expantion.len() {
                pre_expantion_result += (galaxies_pre_expantion[g1].0 as i32 - galaxies_pre_expantion[g2].0 as i32).abs();
                pre_expantion_result += (galaxies_pre_expantion[g1].1 as i32 - galaxies_pre_expantion[g2].1 as i32).abs();
            }
        }
        
        // Find & expand all empty lines
        let mut row = 0;
        while row < space.len() {
            let mut blank_row = true;
            for char in &space[row] {
                if *char == '#' {
                    blank_row = false;
                    break;
                }
            }
            if blank_row {
                //println!("Cloning {:?}", space[row]);
                space.insert(row, space[row].clone());
                row += 1;
            }
            row += 1;
        }
        
        let mut col = 0;
        while col < space[0].len() {
            let mut blank_col = true;
            for char in (&space).iter().map(|v| v[col]) {
                if char == '#' {
                    blank_col = false;
                    break;
                }
            }
            if blank_col {
                for row in &mut space {
                    row.insert(col, '.');
                }
                col += 1;
            }
            col += 1;
        }
        
        // Collect coordinates of all galaxies
        let mut galaxies = Vec::new();
        for row in space.iter().enumerate() {
            for col in row.1.iter().enumerate() {
                if *col.1 == '#' {
                    galaxies.push((row.0, col.0));
                }
            }
        }
        
        // Quadratic loop through all galaxies, adding coordinate-differences to accumulator 1
        let mut post_expantion_result = 0;
        for g1 in 0..galaxies.len() {
            for g2 in (g1+1)..galaxies.len() {
                post_expantion_result += (galaxies[g1].0 as i32 - galaxies[g2].0 as i32).abs();
                post_expantion_result += (galaxies[g1].1 as i32 - galaxies[g2].1 as i32).abs();
            }
        }
        
        println!("Part 1: {}", post_expantion_result);
        
        
        // ==== Part 2 ==== //
        
        println!("Part 2: {}", (post_expantion_result-pre_expantion_result)*100);
        
    }
    
}
