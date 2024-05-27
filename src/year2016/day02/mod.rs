use crate::common::puzzle_data;
use crate::common::vectors::Vector2d;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Key {
    value: String,
}

fn part1keymap() -> Vec<(String, Vector2d)> {
    return vec![
        // 1,2,3
        ("1".to_owned(), Vector2d { x: 0, y: 2 }),
        ("2".to_owned(), Vector2d { x: 1, y: 2 }),
        ("3".to_owned(), Vector2d { x: 2, y: 2 }),
        // 4,5,6
        ("4".to_owned(), Vector2d { x: 0, y: 1 }),
        ("5".to_owned(), Vector2d { x: 1, y: 1 }),
        ("6".to_owned(), Vector2d { x: 2, y: 1 }),
        // 7,8,9
        ("7".to_owned(), Vector2d { x: 0, y: 0 }),
        ("8".to_owned(), Vector2d { x: 1, y: 0 }),
        ("9".to_owned(), Vector2d { x: 2, y: 0 }),
    ];
}
fn part2keymap() -> Vec<(String, Vector2d)> {
    return vec![
        // 1
        ("1".to_owned(), Vector2d { x: 2, y: 4 }),
        // 2,3,4
        ("2".to_owned(), Vector2d { x: 1, y: 3 }),
        ("3".to_owned(), Vector2d { x: 2, y: 3 }),
        ("4".to_owned(), Vector2d { x: 3, y: 3 }),
        // 5,6,7,8,9
        ("5".to_owned(), Vector2d { x: 0, y: 2 }),
        ("6".to_owned(), Vector2d { x: 1, y: 2 }),
        ("7".to_owned(), Vector2d { x: 2, y: 2 }),
        ("8".to_owned(), Vector2d { x: 3, y: 2 }),
        ("9".to_owned(), Vector2d { x: 4, y: 2 }),
        // 2,3,4
        ("A".to_owned(), Vector2d { x: 1, y: 1 }),
        ("B".to_owned(), Vector2d { x: 2, y: 1 }),
        ("C".to_owned(), Vector2d { x: 3, y: 1 }),
        // 2,3,4
        ("D".to_owned(), Vector2d { x: 2, y: 0 }),
    ];
}

fn solve(keymap: &Vec<(String, Vector2d)>) -> String {
    // Key maps
    let mut keys_map: HashMap<String, Key> = HashMap::new();

    // Build a keypad and save it into hashmap
    keymap.into_iter().for_each(|(value, vector)| {
        keys_map.insert(
            vector.id().clone(),
            Key {
                value: value.to_string(),
            },
        );
    });

    // Set up some variables
    let mut position = Vector2d { x: 1, y: 1 };

    let contents = puzzle_data(std::file!());
    let mut digits = String::new();

    for line in contents.split("\n").into_iter() {
        for character in line.chars().into_iter() {
            let direction = match character {
                'U' => Vector2d::up(),
                'D' => Vector2d::down(),
                'L' => Vector2d::left(),
                'R' => Vector2d::right(),
                _ => Vector2d::zero(),
            };
            let new_position = position.add(&direction);
            let valid_position = keys_map.get(&new_position.id());
            // Only move to valid positions
            if valid_position.is_some() {
                position = new_position;
            }
        }

        // Convert position back to digit
        let position_id = position.id();
        let digit = match keys_map.get(&position_id) {
            Some(key) => key.value.to_string(),
            None => "".to_string(),
        };
        digits.push_str(&digit.to_string());
    }
    return digits;
}

pub fn puzzle() {
    println!("Part1: {:?}", solve(&part1keymap()));
    println!("Part2: {:?}", solve(&part2keymap()));
}
