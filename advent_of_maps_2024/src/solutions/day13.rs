pub mod day13 {
    
    pub fn solution(lines: Vec<String>) {
        
        // Just don't trust anyone anymore
        
        let mut answer = 0;
        
        let mut collection = Vec::new();
        
        for (_li, line) in (&lines).iter().enumerate() {
            let arr = line.split_ascii_whitespace().collect::<Vec<_>>();
            let ins = arr[0];
            
            if ins == "ADD" {
                let n = arr[1].parse::<usize>().unwrap();
                if !collection.contains(&n) {
                    collection.push(n);
                }
            } else if ins == "REMOVE" {
                let n = arr[1].parse::<usize>().unwrap();
                let i = collection.iter().position(|a| *a == n);
                if i.is_some() {
                    collection.remove(i.unwrap());
                }
            } else if ins == "ERASE" {
                // Do nothing.
            } else if ins == "SIZE" {
                answer += collection.len();
            } else if ins == "MAX" {
                if collection.len() > 0 {
                    answer += collection.iter().max().unwrap();
                }
            }
            
        }
        
        println!("Actual solution: {}", answer);
        
    }
    
}
