pub mod day11 {
    
    pub fn solution(lines: Vec<String>) {
        
        //println!("{:?}", lines);
        
        // ==== Part 1 ==== //
        
        let space = lines.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        
        let mut blank_rows = Vec::new();
        let mut blank_cols = Vec::new();
        
        // Find all empty lines
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
                blank_rows.push(row);
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
                blank_cols.push(col);
            }
            col += 1;
        }
        
        // Collect coordinates of all galaxies (row, col)
        let mut galaxies = Vec::new();
        for row in space.iter().enumerate() {
            for col in row.1.iter().enumerate() {
                if *col.1 == '#' {
                    galaxies.push((row.0, col.0));
                }
            }
        }
        
        // Iterate through each pair of galaxies, adding up all coordinate differences & accounting for expansion
        fn sum_of_distances(galaxies: &Vec<(usize, usize)>, expansion_rows: &Vec<usize>, expansion_cols: &Vec<usize>, expansion_magnitude: u64) -> u64 {
            let mut result = 0;
            for g1 in 0..galaxies.len() {
                for g2 in (g1+1)..galaxies.len() {
                    let lower_row = galaxies[g1].0.min(galaxies[g2].0);
                    let upper_row = galaxies[g1].0.max(galaxies[g2].0);
                    for row in expansion_rows {
                        if *row >= lower_row && *row <= upper_row {
                            result += expansion_magnitude;
                        }
                    }
                    let lower_col = galaxies[g1].1.min(galaxies[g2].1);
                    let upper_col = galaxies[g1].1.max(galaxies[g2].1);
                    for col in expansion_cols {
                        if *col >= lower_col && *col <= upper_col {
                            result += expansion_magnitude;
                        }
                    }
                    result += (upper_row - lower_row) as u64;
                    result += (upper_col - lower_col) as u64;
                }
            }
            return result;
        }
        
        println!("Part 1: {}", sum_of_distances(&galaxies, &blank_rows, &blank_cols, 1));
        
        
        // ==== Part 2 ==== //
        
        println!("Part 2: {}", sum_of_distances(&galaxies, &blank_rows, &blank_cols, 999999));
        
    }
    
}
