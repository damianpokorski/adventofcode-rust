use crate::common::puzzle_data;

// // Part two tracking

fn solve(is_part2: bool) -> i32 {
    // Path point tracking
    let mut visited: Vec<String> = vec![];

    fn step(v: &mut Vec<String>, x: i32, y: i32) -> bool {
        // Generate unique id & save into vector
        let id = format!("X{} Y{}", x, y);

        // Check if array contains it - if it does that's it
        if v.contains(&id) {
            return true;
        }

        // Save visited spot
        v.push(id);
        return false;
    }

    // Current x,y & direction
    let (mut x,mut y) = (0,0);
    let mut direction: i32 = 0;
    
    // Iteration
    let contents = puzzle_data(std::file!());
    for line in contents.split(", ").into_iter() {
        // Extract entry
        let is_right = line.get(0..1).unwrap() == "R";
        let digit: i32 = line.get(1..).unwrap().parse().unwrap();

        // Move in cycle
        direction = direction + (if is_right { 1 } else { -1 });

        // Cap at 4, keep always positive
        direction = if direction < 0 { direction + 4 } else { direction % 4};

        // 1 & 3 = right & left
        let x_direction = match direction {
            1 => 1,
            3 => -1,
            _ => 0
        };
        
        // 0 & 2 = up & down
        let y_direction = match direction {
            0 => 1,
            2 => -1,
            _ => 0
        };

        // For each step
        for _ in 0..digit {
            x = x + x_direction;
            y = y + y_direction;

            if is_part2 {
                if step(visited.as_mut(), x, y) {
                    return x.abs() + y.abs();
                }
            }
        }
    }

    return x.abs() + y.abs();
}

pub fn puzzle() {
    println!("Part1: {:?}", solve(false));
    println!("Part2: {:?}", solve(true));
}
