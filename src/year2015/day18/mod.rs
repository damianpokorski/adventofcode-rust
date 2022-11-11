use itertools::Itertools;

use crate::common::puzzle_data;

fn ingest() -> Vec<Vec<bool>> {
    puzzle_data(std::file!())
        .split_ascii_whitespace()
        .into_iter()
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|character| character == '#')
                .collect_vec()
        })
        .collect_vec()
}

fn simulate(cycles: i32, corners_always_on: bool) -> usize {
    let empty_row: Vec<bool> = vec![];

    let initial_state = ingest();

    let mut current_state = initial_state.clone();
    println!("Initial state");
    println!("-------------");
    for row in (&current_state).into_iter() {
        for column in (&row).into_iter() {
            print!("{}", if column == &true { '#' } else { '.' })
        }
        println!("");
    }

    let last_index = (current_state.len() - 1) as i32;

    println!("-------------");
    for cycle in 0..cycles {
        print!("{}[2J", 27 as char);
        println!("Cycle #{}", cycle + 1);
        println!("-------------");
        let mut new_state = current_state
            .clone()
            .into_iter()
            .map(|row| row.into_iter().map(|_cell| false).collect_vec())
            .collect_vec();

        for y in 0..current_state.len() {
            for x in 0..(&current_state).len() {
                let mut current_light = (&current_state).get(y).unwrap().get(x).unwrap().clone();

                let relative: Vec<(i32, i32)> = vec![
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];

                let neighbour_lights_lit = relative
                    .into_iter()
                    .map(|tuple| ((tuple.0 + (y as i32)), (tuple.1 + (x as i32))))
                    .map(|position| {
                        if position.0 < 0 || position.1 < 0 {
                            return false;
                        }

                        // Corners always on in part 2
                        if corners_always_on {
                            if (position.0 == 0 && position.1 == 0)
                                || (position.0 == 0 && position.1 == last_index)
                                || (position.0 == last_index && position.1 == 0)
                                || (position.0 == last_index && position.1 == last_index)
                            {
                                return true;
                            }
                        }

                        return (&current_state)
                            .get(position.0 as usize)
                            .unwrap_or(&empty_row.clone())
                            .get(position.1 as usize)
                            .unwrap_or(&false)
                            .clone();
                    })
                    .filter(|state| state == &true)
                    .count();

                if current_light {
                    if neighbour_lights_lit != 2 && neighbour_lights_lit != 3 {
                        current_light = false;
                    }
                } else {
                    if neighbour_lights_lit == 3 {
                        current_light = true;
                    }
                }

                if corners_always_on {
                    if (y == 0 && x == 0)
                        || (y == 0 && x == last_index as usize)
                        || (y == last_index as usize && x == 0)
                        || (y == last_index as usize && x == last_index as usize)
                    {
                        current_light = true;
                    }
                }

                new_state[y][x] = current_light;
            }
        }

        current_state = new_state;
        for row in (&current_state).into_iter() {
            for column in (&row).into_iter() {
                print!("{}", if column == &true { '#' } else { '.' })
            }
            println!("");
        }
    }

    return current_state
        .into_iter()
        .flatten()
        .filter(|light| light == &true)
        .count();
}

fn part1() -> usize {
    return simulate(100, false);
}
fn part2() -> usize {
    return simulate(100, true);
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
