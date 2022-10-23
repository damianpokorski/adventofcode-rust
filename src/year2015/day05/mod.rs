use std::{fs};

const PATH: &str = "src/year2015/day05/data.raw";


fn read_file() -> String {
  println!("Reading a file: {PATH}");
  return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn part1() -> (i32) {
  return -1
}

fn part2() -> (i32) {
  return -1
}

pub fn puzzle() {
  let result = part1();
  println!("Part1: {:?}", result);
  let result = part2();
  println!("Part2: {:?}", result);
}