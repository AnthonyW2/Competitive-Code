pub mod day19 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Find a the maximum sum of a square sub-grid of numbers
        
        let mut grid = Vec::new();
        
        for (_li, line) in (&lines).iter().enumerate() {
            let arr = line.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
            
            grid.push(arr);
            
        }
        
        let height = grid.len();
        let width = grid[0].len();
        
        let cache: Vec<Vec<i64>> = Vec::new();
        
        fn test_sum(grid: &Vec<Vec<i64>>, top_left: (usize, usize), size: usize) -> i64 {
            
            let mut sum = 0;
            
            for x in top_left.0..(top_left.0+size) {
                for y in top_left.1..(top_left.1+size) {
                    sum += grid[y][x];
                }
            }
            
            println!("{:?} {}", top_left, size);
            
            return sum;
            
        }
        
        let mut max = 0;
        
        for w in 1..=(width.max(height)) {
            for y1 in 0..(height+1-w) {
                for x1 in 0..(width+1-w) {
                        let result = test_sum(&grid, (x1,y1), w);
                        if result > max {
                            max = result;
                        }
                    }
                }
            }
        
        println!("Solution: {}", max);
        
    }
    
}
