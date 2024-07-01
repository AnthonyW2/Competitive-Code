pub mod day23 {
    
    use std::collections::HashMap;
    
    pub fn solution(lines: Vec<String>) {
        
        // Planned solution: Every possible state of the burrow is contained in a directional, weighted graph, where adjacent states are connected by edges.
        // The weight of an edge is the energy require to move between states.
        // Only some edges are directional (moving out of or into a side room).
        // Use a pathfinding algorithm (such as BFS, Djikstra's, or A*) to find the shortest route from the initial state to the final state.
        // Use a hashmap (where the state is the key & energy is the value) to store the seen states.
        
        // ==== Part 1 ==== //
        
        let type_map = HashMap::from([
            ('A', 0),
            ('B', 1),
            ('C', 2),
            ('D', 3),
        ]);
        let energy_map = [1, 10, 100, 1000];
        
        // (type, x, y)
        // The hallway stretches from (0,0) to (10,0)
        let mut starting_positions: Vec<(usize, usize, usize)> = Vec::new();
        for (y, s) in (&lines[2..=3]).iter().enumerate() {
            for (x, c) in s.chars().enumerate() {
                if type_map.contains_key(&c) {
                    starting_positions.push((type_map[&c], x-1, y+1));
                }
            }
        }
        
        println!("{:?}", starting_positions);
        
        fn get_adjacent_configurations(positions: &Vec<(usize, usize, usize)>) -> Vec<Vec<(usize, usize, usize)>> {
            
            let mut configurations = Vec::new();
            
            for amphipod in positions {
                
                let mut can_move_to_hallway = false;
                let mut can_move_to_room = false;
                
                // Check y-coordinate
                if amphipod.2 == 0 {
                    // In the hallway - can move into its destination room
                    can_move_to_hallway = true;
                } else if amphipod.2 == 1 {
                    // In the entrance to a room
                    if amphipod.1/2-1 != amphipod.0 {
                        // In the wrong room - can move out
                        can_move_to_room = true;
                    } else {
                        let amphipod_behind = positions.iter().find(|a| a.1 == amphipod.1 && a.2 == 2);
                        if amphipod_behind.is_none() {
                            panic!("Amphipod is in the front of a room with no partner");
                        } else if amphipod_behind.unwrap().0 != amphipod.0 {
                            // Different type of amphipod behind - can move out
                            can_move_to_room = true;
                        }
                    }
                } else if amphipod.2 == 2 && amphipod.1/2-1 != amphipod.0 {
                    // At the back of the wrong room
                    let amphipod_in_front = positions.iter().find(|a| a.1 == amphipod.1 && a.2 == 1);
                    if amphipod_in_front.is_none() {
                        // No amphipod in front - can move out
                        can_move_to_hallway = true;
                    }
                }
                
                if can_move_to_hallway {
                    // Can move out of a room, stopping in the hallway or in its destination room
                    
                } else if can_move_to_room {
                    // Can move to its destination room
                    
                }
                
            }
            
            return configurations;
            
        }
        
        let mut amphipod_locations: Vec<usize> = Vec::new();
        
        println!("Part 1: {}", "_");
        
        
        // ==== Part 2 ==== //
        
        println!("Part 2: {}", "_");
        
    }
    
}
