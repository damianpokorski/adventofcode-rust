use std::{fs};

const PATH: &str = "src/year2015/day06/data.raw";

fn read_file() -> String {
  println!("Reading a file: {PATH}");
  return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn parse() -> Vec<(usize, usize, usize, usize, Option<bool>)> {
  let mut output:Vec<(usize, usize, usize, usize, Option<bool>)> = Vec::new();
  // Apply input
  for line in read_file().split("\n").map(|x| x ) {
    let coords: Vec<usize> = line
      .replace("turn off ", "")
      .replace("turn on ", "")
      .replace("toggle ", "")
      .replace(" through ", ",")
      .split(",")
      .map(|word| word.parse().unwrap())
      .collect();

    let top_x= *coords.get(0).unwrap();
    let top_y = *coords.get(1).unwrap();
    let bottom_x = *coords.get(2).unwrap();
    let bottom_y = *coords.get(3).unwrap();
  
    let mut state: Option<bool> = None;
    if line.starts_with("turn on") {
      state = Some(true);
    } else if line.starts_with("turn off") {
      state = Some(false);
    }

    output.push((top_x, top_y, bottom_x, bottom_y, state));
  }
  return output;
}

fn part1() -> usize {
  // Create grid
  let mut grid = [[false; 1000]; 1000];

  // Process input
  for line in parse().into_iter() {
    let (top_x, top_y, bottom_x, bottom_y, state) = line;

    for x in top_x..bottom_x+1 {
      for y in top_y..bottom_y+1 {
        match state {
            Some(true) => grid[x][y] = true,
            Some(false) => grid[x][y] = false,
            None => grid[x][y] = !grid[x][y],
        }
      }
    }
  }

  let mut on = 0;
  for x in 0..1000 {
    for y in 0..1000 {
      if grid[x][y] {
        on +=1;
      }
    }
  }
  return on;
}

fn part2() -> i64 {
  // Create grid
  let mut grid = [[0; 1000]; 1000];

  // Process input
  for line in parse().into_iter() {
    let (top_x, top_y, bottom_x, bottom_y, state) = line;
    
    for x in top_x..bottom_x+1 {
      for y in top_y..bottom_y+1 {
        match state {
            Some(true) => grid[x][y] += 1,
            Some(false) => {
              if(grid[x][y] >= 1) {
                grid[x][y] -= 1
              }
            },
            None => grid[x][y] += 2,
        }
      }
    }
  }

  let mut brightness: i64 = 0;
  for x in 0..1000 {
    for y in 0..1000 {
      brightness += grid[x][y];
    }
  }
  return brightness;
}

pub fn puzzle() {
  let result = part1();
  println!("Part1: {:?}", result);
  let result = part2();
  println!("Part2: {:?}", result);
}