use std::{fs, cmp::min};

const PATH: &str = "src/year2015/day02/data.raw";

#[derive(Debug)]
struct Row {
  x: i32,
  y: i32,
  z: i32
}

fn read_file() -> String {
  println!("Reading a file: {PATH}");
  return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn read_file_structs() -> Vec<Row> {

  let contents = read_file();
  let mut result:Vec<Row>  = Vec::new();

  for line in contents.split_ascii_whitespace().into_iter() {
    let v: Vec<i32> = line
      .split("x")
      .map(|x| x
          .parse::<i32>()
          .unwrap())
      .collect();

    result.push(Row {
      x: v.get(0).unwrap().clone(),
      y: v.get(1).unwrap().clone(),
      z: v.get(2).unwrap().clone()
    });
  }

  result
}

fn part1() -> i32 {
  let mut total_surface_required = 0;
  for row in read_file_structs().into_iter() {
    let xy = row.x * row.y;
    let yz = row.y * row.z;
    let zx = row.z * row.x;
    total_surface_required += ((xy + yz + zx)*2) + (min(min(xy, yz), zx));
  }
  return total_surface_required;
}

fn part2() -> i32 {
  let mut total_ribbon_required = 0;
  for row in read_file_structs().into_iter() {
    let mut list = vec![row.x, row.y, row.z];
    list.sort();

    let ribbon = ((list.get(0).unwrap() + list.get(1).unwrap())*2) + (row.x * row.y * row.z);

    total_ribbon_required += ribbon;
  }
  return total_ribbon_required;
}

pub fn puzzle() {
  let result = part1();
  println!("Part1: {result}");
  let result = part2();
  println!("Part2: {result}");
}