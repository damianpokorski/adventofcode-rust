use std::{fs, collections::HashMap};

const PATH: &str = "src/year2015/day03/data.raw";


fn read_file() -> String {
  println!("Reading a file: {PATH}");
  return fs::read_to_string(PATH).expect("Should be able to read the file");
}

struct Vector2 {
  x: i32,
  y: i32
}

fn part1() -> usize {
  let mut position = Vector2 {
    x: 0,
    y: 0
  };

  let mut presents: HashMap<(i32, i32), i32> = HashMap::new();
  presents.insert((position.x, position.y), 0);

  
  for line in read_file()
      .split("")
      .into_iter() {
    
    match line {
      "^" => position.y += 1,
      "v" => position.y -= 1,
      ">" => position.x += 1,
      "<" => position.x -= 1,
      _ => println!("invalid character xx")
    }

    presents.entry((position.x, position.y)).and_modify(|present| *present += 1).or_insert(1);
  }
  
  return presents.keys().count()
}

fn part2() -> usize {
  let mut santa = Vector2 {
    x: 0,
    y: 0
  };
  let mut robo_santa = Vector2 {
    x: 0,
    y: 0
  };

  let mut presents: HashMap<(i32, i32), i32> = HashMap::new();
  presents.insert((0, 0), 0);

  let mut turn = false;
  for line in read_file()
      .split("")
      .into_iter() {
    turn = !turn;
    match line {
      "^" => if turn {santa.y += 1} else {robo_santa.y += 1},
      "v" => if turn {santa.y -= 1} else {robo_santa.y -= 1},
      ">" => if turn {santa.x += 1} else {robo_santa.x += 1},
      "<" => if turn {santa.x -= 1} else {robo_santa.x -= 1},
      _ => println!("invalid character y {:?}", line)
    }

    presents
      .entry(if turn {(santa.x, santa.y)} else {(robo_santa.x, robo_santa.y)})
      .and_modify(|present| *present += 1)
      .or_insert(1);
  }
  
  return presents.keys().count()
}

pub fn puzzle() {
  let result = part1();
  println!("Part1: {result}");
  let result = part2();
  println!("Part2: {result}");
}