use std::{collections::HashMap, fs};

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

fn propagate(
    from_lookup: &HashMap<String, Vec<Link>>,
    unique_names: &Vec<String>,
    previous_locations: Vec<Link>,
    current_distance: u32,
    max_distance: u32,
) -> Option<(Vec<Link>, u32)> {
    println!("Unique names: {:?}", unique_names);

    let mut previous_locations: Vec<Link> = previous_locations.clone();

    let mut visited_location_names: Vec<String> = (&previous_locations)
        .into_iter()
        .map(|location| vec![location.from.clone(), location.to.clone()])
        .into_iter()
        .flatten()
        .collect();

    // Add last stop to the visisted list
    let current_stop_name = &previous_locations.last().unwrap().to;
    visited_location_names.push(current_stop_name.clone());

    println!("Visited locations: {:?}", visited_location_names);
    println!("Current stop: {:?}", current_stop_name);

    // Get available
    let available_stops = (&from_lookup).get(current_stop_name);

    if available_stops.is_none() {
        return None;
    }
    let available_stops = available_stops.unwrap();
    let targets: Vec<&Link> = (available_stops)
        .into_iter()
        .filter(|link| {
            (&visited_location_names)
                .into_iter()
                .any(|visited_location| visited_location != &link.from)
        })
        .collect();

    let mut sub_paths: Vec<Option<(Vec<Link>, u32)>> = vec![];
    let available_location_names: Vec<String> =
        (&targets).into_iter().map(|link| link.to.clone()).collect();

    println!("Available locations raw raw: {:?}", available_stops);
    println!("Available locations raw: {:?}", targets);
    println!("Available locations: {:?}", available_location_names);

    for link in targets {
        // Add link to the taken locations
        let mut new_previous_locations = previous_locations.clone();
        new_previous_locations.push(link.clone());

        // Check if we reached all destinations - this one is a win
        if !(&unique_names).into_iter().any(|location| {
            !(&new_previous_locations)
                .into_iter()
                .any(|link| &link.to == location || &link.from == location)
        }) {
            println!("Victory!");
            return Some((new_previous_locations, current_distance + link.distance));
        }

        // Check if we reached the max distance - stop propagating
        if (current_distance + link.distance) > max_distance {
            println!("End path!");
            return None;
        }

        // Keep on digging
        println!("Digging!");
        sub_paths.push(propagate(
            from_lookup,
            unique_names,
            new_previous_locations.clone(),
            (current_distance + link.distance),
            max_distance,
        ));
    }

    // Filter invalid paths out
    let mut sub_paths: Vec<(Vec<Link>, u32)> = sub_paths
        .into_iter()
        .filter(|path| path.is_some())
        .map(|path| path.unwrap())
        .collect();

    // Once we're complete - return the shortest path that's complete
    if sub_paths.len() > 0 {
        println!("Sorting!");
        sub_paths.sort_by(|b, a| a.1.cmp(&b.1));
        return Some(sub_paths.first().unwrap().clone());
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
    println!("Unique names: {:?}", unique_names);

    // Find max distance covering all links
    let max_distance = u32::MAX;

    println!("----");
    // Quasi A* Path finding?
    let mut starting_points: Vec<Option<(Vec<Link>, u32)>> = vec![];
    for link in (&results) {
        starting_points.push(propagate(
            &from_lookup,
            &unique_names,
            vec![link.clone()],
            link.distance,
            max_distance,
        ));
        println!("Tick");
    }
    println!("Hmmm {:?}", starting_points);
    // Find best starting point

    // for path in &result.unwrap().0 {
    //     println!("{:?} -> {:?}", path.from, path.to);
    // }

    // Recurse bellow ?
    // -- Two lists
    // unique_names.first().unwrap().clone()

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
