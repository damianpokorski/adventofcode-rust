use crate::common::puzzle_data;

fn process(contents: String) -> String {
    let characters = contents.chars();

    let mut counters: Vec<(char, i32)> = vec![];
    let mut last_counter: Option<(char, i32)> = None;

    for character in characters.into_iter() {
        // First pass
        if last_counter.is_none() {
            last_counter = Some((character.clone(), 1));
            counters.push(last_counter.unwrap());

        // nth loop - matching counter
        } else {
            // Increment counter
            if last_counter.unwrap().0.eq(&character) {
                counters.pop();
                last_counter = Some((last_counter.unwrap().0, last_counter.unwrap().1 + 1));
                counters.push(last_counter.unwrap());
            } else {
                // Create a new counter
                last_counter = Some((character, 1));
                counters.push(last_counter.unwrap());
            }
        }
    }

    let mut result = String::from("");
    for (character, counter) in counters {
        result.push_str(counter.to_string().as_str());
        result.push_str(character.to_string().as_str());
    }
    return result;
}

fn part1() -> usize {
    let contents = puzzle_data(std::file!());

    let mut counter = 0;
    let mut buffer = process(contents);
    for _i in 0..39 {
        counter = counter + 1;
        buffer = process(buffer);
    }
    return buffer.len();
}

fn part2() -> usize {
    let contents = puzzle_data(std::file!());

    let mut counter = 0;
    let mut buffer = process(contents);
    for _i in 0..49 {
        counter = counter + 1;
        buffer = process(buffer);
    }
    return buffer.len();
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
