use std::{collections::HashMap, fs};

use itertools::Itertools;

const PATH: &str = "src/year2015/day13/data.raw";

#[derive(Debug, Clone)]
struct SeatingImpact {
    name: String,
    next_to: String,
    impact: i32,
}

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn parse() -> Vec<SeatingImpact> {
    let result = read_file();
    let mut links: Vec<SeatingImpact> = vec![];
    for line in (&result).split("\n").into_iter() {
        let buffer = line
            .replace(" would", "")
            .replace("gain", "")
            .replace("lose ", "-")
            .replace(" happiness units by sitting next to ", " ")
            .replace(".", "")
            .replace("  ", " ");

        let pieces: Vec<&str> = buffer.split(" ").into_iter().collect();
        links.push(SeatingImpact {
            name: pieces.get(0).unwrap().to_string(),
            next_to: pieces.get(2).unwrap().to_string(),
            impact: pieces.get(1).unwrap().parse().unwrap(),
        });
        println!("{:?}", (&links).last().unwrap());
    }
    return links;
}

fn organize(relationships: Vec<SeatingImpact>) -> HashMap<String, HashMap<String, i32>> {
    let mut hash_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for relationships in &relationships {
        // First level - insert empty mapping
        if !(&hash_map).contains_key(&relationships.name) {
            (&mut hash_map)
                .entry(relationships.name.clone())
                .or_insert(HashMap::new());
        }
        // Second level
        let inner = (&mut hash_map).get_mut(&relationships.name).unwrap();
        inner
            .entry(relationships.next_to.clone())
            .or_insert(relationships.impact);
    }

    return hash_map;
}

fn get_seat_score(
    relationships: &HashMap<String, HashMap<String, i32>>,
    a: &String,
    b: &String,
) -> i32 {
    return match relationships.get_key_value(a) {
        Some(a) => match a.1.get_key_value(b) {
            Some(b) => b.1.clone(),
            None => 0,
        },
        None => 0,
    };
}

fn get_seat_arrangement_score(
    relationships: &HashMap<String, HashMap<String, i32>>,
    seating: &Vec<&&String>,
    bail_score: i32,
) -> i32 {
    // Get score between first & last element
    let mut buffer = get_seat_score(
        &relationships,
        seating.first().unwrap(),
        seating.last().unwrap(),
    ) + get_seat_score(
        &relationships,
        seating.last().unwrap(),
        seating.first().unwrap(),
    );

    // Process all of the other links
    for index in 1..seating.len() {
        buffer = buffer
            + get_seat_score(
                &relationships,
                seating.get(index - 1).unwrap(),
                seating.get(index).unwrap(),
            )
            + get_seat_score(
                &relationships,
                seating.get(index).unwrap(),
                seating.get(index - 1).unwrap(),
            );
    }

    return buffer;
}

fn part1() -> usize {
    let relationships = organize(parse());

    let guests: Vec<&String> = relationships.keys().collect();

    println!("{:?}", relationships);
    println!("{:?}", guests);
    let mut best: Option<Vec<&&String>> = None;
    let mut bestScore = i32::MIN;
    for permutation in guests.iter().permutations(guests.len()).unique() {
        let score = get_seat_arrangement_score(&relationships, &permutation, 0);
        if bestScore < score {
            bestScore = score;
            best = Some(permutation.clone());
        }
        println!("{:?} - {:?}", permutation, score);
    }
    let best = best.unwrap();

    println!("-----------");
    println!("Best seating: {:?}", best);
    println!("Best score: {:?}", bestScore);
    for index in 1..best.len() {
        let a = best.get(index - 1).unwrap();
        let b = best.get(index).unwrap();
        println!(
            "{:?} -> {:?} = {:?}",
            a,
            b,
            get_seat_score(&relationships, a, b)
        );
    }
    return relationships.len();
}

fn part2() -> i32 {
    return -1;
}

pub fn puzzle() {
    let result = part1();
    println!("Part1: {:?}", result);
    let result = part2();
    println!("Part2: {:?}", result);
}
