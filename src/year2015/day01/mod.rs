use std::fs;

const PATH: &str = "src/year2015/day01/data.raw";

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn part1() -> i32 {
    let contents = read_file();

    let mut counter = 0;

    for char in contents.chars().into_iter() {
        counter = counter + if char.to_string() == "(" { 1 } else { -1 }
    }

    return counter;
}

fn part2() -> usize {
    let contents = read_file();

    let mut counter: i32 = 0;

    for (index, char) in contents.chars().into_iter().enumerate() {
        counter = counter + if char.to_string() == "(" { 1 } else { -1 };

        if counter == -1 {
            return index + 1;
        }
    }

    return 0;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
