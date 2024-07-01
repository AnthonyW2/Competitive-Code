pub mod day19 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Find a the maximum sum of a square sub-grid of numbers
        
        // Construct the grid of numbers from the input
        let mut grid = Vec::new();
        for (_li, line) in (&lines).iter().enumerate() {
            grid.push(line.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>());
        }
        
        let height = grid.len();
        let width = grid[0].len();
        
        // Create a rolling sum along all horizontal lines
        let mut h_sum_grid = vec![vec![0; width]; height];
        for y in 0..height {
            h_sum_grid[y][0] = grid[y][0];
            for x in 1..width {
                h_sum_grid[y][x] = h_sum_grid[y][x-1] + grid[y][x];
            }
        }
        
        // Each entry (i,j) is the sum of the rectangle from the origin to (i,j)
        let mut full_sum_grid = vec![vec![0; width]; height];
        for x in 0..width {
            full_sum_grid[0][x] = h_sum_grid[0][x];
            for y in 1..height {
                full_sum_grid[y][x] = full_sum_grid[y-1][x] + h_sum_grid[y][x];
            }
        }
        
        // Find the sum of a square with given coordinates & size, using the full_sum_grid
        fn get_sum(full_sum_grid: &Vec<Vec<i64>>, x: usize, y: usize, size: usize) -> i64 {
            
            let x2 = x+size-1;
            let y2 = y+size-1;
            
            let mut sum = full_sum_grid[y2][x2];
            
            if x > 0 {
                sum -= full_sum_grid[y2][x-1];
            }
            if y > 0 {
                sum -= full_sum_grid[y-1][x2];
                if x > 0 {
                    sum += full_sum_grid[y-1][x-1];
                }
            }
            
            return sum;
            
        }
        
        // Check all sub-squares of the grid to find the maximum sum
        let mut max = 0;
        for w in 1..=(width.max(height)) {
            for x in 0..(width+1-w) {
                for y in 0..(height+1-w) {
                    let result = get_sum(&full_sum_grid, x, y, w);
                    if result > max {
                        max = result;
                    }
                }
            }
        }
        
        println!("Solution: {}", max);
        
    }
    
}
