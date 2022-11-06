use crate::common::puzzle_data;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SeatingImpact {
    name: String,
    next_to: String,
    impact: i32,
}

fn parse() -> Vec<SeatingImpact> {
    let result = puzzle_data(std::file!());
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

fn find_best_seat_arrangement(
    relationships: &HashMap<String, HashMap<String, i32>>,
    guests: &Vec<&String>,
) -> (Vec<String>, i32) {
    println!("{:?}", relationships);
    println!("{:?}", guests);
    let mut best: Option<Vec<&&String>> = None;
    let mut best_score = i32::MIN;
    for permutation in guests.iter().permutations(guests.len()).unique() {
        let score = get_seat_arrangement_score(&relationships, &permutation);
        if best_score < score {
            best_score = score;
            best = Some(permutation.clone());
            // New best
            println!("New best seating: {:?} = {:?}", permutation, score);
        }
    }
    let best: Vec<String> = best
        .unwrap()
        .into_iter()
        .map(|string_reference| (*string_reference).clone())
        .collect();
    return (best, best_score);
}
fn print_results(
    relationships: &HashMap<String, HashMap<String, i32>>,
    seating: &Vec<String>,
    score: i32,
) {
    println!("-----------");
    println!("Best seating: {:?}", seating);
    println!("Best score: {:?}", score);

    let first = seating.first().unwrap();
    let last = seating.last().unwrap();
    let first_to_last = get_seat_score(&relationships, first, last);
    let last_to_first = get_seat_score(&relationships, last, first);
    println!(
        "{:?} -> {:?} = {:?}, {:?} -> {:?} = {:?} | {:?}",
        first,
        last,
        first_to_last,
        last,
        first,
        last_to_first,
        first_to_last + last_to_first
    );

    for index in 1..seating.len() {
        let a = seating.get(index - 1).unwrap();
        let b = seating.get(index).unwrap();
        println!(
            "{:?} -> {:?} = {:?}, {:?} -> {:?} = {:?} | {:?}",
            a,
            b,
            get_seat_score(&relationships, a, b),
            b,
            a,
            get_seat_score(&relationships, b, a),
            get_seat_score(&relationships, a, b) + get_seat_score(&relationships, b, a)
        );
    }
}
fn part1() -> i32 {
    let relationships = organize(parse());

    let guests: Vec<&String> = relationships.keys().collect();
    let (best, score) = find_best_seat_arrangement(&relationships, &guests);
    print_results(&relationships, &best, score);
    return score;
}

fn part2() -> i32 {
    let relationships = organize(parse());
    let original_guests: Vec<&String> = relationships.keys().collect();
    let mut relationships = relationships.clone();

    // Add yourself as an ambivalent guest
    let me = String::from("me");
    for guest in original_guests {
        if !relationships.keys().contains(&(&me).clone()) {
            relationships.entry((&me).clone()).or_insert(HashMap::new());
        }
        // Me -> Guest
        relationships
            .get_mut(&me)
            .unwrap()
            .entry(guest.clone())
            .or_insert(0);
        // Guest -> Me
        relationships
            .get_mut(&guest.clone())
            .unwrap()
            .entry(me.clone())
            .or_insert(0);
    }

    let guests: Vec<&String> = (relationships).keys().collect();
    let (best, score) = find_best_seat_arrangement(&relationships, &guests);
    print_results(&relationships, &best, score);
    return score;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
