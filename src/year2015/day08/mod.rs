use std::fs;

use fancy_regex::Regex;

const PATH: &str = "src/year2015/day08/data.raw";

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn part1() -> i32 {
    let quote_mark_stripper = Regex::new(r#"^\"(.*)\"$"#).unwrap();
    let hexadecimal_replacer = Regex::new(r#"(\\x[a-f0-9]{2})"#).unwrap();
    let double_escape = Regex::new(r#"\\(.)"#).unwrap();

    let mut sum: i32 = 0;
    for line in read_file().split("\n").into_iter() {
        println!("{:?}", line);
        let question_marks = &quote_mark_stripper.replace_all(&line, "$1").into_owned();
        let hexes = &hexadecimal_replacer
            .replace_all(question_marks, "|")
            .into_owned();
        let backslashes = &double_escape.replace_all(&hexes, "$1").into_owned();

        println!("{:?}", question_marks);
        println!("{:?}", hexes);
        println!("{:?}", backslashes);
        println!(
            "Original {:?} - Cleared {:?}",
            line.len(),
            backslashes.len()
        );

        sum += (line.len() as i32) - (backslashes.len() as i32);
        println!("---");
    }
    return sum;
}

fn part2() -> i32 {
    let quote_mark_finder = Regex::new(r#"^(\")(.*)(\")$"#).unwrap();
    let hexadecimal_adder = Regex::new(r#"(\\x[a-f0-9]{2})"#).unwrap();
    let double_escape = Regex::new(r#"(\\)(.)"#).unwrap();

    let mut sum: i32 = 0;
    for line in read_file().split("\n").into_iter() {
        println!("{:?}", line);
        let question_marks = &quote_mark_finder
            .replace_all(&line, "$1$1$2$3$3")
            .into_owned();
        let hexes = &hexadecimal_adder
            .replace_all(question_marks, "\\$1")
            .into_owned();
        let backslashes = &double_escape.replace_all(&hexes, "$1$1$2").into_owned();

        println!("1: {:?}", question_marks);
        println!("2: {:?}", hexes);
        println!("3: {:?}", backslashes);
        println!(
            "Original {:?} - Cleared {:?}",
            line.len(),
            backslashes.len()
        );

        sum += (backslashes.len() as i32) - (line.len() as i32);
        println!("---");
    }
    return sum;
}

pub fn puzzle() {
    let result = part1();
    println!("Part1: {:?}", result);
    let result = part2();
    println!("Part2: {:?}", result);
}
