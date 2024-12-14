pub mod day14 {
    
    pub fn solution(lines: Vec<String>) {
        
        // ==== Part 1 ==== //
        
        const BOUND_X: isize = 101;
        const BOUND_Y: isize = 103;
        
        let mut initial_robots: Vec<((isize, isize), (isize, isize))> = Vec::new();
        
        for line in &lines {
            let split = line.split(" ")
                .map(
                    |s| s.split("=")
                        .nth(1)
                        .unwrap()
                        .split(",")
                        .map(|s| s.parse::<isize>().unwrap())
                        .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();
            
                initial_robots.push(((split[0][0], split[0][1]),(split[1][0], split[1][1])));
        }
        
        fn simulate_robots(robots: &mut Vec<((isize, isize), (isize, isize))>) {
            for r in 0..robots.len() {
                robots[r].0 = move_robot(robots[r].0, robots[r].1);
            }
        }
        
        fn move_robot((x, y): (isize, isize), (vx, vy): (isize, isize)) -> (isize, isize) {
            let mut new_x = (x + vx) % BOUND_X;
            let mut new_y = (y + vy) % BOUND_Y;
            if new_x < 0 {
                new_x += BOUND_X;
            }
            if new_y < 0 {
                new_y += BOUND_Y;
            }
            return (new_x, new_y);
        }
        
        let mut robots1 = initial_robots.clone();
        
        for _ in 0..100 {
            simulate_robots(&mut robots1);
        }
        
        let quadrants = [
            ((0,0),(BOUND_X/2,BOUND_Y/2)),
            ((BOUND_X/2+1,0),(BOUND_X,BOUND_Y/2)),
            ((0,BOUND_Y/2+1),(BOUND_X/2,BOUND_Y)),
            ((BOUND_X/2+1,BOUND_Y/2+1),(BOUND_X,BOUND_Y)),
        ];
        
        let mut quadrant_counts = Vec::new();
        
        for ((x1,y1),(x2,y2)) in quadrants {
            let mut count = 0;
            for &((x,y),_) in &robots1 {
                if x1 <= x && x < x2 && y1 <= y && y < y2 {
                    count += 1
                }
            }
            quadrant_counts.push(count);
        }
        
        println!("Part 1: {}", quadrant_counts.iter().product::<isize>());
        
        
        // ==== Part 2 ==== //
        
        fn check_for_image(robots: &Vec<((isize, isize), (isize, isize))>) -> bool {
            let mut map = vec![vec!['.'; BOUND_X as usize]; BOUND_Y as usize];
            let offsets = [(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)];
            for &((x,y),_) in robots {
                map[y as usize][x as usize] = '#';
                let mut tree_present = true;
                for (xd,yd) in offsets {
                    let x_check = x+xd;
                    let y_check = y+yd;
                    if !(
                        x_check >= 0 &&
                        y_check >= 0 &&
                        (x_check as usize) < map[0].len() &&
                        (y_check as usize) < map.len() &&
                        map[y_check as usize][x_check as usize] == '#'
                    ) {
                        tree_present = false;
                    }
                }
                if tree_present {
                    return true;
                }
            }
            return false;
        }
        
        let mut robots2 = initial_robots.clone();
        
        for i in 0..100000 {
            simulate_robots(&mut robots2);
            if check_for_image(&robots2) {
                println!("Part 2: {}", i+1);
                break;
            }
        }
        
    }
    
    /*
    fn render_map(robots: &Vec<((isize, isize), (isize, isize))>) {
        let mut map = vec![vec!['.'; BOUND_X as usize]; BOUND_Y as usize];
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for &((x,y),_) in robots {
            map[y as usize][x as usize] = '#';
            xs.push(x);
            ys.push(y);
        }
        let x_stddev = std_deviation(&xs).unwrap();
        let y_stddev = std_deviation(&xs).unwrap();
        //println!("stddev: {} {}", x_stddev, y_stddev);
        if x_stddev < 20.0 && y_stddev < 20.0 {
            println!("{}", map.into_iter().map(|line| line.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
            let delay = time::Duration::from_millis(200);
            thread::sleep(delay);
        }
    }
    */
    
}
