pub mod day30 {
    
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    
    pub fn solution(lines: Vec<String>) {
        
        // Find minimum distance between two "special" nodes in a network
        
        let arr0 = lines[0].split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let special_rooms = lines[1].split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        
        // Each index i in edges stores a list of edges coming out of node i in the graph
        let mut edges = vec![Vec::new(); arr0[0]+1];
        
        for (_li, line) in (&lines).iter().enumerate().skip(2) {
            let arr = line.split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
            
            // Add a two-directional edge between the two nodes
            edges[arr[0]].push((arr[1], arr[2] as u64));
            edges[arr[1]].push((arr[0], arr[2] as u64));
            
        }
        
        let mut min_distance = u64::MAX;
        
        // Use Djikstra's algorithm to find the shortest path between every pair of special rooms
        // Using the Floyd-Warshall algorithm might be better here, but it was easier to use Djikstra's algorithm at the time
        for i in 0..special_rooms.len() {
            for j in (i+1)..special_rooms.len() {
                
                let new_distance = djikstra(&edges, special_rooms[i], special_rooms[j]);
                
                if new_distance.is_some() && new_distance.unwrap() < min_distance {
                    min_distance = new_distance.unwrap();
                }
                
            }
        }
        
        println!("Solution: {}", min_distance);
        
    }
    
    
    fn djikstra(edges: &Vec<Vec<(usize,u64)>>, start: usize, end: usize) -> Option<u64> {
        // Each entry in 'edges' represents a node and stores a list of connected nodes with their weights
        
        // Use a min-heap
        let mut pqueue: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
        pqueue.push(Reverse((0,start)));
        
        let mut visited = vec![false; edges.len()];
        
        // Store the predecessor ID of each traversed node
        let mut predecessors = vec![None; edges.len()];
        
        while pqueue.len() > 0 {
            let (curr_weight, current) = pqueue.pop().unwrap().0;
            
            // Return the minimum distance if the end is found
            if current == end {
                return Some(curr_weight);
            }
            
            visited[current] = true;
            
            // Check each node connected to the current one
            for (next, weight) in edges[current].iter() {
                // Only check nodes that are yet to be visited
                if !visited[*next] {
                    pqueue.push(Reverse((*weight + curr_weight, *next)));
                    predecessors[*next] = Some(current);
                }
            }
            
        }
        
        // Return None if no path to the end was found
        return None;
    }
    
    
}
