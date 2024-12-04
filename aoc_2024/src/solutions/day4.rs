pub mod day4 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        let mut wordsearch: Vec<Vec<char>> = Vec::new();
        
        for line in &lines {
            wordsearch.push(line.chars().collect::<Vec<_>>());
        }
        
        fn find_word (ws: &Vec<Vec<char>>, word: &str, x: isize, y: isize, xv: isize, yv: isize) -> bool {
            
            for (i, c) in word.chars().enumerate() {
                if y+(i as isize)*yv < 0 || y+(i as isize)*yv >= ws.len() as isize || x+(i as isize)*xv < 0 || x+(i as isize)*xv >= ws[0].len() as isize {
                    return false;
                }
                
                let c1 = ws[(y+(i as isize)*yv) as usize][(x+(i as isize)*xv) as usize];
                
                if c != c1 {
                    return false;
                }
            }
            
            return true;
            
        }
        
        let mut matches = 0;
        
        let xmas = "XMAS";
        
        for y in 0..(&wordsearch).len() as isize {
            for x in 0..(&wordsearch)[0].len() as isize {
                if find_word(&wordsearch, xmas, x, y, 0, 1) {
                    matches += 1;
                }
                if find_word(&wordsearch, xmas, x, y, 0, -1) {
                    matches += 1;
                }
                if find_word(&wordsearch, xmas, x, y, 1, 0) {
                    matches += 1;
                }
                if find_word(&wordsearch, xmas, x, y, -1, 0) {
                    matches += 1;
                }
                if find_word(&wordsearch, xmas, x, y, 1, 1) {
                    matches += 1;
                }
                if find_word(&wordsearch, xmas, x, y, 1, -1) {
                    matches += 1;
                }
                if find_word(&wordsearch, xmas, x, y, -1, 1) {
                    matches += 1;
                }
                if find_word(&wordsearch, xmas, x, y, -1, -1) {
                    matches += 1;
                }
            }
        }
        
        println!("Part 1: {}", matches);
        
        
        // ==== Part 2 ==== //
        
        fn find_x_mas (ws: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
            let word = "MAS";
            
            if
                (find_word(ws, word, x-1, y-1, 1, 1) || find_word(ws, word, x+1, y+1, -1, -1))
                && (find_word(ws, word, x-1, y+1, 1, -1) || find_word(ws, word, x+1, y-1, -1, 1))
            {
                return true;
            }
            
            return false;
        }
        
        let mut matches2 = 0;
        
        for y in 0..(&wordsearch).len() as isize {
            for x in 0..(&wordsearch)[0].len() as isize {
                if find_x_mas(&wordsearch, x, y) {
                    matches2 += 1;
                }
            }
        }
        
        println!("Part 2: {}", matches2);
        
    }
    
}
