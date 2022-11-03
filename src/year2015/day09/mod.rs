use std::{collections::HashMap, fs, ops::Index};

const PATH: &str = "src/year2015/day09/data.raw";

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

#[derive(Debug, Clone)]
struct Link {
    from: String,
    to: String,
    distance: u32,
}

fn parse() -> Vec<Link> {
    let mut result: Vec<Link> = vec![];
    let data = read_file();

    for pieces in (&data).split("\n").map(|line| line.replace(" to ", " = ")) {
        let pieces: Vec<&str> = pieces.split(" = ").collect();

        result.push(Link {
            from: pieces[0].to_string(),
            to: pieces[1].to_string(),
            distance: (pieces[2]).parse().unwrap(),
        });
    }

    return result;
}

fn get_nodes_and_links(links: &Vec<Link>) -> HashMap<String, Vec<Link>> {
    // Quick look up tables for from linking
    let mut from_lookup: HashMap<String, Vec<Link>> = HashMap::new();

    for link in links {
        // Add missing entry from
        if !from_lookup.contains_key(&link.from) {
            from_lookup.insert(
                link.from.clone(),
                links
                    .clone()
                    .into_iter()
                    .filter(|other_link| other_link.from.eq(&link.from))
                    .collect(),
            );
        }
    }
    return from_lookup;
}

fn get_node_names(links: &Vec<Link>) -> Vec<String> {
    let mut unique_names: Vec<String> = vec![];

    for link in links {
        // Unique names
        if !unique_names.contains(&link.from) {
            unique_names.push(link.from.clone())
        }
        if !unique_names.contains(&link.to) {
            unique_names.push(link.to.clone())
        }
    }
    return unique_names;
}

fn get_distance(a: &Vec<Link>) -> u32 {
    if a.len() == 0 {
        return 0;
    }
    if a.len() == 1 {
        return a.first().unwrap().distance;
    }

    return a
        .into_iter()
        .map(|a| a.distance)
        .reduce(|a, b| a + b)
        .unwrap_or(0);
}

fn propagate(
    from_lookup: &HashMap<String, Vec<Link>>,
    unique_names: &Vec<String>,
    previous_locations: Vec<Link>,
    depth: u32,
) -> Option<Vec<Link>> {
    println!("----------");
    println!("[{:?}] Start", depth);
    let previous_locations: Vec<Link> = previous_locations.clone();
    let mut previous_location_names: Vec<String> = vec![];

    //
    let current_location = (&previous_locations).last().unwrap().to.clone();
    println!("[{:?}] Current location: {:?}", depth, current_location);

    //
    println!("[{:?}] Path so far: ", depth);
    for previous_link in (&previous_locations).into_iter() {
        println!(
            "[{:?}] - {:?} -> {:?}",
            depth, previous_link.from, previous_link.to
        );
        if !previous_location_names.contains(&previous_link.from) {
            previous_location_names.push(previous_link.from.clone());
        }
        if !previous_location_names.contains(&previous_link.to) {
            previous_location_names.push(previous_link.to.clone());
        }
    }
    let available_links = (&from_lookup).get(&current_location);
    if available_links.is_none() {
        println!("[{:?}] No travel options available: ", depth);
        // Checking if we have reached our final goal
        if previous_location_names.len() == unique_names.len() {
            println!("[{:?}] We have visited all of the places!", depth);
            return Some(previous_locations.clone());
        }
        return None;
    }

    //
    println!("[{:?}] Available options: ", depth);
    for available_link in (&available_links).unwrap().into_iter() {
        if !previous_location_names.contains(&available_link.to) {
            println!(
                "[{:?}] --- {:?} -> {:?}",
                depth, available_link.from, available_link.to
            );
        }
    }

    // Iterate through available links
    println!("[{:?}] Iterating through links: ", depth);
    let mut continued_journeys: Vec<Vec<Link>> = vec![];

    for available_link in (&available_links).unwrap().into_iter() {
        if !previous_location_names.contains(&available_link.to) {
            println!(
                "[{:?}] --- {:?} -> {:?}",
                depth, available_link.from, available_link.to
            );
            let mut continued_journey = previous_locations.clone();
            continued_journey.push(available_link.clone());
            let result = propagate(from_lookup, unique_names, continued_journey, depth + 1);

            if result.is_some() {
                continued_journeys.push(result.unwrap().clone());
            }
        } else {
            println!("[{:?}] --- Already visited {:?}", depth, available_link.to);
        }
    }

    if continued_journeys.len() > 0 {
        // Only 1 result - return this
        if continued_journeys.len() == 1 {
            return Some(continued_journeys.first().unwrap().clone());
        }

        // Pick best result out of the multi choice
        return Some(
            (&continued_journeys)
                .into_iter()
                .reduce(|a, b| {
                    if get_distance(a) < get_distance(b) {
                        return a;
                    }
                    return b;
                })
                .unwrap()
                .clone(),
        );
    }

    println!("No subpaths returned valid results");
    return None;
}

fn part1() -> usize {
    let results = parse();
    let from_lookup = get_nodes_and_links(&results);
    let unique_names = get_node_names(&results);

    // Print processed data
    for (key, value) in (&from_lookup).into_iter() {
        println!("[{:?}] - {:?}", key, value);
        println!("----");
    }

    println!("----");
    // Quasi A* Path finding?
    let mut routes_found: Vec<Vec<Link>> = vec![];
    let mut loop_index = 0;
    for link in (&results) {
        println!("----------");
        println!("Tick {:?}", loop_index);
        loop_index = loop_index + 1;
        let result = propagate(
            &from_lookup.clone(),
            &unique_names.clone(),
            vec![link.clone()],
            0,
        );
        if result.is_some() {
            routes_found.push(result.unwrap().clone());
        }
    }

    println!("----");
    println!("----");
    println!("Valid paths found: {:?}", routes_found.len());
    for route in (&routes_found).into_iter() {
        println!("-- Route - Distance : {:?}", get_distance(&route));
        for link in (&route).into_iter() {
            print!("{:?} -> {:?}, ", link.from, link.to);
        }
    }
    println!("----");
    println!("Unique names to visit: {:?}", unique_names);

    return results.len();
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
